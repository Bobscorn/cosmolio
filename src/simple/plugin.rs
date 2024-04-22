use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierConfiguration, prelude::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_replicon::prelude::*;
use bevy_replicon_renet::renet::{RenetClient, RenetServer, SendType};

use super::{
    data::{DataPlugin, DataServerPlugin}, gameplay::GameplayPlugin, player::PlayerPlugin, setup::SetupPlugin, state::ExecutionPlugin, ui::UIPlugin
};

pub const MOVE_SPEED: f32 = 300.0;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct SetupSystems;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ServerSystems;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientSystems;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AuthoritySystems;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HostAndClientSystems;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct InputSystems;

pub struct SimpleGame;

impl Plugin for SimpleGame
{
    fn build(&self, app: &mut App) {
        app.add_plugins((
            ExecutionPlugin,
            SetupPlugin,
            GameplayPlugin,
            UIPlugin,
            PlayerPlugin,
            DataServerPlugin,
            DataPlugin,
        ));
        app.insert_resource(RapierConfiguration { gravity: Vec2::ZERO, ..default() })
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0), 
                RapierDebugRenderPlugin::default()
            ));
            // .init_state::<GameState>()
            // .init_state::<InGameState>()
            // .init_asset::<ClassBaseData>()
            // .init_asset_loader::<ClassDataLoader>()
            // .init_asset::<WaveData>()
            // .init_asset_loader::<WaveDataLoader>()
            // .init_asset::<bounds::Bounds>()
            // .init_asset_loader::<assets::RonAssetLoader<bounds::Bounds>>()
            // .init_asset::<upgrade::static_upgrades::UpgradeCollection>()
            // .init_asset_loader::<assets::RonAssetLoader<upgrade::static_upgrades::UpgradeCollection>>()
            // .configure_sets(Update, 
            //     SetupSystems.run_if(in_state(GameState::Setup))
            // )
            // .configure_sets(FixedUpdate, (
            //     // InputSystems.run_if((has_authority().or_else(resource_exists::<RenetClient>())).and_then(in_state(GameState::InGame))),
            //     // HostAndClientSystems.run_if(has_authority().or_else(resource_exists::<RenetClient>()).and_then(in_state(GameState::InGame))),
            //     // ClientSystems.run_if(resource_exists::<RenetClient>().and_then(in_state(GameState::InGame))),
            //     // AuthoritySystems.run_if(has_authority().and_then(in_state(GameState::InGame))),
            //     // ServerSystems.run_if(resource_exists::<RenetServer>().and_then(in_state(GameState::InGame))),
            //     InputSystems.run_if(has_authority.or_else(resource_exists::<RenetClient>)),
            //     HostAndClientSystems.run_if(has_authority.or_else(resource_exists::<RenetClient>)),
            //     ClientSystems.run_if(resource_exists::<RenetClient>),
            //     AuthoritySystems.run_if(has_authority),
            //     ServerSystems.run_if(resource_exists::<RenetServer>),
            // ).chain())
            // .insert_resource(WaveOverseer::new())
            // .insert_resource(CurrentWave { wave: 0 })
            // .add_event::<DamageEvent>()
            // .replicate::<ActorClass>()
            // .replicate::<ActorContext>()
            // .replicate::<Bullet>()
            // .replicate::<Missile>()
            // .replicate::<CanShootBullet>()
            // .replicate::<CanUseAbilities>()
            // .replicate::<Damage>()
            // .replicate::<Enemy>()
            // .replicate::<Explosion>()
            // .replicate::<Knockback>()
            // .replicate::<MeleeAttack>()
            // .replicate::<Orientation>()
            // .replicate::<Position>()
            // .replicate::<PlayerColor>()
            // .replicate::<Player>()
            // .replicate::<RangedClassData>()
            // .replicate::<Velocity>()
            // .add_client_event::<MoveDirection>(ChannelKind::Ordered)
            // .add_client_event::<DefaultClassAbility>(ChannelKind::Ordered)
            // .add_client_event::<MeleeClassEvent>(ChannelKind::Ordered)
            // .add_client_event::<GeneralClientEvents>(ChannelKind::Ordered)
            // .add_client_event::<RangedClassEvent>(ChannelKind::Ordered)
            // .add_client_event::<ChosenUpgrade>(ChannelKind::Unordered)
            // .add_server_event::<GeneratedAvailableUpgrades>(ChannelKind::Unordered)
            // .add_server_event::<ServerStateEvent>(ChannelKind::Unordered)
            // .add_server_event::<NewWave>(ChannelKind::Unordered)
            // .add_systems(
                // Startup,
                // (
                    // cli_system.map(Result::unwrap),
                    // init_system,
                    // setup_assets,
                    // cs_setup_fonts,
                // ).chain()
            // )
            // .add_systems(Update, // <- could move into a load/asset plugin or smth
                // (
                    // wait_for_assets.run_if(in_state(GameState::Setup)),
                // )
            // )
            // .add_systems(OnEnter(GameState::ChoosingClass), setup_class_select_ui)
            // .add_systems(Update, // <- good candidate to move into a class select plugin or UI plugin
            //     (
            //         handle_class_select_ui,
            //     ).run_if(in_state(GameState::ChoosingClass))
            // )
            // .add_systems(OnExit(GameState::ChoosingClass), teardown_class_select_ui)
            // .add_systems(FixedUpdate, 
            //     (
            //         s_conn_events,
            //         s_general_client_events,
            //     ).chain().in_set(ServerSystems))
            // .add_systems(FixedUpdate, 
                // (
                    // s_movement_events, 
                    // s_tick_wave_overseer,
                    // s_collision_projectiles_damage,
                    // s_kill_zero_healths,
                    // s_bullet_authority,
                    // s_update_and_destroy_lifetimes,
                    // s_default_class_ability_response,
                    // s_melee_class_ability_response,
                    // s_ranged_class_response,
                    // s_melee_authority,
                    // s_missile_authority,
                    // s_move_missiles,
                    // s_laser_authority,
                    // s_explosion_authority,
                    // s_knockback,
                    // s_destroy_dead_things,
                    // s_tick_damageable,
                    // s_do_damage_events,
                    // s_receive_chosen_upgrades,
                // ).chain().in_set(AuthoritySystems).run_if(in_gaming_state.clone())
            // )
            // .add_systems(FixedUpdate, 
                // (
                    // s_setup_initial_class,
                    // s_rapier_update_position,
                    // s_rapier_velocity_update_pos,
                    // s_tick_next_wave,
                // ).chain().in_set(AuthoritySystems).run_if(in_gaming_state.clone())
            // )
            // .add_systems(FixedUpdate, 
            // (
            //     // s_generate_and_emit_available_upgrades,
            // ).in_set(AuthoritySystems))
            // .add_systems(FixedUpdate, 
                // s_handle_go_in_game_ui)
            // .add_systems(FixedUpdate, 
            //     (
                    // c_movement_predict,
                    // c_destroy_entites_without_match,
                    // c_receive_state_event,
                    // c_receive_next_wave,
            //     ).chain().in_set(ClientSystems)
            // )
            // .add_systems(
            //     FixedUpdate,
            //     (
                    // c_class_input_system,
                    // c_movement_input
            //     ).chain().in_set(InputSystems).run_if(in_gaming_state.clone())
            // )
            // .add_systems(
            //     FixedUpdate,
            //     (
                    // cs_velocity_movement,
                    // cs_velocity_damped_movement,
                    // cs_update_trans_system,
                    // cs_update_orientation_system,
                    // cs_move_enemies,
                    // c_enemies_extras,
                    // c_predict_knockback,
                    // c_bullet_extras,
                    // c_melee_extras,
                    // c_laser_extras,
                    // c_explosion_extras,
                    // c_missile_extras,
                    // c_update_info_text,
                    // c_class_change,
                    // c_add_healthbars,
                    // c_update_healthbars,
                    // bounds::cs_restrict_players_to_bounds,
            //     ).chain().in_set(HostAndClientSystems).run_if(in_gaming_state.clone())
            // )
            // .add_systems(
            //     FixedUpdate,
            //     (
            //         c_create_upgrade_ui,
            //         c_handle_upgrade_clicked,
            //     ).in_set(HostAndClientSystems), // Run this even if not in fighting state
            // )
            // .add_systems(PreUpdate, c_player_spawns.after(ClientSet::Receive));

        // let is_client_or_host = has_authority.or_else(resource_exists::<RenetClient>);
        // app
        //     // .add_systems(OnEnter(GameState::InGame), (in_game::begin_fighting, in_game::setup_uis.run_if(is_client_or_host.clone())))
        //     // .add_systems(OnExit(GameState::InGame), in_game::cleanup_uis.run_if(is_client_or_host.clone()))
        //     // .add_systems(OnEnter(InGameState::Paused), in_game::on_pause.run_if(is_client_or_host.clone()))
        //     // .add_systems(OnExit(InGameState::Paused), in_game::on_resume.run_if(is_client_or_host.clone()))
        //     // .add_systems(OnEnter(InGameState::Break), (in_game::on_enter_upgrade_select.run_if(is_client_or_host.clone()), s_generate_and_emit_available_upgrades.run_if(has_authority)))
        //     // .add_systems(OnTransition { from: InGameState::Break, to: InGameState::Fighting }, in_game::on_upgrade_select_to_fighting.run_if(is_client_or_host))
        //     .add_systems(FixedUpdate, (in_game::handle_resume_button, in_game::s_handle_next_wave_button).run_if(has_authority));
    }
}
