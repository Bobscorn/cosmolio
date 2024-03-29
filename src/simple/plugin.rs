use std::time::Duration;

use bevy::prelude::*;
use bevy_rapier2d::{plugin::RapierConfiguration, prelude::{NoUserData, RapierPhysicsPlugin}, render::RapierDebugRenderPlugin};
use bevy_replicon::{prelude::*, renet::SendType};

use super::{
    behaviours::{
        collision::{s_collision_projectiles_damage, s_tick_damageable}, 
        damage::{s_do_damage_events, Damage}, 
        dead::s_destroy_dead_things, 
        effect::{ActorContext, DamageEvent}, 
        explosion::{c_explosion_extras, s_explosion_authority, Explosion}, 
        laser::{c_laser_extras, s_laser_authority}, 
        missile::{c_missile_extras, s_missile_authority, s_move_missiles, Missile},
    }, classes::{
        bullet::{
            c_bullet_extras, s_bullet_authority, Bullet, CanShootBullet
        }, c_class_input_system, class::{s_setup_initial_class, ActorClass, ClassBaseData, ClassDataLoader}, default_class::{s_default_class_ability_response, DefaultClassAbility}, melee::{c_melee_extras, s_melee_authority, MeleeAttack}, melee_class::{s_melee_class_ability_response, MeleeClassEvent}, ranged_class::{s_ranged_class_response, RangedClassData, RangedClassEvent}, tags::CanUseAbilities
    }, client::*, common::*, enemies::{
        moving::cs_move_enemies, spawning::{c_enemies_extras, s_tick_wave_overseer}, Enemy, WaveOverseer
    }, player::*, server::*, state::{
        c_receive_state_event, class_select::{handle_class_select_ui, s_handle_go_in_game_ui, setup_class_select_ui, teardown_class_select_ui}, setup::{c_update_bullet_text, cli_system, init_system, setup_class_assets, wait_for_assets}, GameState, ServerStateEvent
    }, upgrade::{s_generate_and_emit_available_upgrades, s_receive_chosen_upgrades, ui::{c_create_upgrade_ui, c_handle_upgrade_clicked}, ChosenUpgrade, GeneratedAvailableUpgrades}, visuals::{healthbar::{c_add_healthbars, c_update_healthbars}, ui::cs_setup_fonts}
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
        app.insert_resource(RapierConfiguration { gravity: Vec2::ZERO, ..default() })
            .add_plugins((
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(1.0), 
                RapierDebugRenderPlugin::default()
            ))
            .add_state::<GameState>()
            .init_asset::<ClassBaseData>()
            .init_asset_loader::<ClassDataLoader>()
            .configure_sets(Update, 
                SetupSystems.run_if(in_state(GameState::Setup))
            )
            .configure_sets(FixedUpdate, (
                // InputSystems.run_if((has_authority().or_else(resource_exists::<RenetClient>())).and_then(in_state(GameState::InGame))),
                // HostAndClientSystems.run_if(has_authority().or_else(resource_exists::<RenetClient>()).and_then(in_state(GameState::InGame))),
                // ClientSystems.run_if(resource_exists::<RenetClient>().and_then(in_state(GameState::InGame))),
                // AuthoritySystems.run_if(has_authority().and_then(in_state(GameState::InGame))),
                // ServerSystems.run_if(resource_exists::<RenetServer>().and_then(in_state(GameState::InGame))),
                InputSystems.run_if((has_authority().or_else(resource_exists::<RenetClient>()))),
                HostAndClientSystems.run_if(has_authority().or_else(resource_exists::<RenetClient>())),
                ClientSystems.run_if(resource_exists::<RenetClient>()),
                AuthoritySystems.run_if(has_authority()),
                ServerSystems.run_if(resource_exists::<RenetServer>()),
            ).chain())
            .insert_resource(WaveOverseer::new(25.0))
            .add_event::<DamageEvent>()
            .replicate::<ActorClass>()
            .replicate::<ActorContext>()
            .replicate::<Bullet>()
            .replicate::<Missile>()
            .replicate::<CanShootBullet>()
            .replicate::<CanUseAbilities>()
            .replicate::<Damage>()
            .replicate::<Enemy>()
            .replicate::<Explosion>()
            .replicate::<Knockback>()
            .replicate::<MeleeAttack>()
            .replicate::<Orientation>()
            .replicate::<Position>()
            .replicate::<PlayerColor>()
            .replicate::<Player>()
            .replicate::<RangedClassData>()
            .replicate::<Velocity>()
            .add_client_event::<MoveDirection>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<DefaultClassAbility>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<MeleeClassEvent>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<GeneralClientEvents>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<RangedClassEvent>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<ChosenUpgrade>(SendType::ReliableUnordered { resend_time: Duration::from_millis(300) })
            .add_server_event::<GeneratedAvailableUpgrades>(SendType::ReliableUnordered { resend_time: Duration::from_millis(300) })
            .add_server_event::<ServerStateEvent>(SendType::ReliableUnordered { resend_time: Duration::from_millis(300) })
            .add_systems(
                Startup,
                (
                    cli_system.map(Result::unwrap),
                    init_system,
                    setup_class_assets,
                    cs_setup_fonts,
                ).chain()
            )
            .add_systems(Update, // <- could move into a load/asset plugin or smth
                (
                    wait_for_assets.run_if(in_state(GameState::Setup)),
                )
            )
            .add_systems(OnEnter(GameState::ChoosingClass), setup_class_select_ui)
            .add_systems(Update, // <- good candidate to move into a class select plugin or UI plugin
                (
                    handle_class_select_ui,
                ).run_if(in_state(GameState::ChoosingClass))
            )
            .add_systems(OnExit(GameState::ChoosingClass), teardown_class_select_ui)
            .add_systems(FixedUpdate, 
                (
                    s_conn_events,
                    s_general_client_events,
                ).chain().in_set(ServerSystems))
            .add_systems(FixedUpdate, 
                (
                    s_movement_events, 
                    s_tick_wave_overseer,
                    s_collision_projectiles_damage,
                    s_kill_zero_healths,
                    s_bullet_authority,
                    s_update_and_destroy_lifetimes,
                    s_default_class_ability_response,
                    s_melee_class_ability_response,
                    s_ranged_class_response,
                    s_melee_authority,
                    s_missile_authority,
                    s_move_missiles,
                    s_laser_authority,
                    s_explosion_authority,
                    s_knockback,
                    s_destroy_dead_things,
                    s_tick_damageable,
                    s_do_damage_events,
                    s_receive_chosen_upgrades,
                ).chain().in_set(AuthoritySystems).run_if(in_state(GameState::InGame))
            )
            .add_systems(FixedUpdate, 
                (
                    s_setup_initial_class,
                    s_rapier_update_position,
                    s_rapier_velocity_update_pos,
                ).chain().in_set(AuthoritySystems).run_if(in_state(GameState::InGame))
            )
            .add_systems(FixedUpdate, 
            (
                s_generate_and_emit_available_upgrades,
            ).in_set(AuthoritySystems))
            .add_systems(FixedUpdate, 
                s_handle_go_in_game_ui)
            .add_systems(FixedUpdate, 
                (
                    c_movement_predict,
                    c_destroy_entites_without_match,
                    c_receive_state_event,
                ).chain().in_set(ClientSystems)
            )
            .add_systems(
                FixedUpdate,
                (
                    c_class_input_system,
                    c_movement_input
                ).chain().in_set(InputSystems).run_if(in_state(GameState::InGame))
            )
            .add_systems(
                FixedUpdate,
                (
                    cs_velocity_movement,
                    cs_velocity_damped_movement,
                    cs_update_trans_system,
                    cs_update_orientation_system,
                    cs_move_enemies,
                    c_enemies_extras,
                    c_predict_knockback,
                    c_bullet_extras,
                    c_melee_extras,
                    c_laser_extras,
                    c_explosion_extras,
                    c_missile_extras,
                    c_update_bullet_text,
                    c_class_change,
                    c_add_healthbars,
                    c_update_healthbars,
                    c_handle_upgrade_clicked,
                    c_create_upgrade_ui,
                ).chain().in_set(HostAndClientSystems).run_if(in_state(GameState::InGame))
            )
            .add_systems(PreUpdate, c_player_spawns.after(ClientSet::Receive));
    }
}

