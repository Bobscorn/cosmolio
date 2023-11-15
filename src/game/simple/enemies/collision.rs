use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::simple::{projectile::{Projectile, ProjectileDamage}, common::Health};

use super::Enemy;

pub fn s_collision_projectiles_enemy(
    mut commands: Commands,
    rapier_context: Res<RapierContext>,
    projectiles: Query<(Entity, &ProjectileDamage), (With<Projectile>, Without<Enemy>, With<Collider>)>,
    mut enemies: Query<(Entity, &mut Health), (With<Enemy>, Without<Projectile>, With<Collider>)>
) {
    for (projectile_entity, projectile_dmg) in &projectiles
    {
        for (enemy_entity, mut health) in &mut enemies
        {
            if rapier_context.intersection_pair(projectile_entity, enemy_entity) != Some(true)
            {
                continue;
            }

            info!("Projectile '{projectile_entity:?}' hit enemy '{enemy_entity:?}'");
            health.0 -= projectile_dmg.0;
            let Some(ent_coms) = commands.get_entity(projectile_entity) else { break };
            ent_coms.despawn_recursive();

            break;
        }
    }
}
