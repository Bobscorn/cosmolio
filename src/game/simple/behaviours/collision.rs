use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierContext, geometry::{Collider, Sensor}};

use crate::game::simple::{behaviours::projectile::{ProjectileDamage, Projectile, ProjectileKnockbackType}, enemies::Enemy, common::{Health, Velocity, Dead, Position}};


#[derive(Component)]
pub struct Damageable
{
    pub invulnerability_remaining: f32,
    pub invulnerability_duration: f32,
}


pub fn s_collision_projectiles_damage(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    mut projectiles: Query<(Entity, &mut ProjectileDamage, &Projectile), (Without<Enemy>, With<Collider>, Without<Sensor>)>,
    mut damageable: Query<(Entity, &mut Health, &mut Damageable, &Position, &mut Velocity), (Without<Projectile>, With<Collider>, With<Sensor>)>
) {
    for (projectile_entity, mut projectile_health, proj) in &mut projectiles
    {
        for (entity, mut health, mut damageable, position, mut velocity) in &mut damageable
        {
            if damageable.invulnerability_remaining > 0.0
            {
                continue;
            }
            if rapier_context.intersection_pair(projectile_entity, entity) != Some(true)
            {
                continue;
            }

            info!("Projectile '{projectile_entity:?}' hit enemy '{entity:?}'");
            health.0 -= projectile_health.damage;
            projectile_health.did_damage = true;
            damageable.invulnerability_remaining = damageable.invulnerability_duration;

            if projectile_health.should_destroy()
            {
                let Some(mut ent_coms) = commands.get_entity(projectile_entity) else { break };
                ent_coms.insert(Dead);
            }

            if let Some(knockback) = &proj.knockback
            {
                let impulse = match knockback
                {
                    ProjectileKnockbackType::Impulse(i) => *i,
                    ProjectileKnockbackType::Repulsion { center, strength } => (position.0 - *center).normalize_or_zero() * *strength,
                    ProjectileKnockbackType::Attraction { center, strength } => (*center - position.0).normalize_or_zero() * *strength,
                };
                
                velocity.apply_impulse(impulse);
            }

            break;
        }
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
