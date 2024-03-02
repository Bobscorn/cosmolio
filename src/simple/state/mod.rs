use bevy::prelude::*;

pub mod setup;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState
{
    #[default]
    Setup,
    InGame,
}
