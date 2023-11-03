use bevy::prelude::*;
use bevy_replicon::{prelude::*, renet::ServerEvent, client};

use super::plugin::*;


#[derive(Bundle)]
pub struct PlayerServerBundle
{
    player: Player,
    position: PlayerPosition,
    color: PlayerColor,
    replication: Replication
}

impl PlayerServerBundle
{
    pub fn new(id: u64, position: Vec2, color: Color) -> Self
    {
        Self 
        { 
            player: Player(id), 
            position: PlayerPosition(position), 
            color: PlayerColor(color), 
            replication: Replication
        }
    }
}



pub fn server_event_system(mut commands: Commands, mut server_event: EventReader<ServerEvent>)
{
    for event in &mut server_event
    {
        match event
        {
            ServerEvent::ClientConnected { client_id } => {
                info!("player: {client_id} Connected");

                let r = ((client_id % 25) as f32) / 25.0;
                let g = ((client_id % 19) as f32) / 19.0;
                let b = ((client_id % 29) as f32) / 29.0;
                commands.spawn(PlayerServerBundle::new(
                    *client_id,
                    Vec2::ZERO,
                    Color::rgb(r, g, b),
                ));
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("client {client_id} disconnected: {reason}");
            }
        }
    }
}


pub fn movement_system(
    time: Res<Time>,
    mut move_events: EventReader<FromClient<MoveDirection>>,
    mut players: Query<(&Player, &mut PlayerPosition)>,
) {
    for FromClient { client_id, event } in &mut move_events 
    {
        info!("Received event: {event:?} from client {client_id}");
        for (player, mut position) in &mut players
        {
            if *client_id == player.0 {
                let movement = event.0 * time.delta_seconds() * MOVE_SPEED;
                **position += movement;
            }
        }
    }
}

pub fn server_ability_response(
    mut ability_events: EventReader<FromClient<AbilityActivation>>,
    players: Query<(&Player, &PlayerPosition)>,
    mut commands: Commands,
    mut client_map: ResMut<ClientEntityMap>,
    tick: Res<RepliconTick>
) {
    for FromClient { client_id, event } in &mut ability_events
    {
        match event
        {
            AbilityActivation::None => { info!("Client '{client_id}' send empty ability event") },
            AbilityActivation::ShootBullet(client_bullet) => 
            {
                for (player, pos) in &players
                {
                    if *client_id != player.0
                    {
                        continue;
                    }

                    let server_bullet = commands.spawn(BulletBundle::new(pos.0, Vec2::new(10.0, 0.0), Vec2::new(5.0, 5.0))).id();

                    client_map.insert(*client_id, ClientMapping { tick: *tick, server_entity: server_bullet, client_entity: *client_bullet });
                }
            }
        }
    }
}

pub fn server_bullet_movement(
    mut bullets: Query<(&Bullet, &mut Transform)>,
    time: Res<Time>
) {
    for (bullet, mut trans) in &mut bullets
    {
        trans.translation += (bullet.velocity * time.delta_seconds()).extend(0.0);
    }
}
