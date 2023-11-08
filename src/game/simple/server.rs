use bevy::prelude::*;
use bevy_replicon::{prelude::*, renet::ServerEvent};

use super::{
    plugin::*,
    common::*, abilities::shoot::CanShootBullet
};

#[derive(Bundle)]
pub struct PlayerServerBundle
{
    player: Player,
    position: Position,
    color: PlayerColor,
    can_shoot: CanShootBullet,
    replication: Replication
}

impl PlayerServerBundle
{
    pub fn new(id: u64, position: Vec2, color: Color) -> Self
    {
        Self 
        { 
            player: Player(id), 
            position: Position(position), 
            color: PlayerColor(color), 
            can_shoot: CanShootBullet,
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
    mut players: Query<(&Player, &mut Position)>,
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
