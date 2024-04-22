use bevy::prelude::*;
use bevy_rapier2d::{geometry::Collider, plugin::RapierContext};

use super::{
    actor::{
        Damage, DamageKnockback, ActorChild, ActorContext, ActorSensors,
        DamageEvent, ChildType,
        effect_application::{
            ActorOnHitEffectContext, EffectContextWorldAccess, 
            apply_on_ability_hit_effects, ActorReference,
        }
    }, 
    Dead, Knockback, Position
};


#[derive(Component)]
pub struct Damageable
{
    pub invulnerability_remaining: f32,
    pub invulnerability_duration: f32,
}

fn do_collision_logic(
    commands: &mut Commands, 
    projectile_entity: Entity, 
    proj: &mut Damage, 
    proj_pos: Vec2,
    target_position: &Position,
    target_knckbk: &mut Knockback
) -> f32 {
    if proj.should_destroy()
    {
        commands.entity(projectile_entity).insert(Dead);
    }

    if let Some(knockback) = &proj.knockback
    {
        let impulse = match knockback
        {
            DamageKnockback::Impulse(i) => *i,
            DamageKnockback::Repulsion { center, strength } => (target_position.0 - *center).normalize_or_zero() * *strength,
            DamageKnockback::Attraction { center, strength } => (*center - target_position.0).normalize_or_zero() * *strength,
            DamageKnockback::RepulsionFromSelf { strength } => (target_position.0 - proj_pos).normalize_or_zero() * *strength,
        };
        
        *target_knckbk = Knockback::new(impulse, 0.5, [1.0, 1.0, 1.0]);
    }

    proj.damage
}


pub fn s_collision_projectiles_damage(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut undamageable_actors: Query<(Entity, &mut ActorContext, &mut Position), (Without<Damageable>, Without<ActorChild>)>,
    mut actor_query: Query<(Entity, &mut ActorContext, &mut Damageable, &mut Position, &ActorSensors, &mut Knockback, &Name), Without<Damage>>,
    mut actor_projectiles: Query<(Entity, &mut Damage, &GlobalTransform, &ActorChild, &Name), (Without<ActorContext>, With<Collider>)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let mut ability_hits: Vec<(Entity, ChildType, Vec2)> = Vec::new();
    // Do direct damage to non-actors (and trigger ability hits), do actor damage to other actors
    // info!("There are {} projectiles currently", actor_projectiles.iter().count());
    // info!("There are {} damageable actors currently", actor_query.iter().count());
    for (projectile_entity, mut proj, proj_trans, child, proj_name) in &mut actor_projectiles
    {
        if proj.deal_damage_once && proj.did_damage
        {
            continue;
        }
        for (target_entity, _, mut target_damageable, target_position, sensors, mut target_knckbk, actor_name) in &mut actor_query
        {
            if target_damageable.invulnerability_remaining > 0.0
            {
                continue;
            }
            if !sensors.sensors.iter().chain(std::iter::once(&target_entity)).any(|sensor_ent| rapier_context.intersection_pair(projectile_entity, *sensor_ent) == Some(true))
            {
                continue;
            }
            info!("Projectile entity '{}' collision with entity '{}', proj at {}, entity at: {}", proj_name, actor_name, proj_trans.translation(), target_position.0);

            let dmg_to_do = do_collision_logic(&mut commands, projectile_entity, &mut proj, proj_trans.translation().truncate(), &target_position, &mut target_knckbk);

            ability_hits.push((child.parent_actor, child.ability_type, proj_trans.translation().truncate()));
            damage_events.send(DamageEvent { instigator: child.parent_actor, victim: target_entity, damage: dmg_to_do });
            proj.did_damage = true;
            target_damageable.invulnerability_remaining = target_damageable.invulnerability_duration;

            if proj.deal_damage_once
            {
                break;
            }
        }
    }

    let mut damage_hits = Vec::new();
    for (actor_entity, ability_type, hit_location) in &ability_hits
    {
        let (parent_entity, mut context, mut position) = match actor_query.get_mut(*actor_entity)
        {
            Ok((ent, context, _, position, _, _, _)) => (ent, context, position),
            Err(_) => { 
                match undamageable_actors.get_mut(*actor_entity) {
                    Ok((ent, context, position)) => (ent, context, position),
                    Err(_) => { warn!("Could not find parent actor for ability hit!"); continue }
                }
            }
        };
        info!("Doing on ability hit for actor!");

        apply_on_ability_hit_effects(*ability_type, &mut ActorOnHitEffectContext{
            world_access: &mut EffectContextWorldAccess { commands: &mut commands, damage_instances: &mut damage_hits },
            instigator: &mut ActorReference { entity: parent_entity, context: &mut context, location: &mut position },
            victim: None, // TODO: eventually actually grab the context of hit victims
            hit_location: *hit_location
        })
    }

    damage_hits.iter().for_each(|x| { damage_events.send(*x); });
}

pub fn s_tick_damageable(
    mut damageable: Query<&mut Damageable>,
    time: Res<Time>,
) {
    for mut dmg in &mut damageable
    {
        dmg.invulnerability_remaining = (dmg.invulnerability_remaining - time.delta_seconds()).max(0.0);
    }
}
