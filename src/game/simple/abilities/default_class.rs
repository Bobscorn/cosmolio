use bevy::prelude::*;

use bevy_replicon::{replicon_core::replicon_tick::RepliconTick, network_event::client_event::FromClient, server::{ClientMapping, ClientEntityMap, SERVER_ID}, renet::ClientId};
use serde::{Deserialize, Serialize};

use crate::game::simple::{common::Position, player::Player, abilities::bullet::BulletReplicationBundle, consts::BASE_BULLET_SPEED};

#[derive(Event, Serialize, Deserialize)]
pub enum DefaultClassAbility
{
    ShootAbility{ dir: Vec2, color: Color, prespawned: Entity },
}

pub fn server_default_class_ability_response(
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
                shoot_ability(&mut commands, &mut client_mapping, &players, client_id.raw(), *dir, *color, *prespawned, *tick);
            }
        }
    }
}

fn shoot_ability(
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
