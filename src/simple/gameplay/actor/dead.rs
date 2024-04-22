use bevy::prelude::*;

use crate::simple::{gameplay::{Dead, Position}, consts::SERVER_STR};

use super::{
    effect_application::{
        apply_on_ability_end_effects, 
        apply_on_death_effects, 
        apply_on_kill_effects,
        ActorReference,
        ActorEffectContext,
        ActorKillEffectContext,
        ActorDeathEffectContext,
        EffectContextWorldAccess,
    },
    ActorChild, 
    ActorContext, 
    DamageSource,
    DamageEvent,
};




pub fn s_destroy_dead_things(
    mut commands: Commands,
    mut parent_lookup: Query<(Entity, &mut ActorContext, &mut Position), Without<Dead>>,
    mut dead_parents: Query<(Entity, &mut ActorContext, &mut Position), With<Dead>>,
    dead_children: Query<(Entity, &ActorChild), With<Dead>>,
    dead_things: Query<Entity, (Without<ActorChild>, Without<ActorContext>, With<Dead>)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let mut damage_es = Vec::new();
    for (entity, child) in &dead_children
    {
        if let Ok((parent_ent, mut actor_context, mut position)) = parent_lookup.get_mut(child.parent_actor)
        {
            apply_on_ability_end_effects(child.ability_type, &mut ActorEffectContext{ 
                actor: &mut ActorReference { context: &mut actor_context, entity: parent_ent, location: &mut position },
                world_access: &mut EffectContextWorldAccess { commands: &mut commands, damage_instances: &mut damage_es },
            });
        }
        else if let Ok((parent_ent, mut actor_context, mut position)) = dead_parents.get_mut(entity) 
        {
            apply_on_ability_end_effects(child.ability_type, &mut ActorEffectContext{ 
                actor: &mut ActorReference { context: &mut actor_context, entity: parent_ent, location: &mut position },
                world_access: &mut EffectContextWorldAccess { commands: &mut commands, damage_instances: &mut damage_es },
            });
        }
        else
        {
            error!("Did not find parent for dead entity!");
        }
        commands.entity(entity).despawn_recursive();
    }
    let mut dying_actors = Vec::new();
    for (entity, actor, _) in &mut dead_parents
    {
        dying_actors.push((entity, actor.last_damage_source));
        commands.entity(entity).despawn_recursive();
    }
    for entity in &dead_things
    {
        commands.entity(entity).despawn_recursive();
    }

    for dying_actor in dying_actors
    {
        if let Some(source) = dying_actor.1
        {
            if let DamageSource::Actor(killer) = source
            {
                info!("{SERVER_STR} Trying to apply on kill effects for {killer:?} and on death effects for {:?}", dying_actor.0);
                let ((victim_e, mut victim_a, mut victim_p), (killer_e, mut killer_a, mut killer_p)) = match parent_lookup.get_mut(killer)
                {
                    Ok(killer_stuff) => match dead_parents.get_mut(dying_actor.0) {
                        Ok(dying_stuff) => (dying_stuff, killer_stuff),
                        Err(_) => { error!("Could not find Killer and Victim stuff in queries!"); continue; }
                    },
                    Err(_) => match dead_parents.get_many_mut([dying_actor.0, killer])
                    {
                        Ok([dying_stuff, killer_stuff]) => (dying_stuff, killer_stuff),
                        Err(_) => { error!("Could not find Killer and Victim stuff in queries!"); continue; },
                    },
                };
                apply_on_kill_effects(&mut ActorKillEffectContext {
                    world_access: &mut EffectContextWorldAccess { commands: &mut commands, damage_instances: &mut damage_es },
                    instigator: &mut ActorReference { entity: killer_e, context: &mut killer_a, location: &mut killer_p },
                    victim: &mut ActorReference { entity: victim_e, context: &mut victim_a, location: &mut victim_p },
                });
                apply_on_death_effects(&mut ActorDeathEffectContext {
                    world_access: &mut EffectContextWorldAccess { commands: &mut commands, damage_instances: &mut damage_es },
                    instigator: Some(&mut ActorReference { entity: killer_e, context: &mut killer_a, location: &mut killer_p }),
                    victim: &mut ActorReference { entity: victim_e, context: &mut victim_a, location: &mut victim_p },
                });
                continue;
            }
        }
        
        info!("Trying to apply on death effects (with no killer) for entity {:?}", dying_actor.0);
        let Ok((victim_e, mut victim_a, mut victim_p)) = dead_parents.get_mut(dying_actor.0) else { warn!("Failed to get victim stuff!"); continue; };
        apply_on_death_effects(&mut ActorDeathEffectContext{ 
            world_access: &mut EffectContextWorldAccess { commands: &mut commands, damage_instances: &mut damage_es },
            instigator: None,
            victim: &mut ActorReference { entity: victim_e, context: &mut victim_a, location: &mut victim_p },
        });
    }

    for damage_e in damage_es
    {
        damage_events.send(damage_e);
    }
}




