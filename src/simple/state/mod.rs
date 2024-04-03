use bevy::{ecs::event, prelude::*};
use serde::{Deserialize, Serialize};

use crate::simple::consts::CLIENT_STR;

pub mod setup;
pub mod class_select;
pub mod in_game;

/// Messages sent from server to clients about game state
#[derive(Event, PartialEq, Serialize, Deserialize)]
pub enum ServerStateEvent
{
    GoInGame,
    ReturnToLobby,
    PauseFighting,
    ResumeFighting,
    MoveToBreak,
    BreakToFighting,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum GameState
{
    #[default]
    Setup,
    ChoosingClass,
    InGame,
}

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum InGameState
{
    #[default]
    NotInGame,
    Fighting,
    Break,
    Paused,
}


pub fn c_receive_state_event(
    cur_state: Res<State<GameState>>,
    cur_ig_state: Res<State<InGameState>>,
    mut event_reader: EventReader<ServerStateEvent>,
    mut next_state: ResMut<NextState<GameState>>,
    mut next_in_game_state: ResMut<NextState<InGameState>>,
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
                next_in_game_state.set(InGameState::Fighting);
            },
            ServerStateEvent::ReturnToLobby => {
                if cur_state.get() == &GameState::ChoosingClass
                {
                    warn!("{CLIENT_STR} Received ReturnToLobby event when already in lobby!");
                    continue;
                }
                info!("{CLIENT_STR} Returning to lobby");
                next_state.set(GameState::ChoosingClass);
                if cur_ig_state.get() != &InGameState::NotInGame
                {
                    next_in_game_state.set(InGameState::NotInGame);
                }
            },
            ServerStateEvent::PauseFighting => {
                if cur_state.get() != &GameState::InGame
                {
                    warn!("{CLIENT_STR} Received Pause state event when not in game!");
                    continue;
                }
                if cur_ig_state.get() != &InGameState::Fighting
                {
                    warn!("{CLIENT_STR} Received Pause state event when not fighting!");
                    continue;
                }
                info!("{CLIENT_STR} Pausing the game...");
                next_in_game_state.set(InGameState::Paused);
            },
            ServerStateEvent::ResumeFighting => {
                if cur_state.get() != &GameState::InGame
                {
                    warn!("{CLIENT_STR} Received Resume state event when not in game!");
                    continue;
                }
                if cur_ig_state.get() != &InGameState::Paused
                {
                    warn!("{CLIENT_STR} Received Resume state event when not paused!");
                    continue;
                }
                info!("{CLIENT_STR} Resuming the game...");
                next_in_game_state.set(InGameState::Fighting);
            },
            ServerStateEvent::MoveToBreak => {
                if cur_state.get() != &GameState::InGame
                {
                    warn!("{CLIENT_STR} Received MoveToBreak state event when not in game!");
                    continue;
                }
                if vec![InGameState::Paused, InGameState::Fighting].iter().all(|st| st != cur_ig_state.get())
                {
                    warn!("{CLIENT_STR} Received MoveToBreak state not in either Paused or Fighting states!");
                    continue;
                }
                info!("{CLIENT_STR} Going to Break screen");
                next_in_game_state.set(InGameState::Break);
            },
            ServerStateEvent::BreakToFighting => {
                if cur_state.get() != &GameState::InGame
                {
                    warn!("{CLIENT_STR} Received Break -> Fighting state event when not in game!");
                    continue;
                }
                if cur_ig_state.get() != &InGameState::Break
                {
                    warn!("{CLIENT_STR} Received Break -> Fighting state event when not choosing upgrade!");
                    continue;
                }
                info!("{CLIENT_STR} Returning to fighting from break");
                next_in_game_state.set(InGameState::Fighting);
            }
        }
    }
}
