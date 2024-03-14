use bevy::prelude::*;

pub mod setup;
pub mod class_select;

/// Messages sent from server to clients about game state
#[derive(Event)]
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
