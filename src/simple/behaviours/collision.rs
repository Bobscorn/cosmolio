use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierContext, geometry::{Collider, Sensor}};

use crate::simple::{
    behaviours::damage::{Damage, DamageKnockback}, 
    enemies::Enemy, 
    common::{Health, Velocity, Dead, Position}
};

use super::effect::{
    apply_on_ability_hit_effects, 
    AbilityType, 
    ActorChild, 
    ActorContext, 
    ActorOnHitEffectContext, 
    DamageEvent
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
    target_health: &mut Health,
    target_damageable: &mut Damageable,
    target_position: &Position,
    target_velocity: &mut Velocity
) -> f32 {
    info!("Projectile '{projectile_entity:?}' hit enemy '{target_entity:?}'");

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
        
        target_velocity.apply_impulse(impulse);
    }

    proj.damage
}


pub fn s_collision_projectiles_damage(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut actor_queries: ParamSet<(
        Query<(&mut ActorContext, &mut Position), Without<Damage>>,
        Query<(Entity, &mut Health, &mut Damageable, &Position, &mut Velocity), (Without<Damage>, With<Collider>, With<Sensor>, Without<ActorContext>)>,
        Query<(Entity, &mut Health, &mut Damageable, &Position, &mut Velocity), (Without<Damage>, With<Collider>, With<Sensor>, With<ActorContext>)>
    )>,
    mut non_actor_projectiles: Query<(Entity, &mut Damage), (Without<Enemy>, With<Collider>, Without<Sensor>, Without<ActorChild>)>,
    mut actor_projectiles: Query<(Entity, &mut Damage, &Position, &ActorChild), (Without<Enemy>, With<Collider>, Without<Sensor>)>,
    mut damage_events: EventWriter<DamageEvent>,
) {
    let mut ability_hits: Vec<(Entity, AbilityType, Vec2)> = Vec::new();
    // Do damage directly, the old way
    for (projectile_entity, mut proj) in &mut non_actor_projectiles
    {
        if proj.deal_damage_once && proj.did_damage
        {
            continue;
        }
        for (target_entity, mut target_health, mut target_damageable, target_position, mut target_velocity) in &mut actor_queries.p1()
        {
            if target_damageable.invulnerability_remaining > 0.0
            {
                continue;
            }
            if rapier_context.intersection_pair(projectile_entity, target_entity) != Some(true)
            {
                continue;
            }

            let dmg_to_do = do_collision_logic(&mut commands, projectile_entity, &mut proj, target_entity, &mut target_health, &mut target_damageable, &target_position, &mut target_velocity);

            target_health.0 -= dmg_to_do;
            proj.did_damage = true;
            target_damageable.invulnerability_remaining = target_damageable.invulnerability_duration;

            break;
        }
        for (target_entity, mut target_health, mut target_damageable, target_position, mut target_velocity) in &mut actor_queries.p2()
        {
            if target_damageable.invulnerability_remaining > 0.0
            {
                continue;
            }
            if rapier_context.intersection_pair(projectile_entity, target_entity) != Some(true)
            {
                continue;
            }

            let dmg_to_do = do_collision_logic(&mut commands, projectile_entity, &mut proj, target_entity, &mut target_health, &mut target_damageable, &target_position, &mut target_velocity);

            target_health.0 -= dmg_to_do;
            proj.did_damage = true;
            target_damageable.invulnerability_remaining = target_damageable.invulnerability_duration;
            warn!("Non-actor collision is damaging an actor!");

            break;
        }
    }
    // Do direct damage to non-actors (and trigger ability hits), do actor damage to other actors
    for (projectile_entity, mut proj, position, child) in &mut actor_projectiles
    {
        if proj.deal_damage_once && proj.did_damage
        {
            continue;
        }
        for (target_entity, mut target_health, mut target_damageable, target_position, mut target_velocity) in &mut actor_queries.p1()
        {
            if target_damageable.invulnerability_remaining > 0.0
            {
                continue;
            }
            if rapier_context.intersection_pair(projectile_entity, target_entity) != Some(true)
            {
                continue;
            }

            let dmg = do_collision_logic(&mut commands, projectile_entity, &mut proj, target_entity, &mut target_health, &mut target_damageable, &target_position, &mut target_velocity);

            ability_hits.push((child.parent_actor, child.ability_type, position.0));
            target_health.0 -= dmg;
            proj.did_damage = true;
            target_damageable.invulnerability_remaining = target_damageable.invulnerability_duration;

            break;
        }
        for (target_entity, mut target_health, mut target_damageable, target_position, mut target_velocity) in &mut actor_queries.p2()
        {
            if target_damageable.invulnerability_remaining > 0.0
            {
                continue;
            }
            if rapier_context.intersection_pair(projectile_entity, target_entity) != Some(true)
            {
                continue;
            }

            let dmg_to_do = do_collision_logic(&mut commands, projectile_entity, &mut proj, target_entity, &mut target_health, &mut target_damageable, &target_position, &mut target_velocity);

            ability_hits.push((child.parent_actor, child.ability_type, position.0));
            damage_events.send(DamageEvent { instigator: child.parent_actor, victim: target_entity, damage: dmg_to_do });
            proj.did_damage = true;

            break;
        }
    }


    for (actor_entity, ability_type, hit_location) in &ability_hits
    {
        let mut actor_q = actor_queries.p0();
        let Ok((mut context, mut position)) = actor_q.get_mut(*actor_entity) else { continue };

        apply_on_ability_hit_effects(*ability_type, &mut ActorOnHitEffectContext{
            commands: &mut commands,
            instigator_context: &mut context,
            instigator_location: &mut position,
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
