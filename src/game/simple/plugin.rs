use std::{net::{IpAddr, UdpSocket, SocketAddr, Ipv4Addr}, time::{SystemTime, Duration}, error::Error};

use bevy::prelude::*;
use bevy_rapier2d::{prelude::{RapierPhysicsPlugin, NoUserData}, render::RapierDebugRenderPlugin};
use bevy_replicon::{prelude::*, renet::{transport::{NetcodeClientTransport, ClientAuthentication, ServerConfig, ServerAuthentication, NetcodeServerTransport}, ConnectionConfig, SendType}};

use clap::Parser;

use super::{
    client::*, 
    enemies::{
        Enemy,
        EnemySpawning,
        spawning::{c_enemies_extras, s_spawn_enemies}, 
        moving::cs_move_enemies
    },
    server::*,
    common::*,
    abilities::{*, bullet::{Bullet, CanShootBullet, s_bullet_authority, c_bullet_extras}, default_class::{DefaultClassAbility, s_default_class_ability_response}, melee::{c_melee_extras, s_melee_authority, MeleeAttack}, melee_class::{s_melee_class_ability_response, MeleeClassEvent}, tags::CanUseAbilities, ranged_class::{s_ranged_class_response, RangedClassEvent}},
    player::*, 
    behaviours::{
        missile::{s_move_missiles, s_missile_authority, c_missile_extras}, 
        laser::{s_laser_authority, c_laser_extras}, 
        explosion::{Explosion, s_explosion_authority, c_explosion_extras}, 
        dead::s_destroy_dead_things, 
        collision::{s_collision_projectiles_damage, s_tick_damageable}, 
        projectile::ProjectileDamage
    }
};

pub const MOVE_SPEED: f32 = 300.0;

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
                RapierPhysicsPlugin::<NoUserData>::pixels_per_meter(100.0), 
                RapierDebugRenderPlugin::default()
            ))
            .configure_sets(FixedUpdate, (
                InputSystems.run_if(has_authority().or_else(resource_exists::<RenetClient>())),
                HostAndClientSystems.run_if(has_authority().or_else(resource_exists::<RenetClient>())),
                ClientSystems.run_if(resource_exists::<RenetClient>()),
                AuthoritySystems.run_if(has_authority()),
                ServerSystems.run_if(resource_exists::<RenetServer>()),
            ).chain())
            .insert_resource(EnemySpawning::new(0.35))
            .replicate::<Position>()
            .replicate::<Orientation>()
            .replicate::<PlayerColor>()
            .replicate::<Player>()
            .replicate::<PlayerClass>()
            .replicate::<Knockback>()
            .replicate::<Bullet>()
            .replicate::<Explosion>()
            .replicate::<MeleeAttack>()
            .replicate::<ProjectileDamage>()
            .replicate::<Velocity>()
            .replicate::<CanShootBullet>()
            .replicate::<CanUseAbilities>()
            .replicate::<Enemy>()
            .replicate::<Health>()
            .add_client_event::<MoveDirection>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<DefaultClassAbility>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<MeleeClassEvent>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<GeneralClientEvents>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<RangedClassEvent>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_systems(
                Startup,
                (
                    cli_system.map(Result::unwrap),
                    init_system,
                    c_setup_abilities,
                )
            )
            .add_systems(FixedUpdate, 
                (
                    s_conn_events,
                    s_general_client_events,
                ).chain().in_set(ServerSystems))
            .add_systems(FixedUpdate, 
                (
                    s_movement_events, 
                    s_spawn_enemies,
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
                ).chain().in_set(AuthoritySystems)
            )
            .add_systems(FixedUpdate, 
                (
                    c_movement_predict,
                    c_enemies_extras,
                    c_destroy_entites_without_match,
                ).chain().in_set(ClientSystems)
            )
            .add_systems(
                FixedUpdate,
                (
                    c_class_input_system,
                    c_movement_input
                ).chain().in_set(InputSystems)
            )
            .add_systems(
                FixedUpdate,
                (
                    cs_velocity_movement,
                    cs_velocity_damped_movement,
                    cs_update_trans_system,
                    cs_update_orientation_system,
                    cs_move_enemies,
                    c_predict_knockback,
                    c_bullet_extras,
                    c_melee_extras,
                    c_laser_extras,
                    c_explosion_extras,
                    c_missile_extras,
                    c_update_bullet_text,
                    c_class_change,
                ).chain().in_set(HostAndClientSystems)
            )
            .add_systems(PreUpdate, c_player_spawns.after(ClientSet::Receive));
    }
}

