mod bounds;
mod collision;
mod common;
mod enemies;
mod upgrade;

pub mod actor;
pub mod classes;
pub mod objects;

pub use common::{Position, Velocity, Orientation, VelocityDamping, Dead, Lifetime, DestroyIfNoMatchWithin, Knockback};
pub use collision::Damageable;
pub use enemies::{Enemy, EnemySpawnType, WaveOverseer};
pub use upgrade::{AvailablePlayerUpgrades, ChosenUpgrade, GeneratedAvailableUpgrades, Upgrade, UpgradeBehaviour};


use bevy::prelude::*;
use bevy_replicon::prelude::*;
use crate::simple::state;

pub struct GameplayPlugin;

impl Plugin for GameplayPlugin
{
    fn build(&self, app: &mut App) {
        app
            // Enemy v
            .insert_resource(enemies::WaveOverseer::new())
            .insert_resource(enemies::CurrentWave { wave: 0 })
            .replicate::<enemies::Enemy>()
            .add_server_event::<enemies::NewWave>(ChannelKind::Unordered)
            //   Authority + Fighting systems v
            .add_systems(FixedUpdate, (
                enemies::spawning::s_tick_wave_overseer,
                enemies::spawning::s_tick_next_wave,
            ).in_set(state::AuthoritySystems).in_set(state::FightingSystems))
            //   Client only systems v
            .add_systems(FixedUpdate, (
                enemies::spawning::c_receive_next_wave,
            ).in_set(state::ClientSystems))
            //   Client & Server + Fighting systems v
            .add_systems(FixedUpdate, (
                enemies::moving::cs_move_enemies,
            ).in_set(state::FightingSystems))
            //   Client & Host systems v
            .add_systems(FixedUpdate, (
                enemies::spawning::c_enemies_extras,
            ).in_set(state::HostAndClientSystems))
            // Objects v
            .add_plugins(objects::GameplayObjectPlugin)
            // Actor v
            .add_plugins(actor::ActorPlugin)
            // Common v
            .replicate::<Knockback>()
            .replicate::<Orientation>()
            .replicate::<Position>()
            .replicate::<Velocity>()
            .add_systems(FixedUpdate, (
                common::s_kill_zero_healths,
                common::s_knockback,
                common::s_update_and_destroy_lifetimes,
                common::s_rapier_update_position,
                common::s_rapier_velocity_update_pos,
            ).in_set(state::AuthoritySystems).in_set(state::FightingSystems))
            .add_systems(FixedUpdate, (
                common::cs_velocity_movement,
                common::cs_velocity_damped_movement,
            ).in_set(state::FightingSystems))
            .add_systems(FixedUpdate, (
                common::cs_update_trans_system,
                common::cs_update_orientation_system,
            ).in_set(state::HostAndClientSystems))
            .add_systems(FixedUpdate, (
                common::c_destroy_entites_without_match,
                common::c_predict_knockback,
            ).in_set(state::ClientSystems))
            // Classes v
            .add_plugins(classes::ClassesPlugin)
            // Upgrade v
            .add_client_event::<upgrade::ChosenUpgrade>(ChannelKind::Unordered)
            .add_server_event::<upgrade::GeneratedAvailableUpgrades>(ChannelKind::Unordered)
            .add_systems(FixedUpdate, (
                upgrade::s_receive_chosen_upgrades,
            ).in_set(state::AuthoritySystems))
            .add_systems(OnEnter(state::InGameState::Break), upgrade::s_generate_and_emit_available_upgrades.run_if(has_authority))
            // Collision v
            .add_systems(FixedUpdate, (
                collision::s_collision_projectiles_damage,
                collision::s_tick_damageable,
            ).in_set(state::AuthoritySystems).in_set(state::FightingSystems))
            // Bounds v
            .add_systems(FixedUpdate, (
                bounds::cs_restrict_players_to_bounds,
            ).in_set(state::FightingSystems))
            ;
    }
}
