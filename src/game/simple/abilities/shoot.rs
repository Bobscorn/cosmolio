use bevy::{prelude::*, window::PrimaryWindow};

use crate::game::simple::{util::get_screenspace_cursor_pos, player::{LocalPlayer, LocalPlayerId}, abilities::bullet::BulletReplicationBundle, consts::BASE_BULLET_SPEED};

use super::{bullet::CanShootBullet, default_class::DefaultClassAbility};
use crate::game::simple::common::*;


pub fn client_shoot_ability_systems(
    transform_query: Query<&Transform, (With<LocalPlayer>, With<CanShootBullet>)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ability_events: EventWriter<DefaultClassAbility>, 
    mut commands: Commands, 
    player: Res<LocalPlayerId>
) {
    let Ok(window) = window_q.get_single() else { return };
    let Ok((camera, camera_trans)) = camera_q.get_single() else { return };
    let Some(cursor_pos) = get_screenspace_cursor_pos(window, camera, camera_trans) else { return };

    let Ok(player_trans) = transform_query.get(player.entity) else { return };
    let player_pos = player_trans.translation.truncate();

    let bullet_dir = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::new(1.0, 0.0));

    let bullet_entity = commands.spawn((
        BulletReplicationBundle::new(player_pos, bullet_dir * BASE_BULLET_SPEED, 5.0), 
        DestroyIfNoMatchWithin{ remaining_time: 0.2 }
    )).id();
    info!("Client: Spawning Bullet Entity ({bullet_entity:?}) from Input");
    ability_events.send(DefaultClassAbility::ShootAbility{ dir: bullet_dir, prespawned: bullet_entity });
}

