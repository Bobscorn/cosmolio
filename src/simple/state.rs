use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet::renet::{RenetClient, RenetServer};
use serde::{Deserialize, Serialize};
use super::consts::CLIENT_STR;

/// Systems to be run on a dedicated server (make sure AuthoritySystems is not more appropriate)
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ServerSystems;

/// Systems to be run on clients (but not the host, try the HostAndClientSystems first)
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ClientSystems;

/// Systems to be run on the server, or the hosting client
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct AuthoritySystems;

/// Systems to be run on clients, OR the host
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct HostAndClientSystems;

/// Systems to be run while in setup (of assets and such)
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct SetupSystems;

/// Systems to be run while choosing class
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct ChoosingClassSystems;

/// Systems to be run while in game
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct InGameSystems;

/// Systems to be run while in game, and fighting, on the server
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct FightingSystems;

/// Systems to be run while in game, but paused
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct PausedSystems;

/// Systems to be run while in game, but during a break
#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
pub struct BreakSystems;

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

/// Receives server state events
fn c_receive_state_event(
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

/// Run when entering the GameState::InGame
/// Sets the InGameState to Fighting
fn begin_fighting(
    mut ign_state: ResMut<NextState<InGameState>>,
) {
    ign_state.set(InGameState::Fighting);
}


/// Run when leaving GameState::InGame
/// This system will just set InGameState to InGameState::NotInGame
fn leave_in_game_state(
    mut next_state: ResMut<NextState<InGameState>>
) {
    next_state.set(InGameState::NotInGame);
}



/// Configures the system sets defined here
pub struct ExecutionPlugin;

impl Plugin for ExecutionPlugin
{
    fn build(&self, app: &mut App)
    {
        let is_server = resource_exists::<RenetServer>;
        let is_client = resource_exists::<RenetClient>;

        app
            // States v
            .init_state::<GameState>()
            .init_state::<InGameState>()
            // Server sets v
            .configure_sets(FixedUpdate, (
                HostAndClientSystems.run_if(has_authority.or_else(is_client)),
                AuthoritySystems.run_if(has_authority),
                ServerSystems.run_if(is_server),
                ClientSystems.run_if(is_client),
            ))
            .configure_sets(Update, (
                HostAndClientSystems.run_if(has_authority.or_else(is_client)),
                AuthoritySystems.run_if(has_authority),
                ServerSystems.run_if(is_server),
                ClientSystems.run_if(is_client),
            ))
            // State sets v
            .configure_sets(FixedUpdate, (
                SetupSystems.run_if(in_state(GameState::Setup)),
                ChoosingClassSystems.run_if(in_state(GameState::ChoosingClass)),
                InGameSystems.run_if(in_state(GameState::InGame)),
                FightingSystems.run_if(in_state(InGameState::Fighting)),
                PausedSystems.run_if(in_state(InGameState::Paused)),
                BreakSystems.run_if(in_state(InGameState::Break)),
            ))
            .configure_sets(Update, (
                SetupSystems.run_if(in_state(GameState::Setup)),
                ChoosingClassSystems.run_if(in_state(GameState::ChoosingClass)),
                InGameSystems.run_if(in_state(GameState::InGame)),
                FightingSystems.run_if(in_state(InGameState::Fighting)),
                PausedSystems.run_if(in_state(InGameState::Paused)),
                BreakSystems.run_if(in_state(InGameState::Break)),
            ))
            // Setup system v
            .add_systems(OnEnter(GameState::InGame), begin_fighting)
            .add_systems(OnExit(GameState::InGame), leave_in_game_state)
            .add_systems(FixedUpdate, c_receive_state_event.in_set(ClientSystems))
            // Events v
            .add_server_event::<ServerStateEvent>(ChannelKind::Ordered)
            ;
    }
}
