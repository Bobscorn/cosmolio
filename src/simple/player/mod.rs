mod event;
mod input;
mod movement;
mod spawning;

use std::thread::spawn;

pub use spawning::{LocalPlayer, Player, PlayerServerBundle};
pub use event::GeneralClientEvents;

use bevy::prelude::*;
use bevy_replicon::prelude::*;

#[derive(Resource, Reflect)]
pub struct LocalPlayerId
{
    pub is_host: bool,
    pub id: u64,
    pub entity: Entity
}

impl LocalPlayerId
{
    pub fn should_predict(&self) -> bool
    {
        !self.is_host
    }
}

use crate::simple::state;

pub struct PlayerPlugin;


impl Plugin for PlayerPlugin
{
    fn build(&self, app: &mut App) {
        app
            .replicate::<Player>()
            .replicate::<spawning::PlayerColor>()
            .add_client_event::<event::GeneralClientEvents>(ChannelKind::Ordered)
            .add_client_event::<input::MoveDirection>(ChannelKind::Ordered)
            .add_systems(FixedUpdate, (
                spawning::s_conn_events, 
                event::s_general_client_events,
                movement::s_movement_events,
            ).in_set(state::AuthoritySystems))
            .add_systems(FixedUpdate, (
                movement::c_movement_predict,
                input::c_movement_input,
                event::c_class_change,
            ).in_set(state::HostAndClientSystems))
            .add_systems(PreUpdate, spawning::c_player_spawns.after(ClientSet::Receive))
            ;
    }
}
