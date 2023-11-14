use bevy::{prelude::*, window::PrimaryWindow};
use bevy_replicon::prelude::*;

use crate::game::simple::{util::get_screenspace_cursor_pos, player::{Player, LocalPlayer, LocalPlayerId}, abilities::bullet::BulletReplicationBundle, consts::BASE_BULLET_SPEED};

use super::{AbilityActivation, bullet::CanShootBullet};
use crate::game::simple::common::*;


/// Client only input system
pub fn ability_input_system(
    transform_query: Query<&Transform, (With<LocalPlayer>, With<CanShootBullet>)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ability_events: EventWriter<AbilityActivation>, 
    mut commands: Commands, 
    input: Res<Input<KeyCode>>,
    player: Res<LocalPlayerId>
) {
    if !input.just_pressed(KeyCode::Space)
    {
        return;
    }
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
    ability_events.send(AbilityActivation::ShootBullet(bullet_dir, bullet_entity));
}

pub fn server_ability_response(
    mut ability_events: EventReader<FromClient<AbilityActivation>>,
    players: Query<(&Player, &Position)>,
    mut commands: Commands,
    mut client_map: ResMut<ClientEntityMap>,
    tick: Res<RepliconTick>
) {
    for FromClient { client_id, event } in &mut ability_events
    {
        if *client_id == SERVER_ID
        {
            // Skip events from the "server's client" 
            // This assumes the client predicts what the server will do, meaning if the server did it, it would happen twice for the local player
            continue;
        }
        match event
        {
            AbilityActivation::None => { info!("Client '{client_id}' send empty ability event") },
            AbilityActivation::ShootBullet(dir, client_bullet) => 
            {
                for (player, pos) in &players
                {
                    if *client_id != player.0
                    {
                        continue;
                    }

                    let server_bullet = commands.spawn(BulletReplicationBundle::new(pos.0, *dir * BASE_BULLET_SPEED, 5.0)).id();

                    info!("Server: Spawning ({server_bullet:?}) for client '{client_id}'s {client_bullet:?}");
                    client_map.insert(*client_id, ClientMapping { tick: *tick, server_entity: server_bullet, client_entity: *client_bullet });
                }
            }
        }
    }
}
