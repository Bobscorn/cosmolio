use bevy::{prelude::*, window::PrimaryWindow};

use bevy_replicon::{replicon_core::replicon_tick::RepliconTick, network_event::client_event::FromClient, server::{ClientMapping, ClientEntityMap, SERVER_ID}, renet::ClientId};
use serde::{Deserialize, Serialize};

use crate::game::simple::{common::{Position, DestroyIfNoMatchWithin}, player::{Player, LocalPlayer, LocalPlayerId}, abilities::bullet::BulletReplicationBundle, consts::BASE_BULLET_SPEED, util::get_screenspace_cursor_pos};

use super::bullet::CanShootBullet;

#[derive(Event, Serialize, Deserialize)]
pub enum DefaultClassAbility
{
    ShootAbility{ dir: Vec2, color: Color, prespawned: Entity },
}

pub fn s_default_class_ability_response(
    mut commands: Commands,
    mut client_events: EventReader<FromClient<DefaultClassAbility>>,
    mut client_mapping: ResMut<ClientEntityMap>,
    players: Query<(&Player, &Position)>,
    tick: Res<RepliconTick>,
) {
    for FromClient { client_id, event } in client_events.read()
    {
        if client_id.raw() == SERVER_ID.raw()
        {
            continue;
        }

        match event
        {
            DefaultClassAbility::ShootAbility { dir, color, prespawned } =>
            {
                s_shoot_ability(&mut commands, &mut client_mapping, &players, client_id.raw(), *dir, *color, *prespawned, *tick);
            }
        }
    }
}

fn s_shoot_ability(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position)>,
    client_id: u64,
    dir: Vec2,
    color: Color,
    prespawned: Entity,
    tick: RepliconTick,
) {
    for (player, pos) in players
    {
        if client_id != player.0
        {
            continue;
        }

        let server_bullet = commands.spawn(BulletReplicationBundle::new(pos.0, color, dir * BASE_BULLET_SPEED, 5.0)).id();

        info!("Server: Spawning ({server_bullet:?}) for client '{client_id}'s {prespawned:?}");
        client_map.insert(ClientId::from_raw(client_id), ClientMapping { tick: tick, server_entity: server_bullet, client_entity: prespawned });
    }
}


pub trait GetColor
{
    fn get_color() -> Color;
}

pub fn c_shoot_ability<T: GetColor>(
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
        BulletReplicationBundle::new(player_pos, T::get_color(), bullet_dir * BASE_BULLET_SPEED, 5.0), 
        DestroyIfNoMatchWithin{ remaining_time: 0.2 }
    )).id();
    info!("Client: Spawning Bullet Entity ({bullet_entity:?}) from Input");
    ability_events.send(DefaultClassAbility::ShootAbility{ dir: bullet_dir, color: T::get_color(), prespawned: bullet_entity });
}
