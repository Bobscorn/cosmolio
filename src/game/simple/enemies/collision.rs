use bevy::prelude::*;
use bevy_rapier2d::prelude::*;

use crate::game::simple::{behaviours::projectile::{Projectile, ProjectileDamage}, common::{Velocity, Health, Dead}};

use super::Enemy;

// pub fn s_collision_projectiles_enemy(
//     mut commands: Commands,
//     rapier_context: Res<RapierContext>,
//     mut projectiles: Query<(Entity, &mut ProjectileDamage, &Projectile), (Without<Enemy>, With<Collider>)>,
//     mut enemies: Query<(Entity, &mut Health, &mut Velocity), (With<Enemy>, Without<Projectile>, With<Collider>)>
// ) {
//     for (projectile_entity, mut projectile_dmg, proj) in &mut projectiles
//     {
//         if projectile_dmg.did_damage
//         {
//             continue;
//         }
//         for (enemy_entity, mut health, mut velocity) in &mut enemies
//         {
//             if rapier_context.intersection_pair(projectile_entity, enemy_entity) != Some(true)
//             {
//                 continue;
//             }

//             info!("Projectile '{projectile_entity:?}' hit enemy '{enemy_entity:?}'");
//             health.0 -= projectile_dmg.damage;
//             projectile_dmg.did_damage = true;

//             if projectile_dmg.should_destroy()
//             {
//                 let Some(mut ent_coms) = commands.get_entity(projectile_entity) else { break };
//                 ent_coms.insert(Dead);
//             }

//             velocity.apply_impulse(proj.knockback);

//             break;
//         }
//     }
// }