fn cs_update_trans_system(mut players: Query<(&Position, &mut Transform), Without<bevy_rapier2d::prelude::RigidBody>>)
{
    for (player_pos, mut transform) in &mut players
    {
        transform.translation = player_pos.extend(0.0);
    }
}

fn cs_update_orientation_system(mut objects: Query<(&Orientation, &mut Transform)>)
{
    for (object_orientation, mut transform) in &mut objects
    {
        transform.rotation = Quat::from_rotation_z(object_orientation.0);
    }
}

pub fn cs_velocity_movement(
    mut objects: Query<(&Velocity, &mut Position), Without<VelocityDamping>>,
    time: Res<Time>
) {
    for (vel, mut pos) in &mut objects
    {
        pos.0 += vel.0 * time.delta_seconds();
    }
}

pub fn cs_velocity_damped_movement(
    mut objects: Query<(&mut Velocity, &VelocityDamping, &mut Position)>,
    time: Res<Time>,
) {
    for (mut vel, damp, mut pos) in &mut objects
    {
        pos.0 += vel.0 * time.delta_seconds();
        vel.0 *= 1.0 - damp.0 * time.delta_seconds();
    }
}

// Copy position modified by bevy_rapier across to Position value to be replicated
pub fn s_rapier_update_position(
    mut objects: Query<(&Transform, &mut Position), (With<bevy_rapier2d::prelude::Velocity>, With<bevy_rapier2d::prelude::RigidBody>)>, 
) {
    for (trans, mut pos) in &mut objects
    {
        pos.0 = trans.translation.truncate();
    }
}

pub fn s_rapier_velocity_update_pos(
    mut objects: Query<(&bevy_rapier2d::prelude::Velocity, &mut Position), Without<bevy_rapier2d::prelude::RigidBody>>,
    time: Res<Time>,
) {
    for (vel, mut pos) in &mut objects
    {
        pos.0 += vel.linvel * time.delta_seconds();
    }
}