fn cli_system(
    mut commands: Commands,
    cli: Res<Cli>,
    network_channels: Res<NetworkChannels>,
) -> Result<(), Box<dyn Error>> {
    match *cli {
        Cli::SinglePlayer => {
            let ent = commands.spawn(PlayerServerBundle::new(SERVER_ID.raw(), Vec2::ZERO, Color::GREEN)).id();
            commands.insert_resource(LocalPlayerId{ is_host: true, id: SERVER_ID.raw(), entity: ent });
        }
        Cli::Server { port } => {
            info!("Starting a server on port {port}");
            let server_channels_config = network_channels.get_server_configs();
            let client_channels_config = network_channels.get_client_configs();

            let server = RenetServer::new(ConnectionConfig {
                server_channels_config,
                client_channels_config,
                ..Default::default()
            });

            let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
            let public_addr = SocketAddr::new(Ipv4Addr::UNSPECIFIED.into(), port);
            let socket = UdpSocket::bind(public_addr)?;
            let server_config = ServerConfig {
                current_time,
                max_clients: 10,
                protocol_id: PROTOCOL_ID,
                public_addresses: vec![public_addr],
                authentication: ServerAuthentication::Unsecure
            };
            let transport = NetcodeServerTransport::new(server_config, socket)?;

            commands.insert_resource(server);
            commands.insert_resource(transport);
            commands.insert_resource(LocalPlayerId{ is_host: true, id: SERVER_ID.raw(), entity: Entity::PLACEHOLDER });

            commands.spawn(TextBundle::from_section(
                "Server",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            commands.spawn(PlayerServerBundle::new(SERVER_ID.raw(), Vec2::ZERO, Color::GREEN));
        }
        Cli::Client { port, ip } => {
            info!("Starting a client connecting to: {ip:?}:{port}");
            let server_channels_config = network_channels.get_server_configs();
            let client_channels_config = network_channels.get_client_configs();

            info!("Making RenetClient...");
            let client = RenetClient::new(ConnectionConfig {
                server_channels_config,
                client_channels_config,
                ..Default::default()
            });

            let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
            let client_id = current_time.as_millis() as u64;
            info!("Getting server address...");
            let server_addr = SocketAddr::new(ip, port);
            info!("Binding server address...");
            let socket = UdpSocket::bind((Ipv4Addr::UNSPECIFIED, 0))?;
            let authentication = ClientAuthentication::Unsecure {
                client_id,
                protocol_id: PROTOCOL_ID,
                server_addr,
                user_data: None,
            };
            info!("Creating transport...");
            let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;

            commands.insert_resource(client);
            commands.insert_resource(transport);
            commands.insert_resource(LocalPlayerId{ is_host: false, id: client_id, entity: Entity::PLACEHOLDER });

            commands.spawn(TextBundle::from_section(
                format!("Client: {client_id:?}"),
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));

            info!("Donezo");
        }
    }

    Ok(())
}

fn init_system(mut commands: Commands)
{
    commands.spawn(Camera2dBundle::default());

    commands.spawn((TextBundle::from_section(
        "0 Bullets", 
        TextStyle { font_size: 30.0, color: Color::WHITE, ..default() }
    ).with_style(Style { 
        align_self: AlignSelf::FlexEnd, justify_self: JustifySelf::Start, flex_direction: FlexDirection::Column, ..default() 
    }), BulletText));
}

const PORT: u16 = 5003;
const PROTOCOL_ID: u64 = 0;

#[derive(Parser, PartialEq, Resource)]
pub enum Cli
{
    SinglePlayer,
    Server {
        #[arg(short, long, default_value_t = PORT)]
        port: u16
    },
    Client {
        #[arg(short, long, default_value_t = Ipv4Addr::LOCALHOST.into())]
        ip: IpAddr,

        #[arg(short, long, default_value_t = PORT)]
        port: u16
    }
}

impl Default for Cli
{
    fn default() -> Self {
        Self::parse()
    }
}

fn cs_update_trans_system(mut players: Query<(&Position, &mut Transform)>)
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

#[derive(Component)]
pub struct BulletText;

pub fn c_update_bullet_text(
    bullets: Query<(), With<Bullet>>,
    mut text: Query<&mut Text, (Without<Bullet>, With<BulletText>)>,
) {
    let Ok(mut text) = text.get_single_mut() else { return; };
    let bullet_count = bullets.iter().count();
    text.sections[0].value = String::from(format!("{bullet_count} bullets"));
}

