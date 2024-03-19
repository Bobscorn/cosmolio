use bevy::{ecs::event, prelude::*};
use serde::{Deserialize, Serialize};

use crate::simple::consts::CLIENT_STR;

pub mod setup;
pub mod class_select;

/// Messages sent from server to clients about game state
#[derive(Event, PartialEq, Serialize, Deserialize)]
pub enum ServerStateEvent
{
    GoInGame,
    ReturnToLobby,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState
{
    #[default]
    Setup,
    ChoosingClass,
    InGame,
}


pub fn c_receive_state_event(
    cur_state: Res<State<GameState>>,
    mut event_reader: EventReader<ServerStateEvent>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for state_event in event_reader.read()
    {
        match state_event
        {
            ServerStateEvent::GoInGame => {
                if cur_state.get() == &GameState::InGame
                {
                    warn!("{CLIENT_STR} Received GoInGame event when already in game!");
                    continue;
                }
                info!("{CLIENT_STR} Going in game");
                next_state.set(GameState::InGame);
            },
            ServerStateEvent::ReturnToLobby => {
                if cur_state.get() == &GameState::ChoosingClass
                {
                    warn!("{CLIENT_STR} Received ReturnToLobby event when already in lobby!");
                    continue;
                }
                info!("{CLIENT_STR} Returning to lobby");
                next_state.set(GameState::ChoosingClass);
            }
        }
    }
}
