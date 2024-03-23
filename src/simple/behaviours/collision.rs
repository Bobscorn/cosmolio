use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierContext, geometry::{Collider, Sensor}};

use crate::simple::{
    behaviours::damage::{Damage, DamageKnockback}, 
    enemies::Enemy, 
    common::{Velocity, Dead, Position}
};

use super::effect::{
    apply_on_ability_hit_effects, ActorChild, ActorContext, ActorOnHitEffectContext, ActorSensors, ChildType, DamageEvent
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
    target_entity: Entity,
    target_position: &Position,
    // target_velocity: &mut Velocity
) -> f32 {
    debug!("Projectile '{projectile_entity:?}' hit enemy '{target_entity:?}'");

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
        };
        
        // target_velocity.apply_impulse(impulse);
    }

    proj.damage
}


pub fn s_collision_projectiles_damage(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut undamageable_actors: Query<(Entity, &mut ActorContext, &mut Position), (Without<Damageable>, Without<ActorChild>)>,
    mut actor_query: Query<(Entity, &mut ActorContext, &mut Damageable, &mut Position, &ActorSensors), Without<Damage>>,
    mut actor_projectiles: Query<(Entity, &mut Damage, &Position, &ActorChild), (Without<ActorContext>, With<Collider>)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let mut ability_hits: Vec<(Entity, ChildType, Vec2)> = Vec::new();
    // Do direct damage to non-actors (and trigger ability hits), do actor damage to other actors
    for (projectile_entity, mut proj, position, child) in &mut actor_projectiles
    {
        if proj.deal_damage_once && proj.did_damage
        {
            continue;
        }
        for (target_entity, _, mut target_damageable, target_position, sensors) in &mut actor_query
        {
            if target_damageable.invulnerability_remaining > 0.0
            {
                continue;
            }
            if !sensors.sensors.iter().chain(std::iter::once(&target_entity)).any(|sensor_ent| rapier_context.intersection_pair(projectile_entity, *sensor_ent) == Some(true))
            {
                continue;
            }

            let dmg_to_do = do_collision_logic(&mut commands, projectile_entity, &mut proj, target_entity, &target_position);

            ability_hits.push((child.parent_actor, child.ability_type, position.0));
            damage_events.send(DamageEvent { instigator: child.parent_actor, victim: target_entity, damage: dmg_to_do });
            proj.did_damage = true;
            target_damageable.invulnerability_remaining = target_damageable.invulnerability_duration;

            break;
        }
    }


    for (actor_entity, ability_type, hit_location) in &ability_hits
    {
        let (parent_entity, mut context, mut position) = match actor_query.get_mut(*actor_entity)
        {
            Ok((ent, context, _, position, _)) => (ent, context, position),
            Err(_) => { 
                match undamageable_actors.get_mut(*actor_entity) {
                    Ok((ent, context, position)) => (ent, context, position),
                    Err(_) => { warn!("Could not find parent actor for ability hit!"); continue }
                }
            }
        };
        info!("Doing on ability hit for actor!");

        apply_on_ability_hit_effects(*ability_type, &mut ActorOnHitEffectContext{
            commands: &mut commands,
            instigator_entity: parent_entity,
            instigator_context: &mut context,
            instigator_location: &mut position,
            victim_entity: Entity::PLACEHOLDER,
            victim_context: None,
            victim_location: None,
            hit_location: *hit_location
        })
    }
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
