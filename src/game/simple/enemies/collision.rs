use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::simple::{projectile::{Projectile, ProjectileDamage}, common::{Velocity, Health}};

use super::Enemy;

pub fn s_collision_projectiles_enemy(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    projectiles: Query<(Entity, &ProjectileDamage, &Projectile), (Without<Enemy>, With<Collider>)>,
    mut enemies: Query<(Entity, &mut Health, &mut Velocity), (With<Enemy>, Without<Projectile>, With<Collider>)>
) {
    for (projectile_entity, projectile_dmg, proj) in &projectiles
    {
        for (enemy_entity, mut health, mut velocity) in &mut enemies
        {
            if rapier_context.intersection_pair(projectile_entity, enemy_entity) != Some(true)
            {
                continue;
            }

            info!("Projectile '{projectile_entity:?}' hit enemy '{enemy_entity:?}'");
            health.0 -= projectile_dmg.0;
            let Some(ent_coms) = commands.get_entity(projectile_entity) else { break };
            ent_coms.despawn_recursive();

            velocity.apply_impulse(proj.knockback);

            break;
        }
    }
}
