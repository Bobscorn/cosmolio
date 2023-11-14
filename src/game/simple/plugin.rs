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
        spawning::{receive_enemies, spawn_enemies}, 
        moving::move_enemies,
        collision::collision_projectiles_enemy,
        kill::server_kill_dead_enemies
    },
    server::*,
    common::*,
    abilities::{*, bullet::{Bullet, CanShootBullet, bullet_authority_system, bullet_extras_system}},
    player::*
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
            .configure_sets(Update, (
                InputSystems.run_if(has_authority().or_else(resource_exists::<RenetClient>())),
                HostAndClientSystems.run_if(has_authority().or_else(resource_exists::<RenetClient>())),
                ClientSystems.run_if(resource_exists::<RenetClient>()),
                AuthoritySystems.run_if(has_authority()),
                ServerSystems.run_if(resource_exists::<RenetServer>()),
            ).chain())
            .insert_resource(EnemySpawning::new(0.0))
            .replicate::<Position>()
            .replicate::<PlayerColor>()
            .replicate::<Player>()
            .replicate::<Bullet>()
            .replicate::<Velocity>()
            .replicate::<CanShootBullet>()
            .replicate::<Enemy>()
            .replicate::<Health>()
            .add_client_event::<MoveDirection>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_client_event::<AbilityActivation>(SendType::ReliableOrdered { resend_time: Duration::from_millis(300) })
            .add_mapped_server_event::<DestroyEntity>(SendType::ReliableUnordered { resend_time: Duration::from_millis(300) })
            .add_systems(
                Startup,
            (
                    cli_system.pipe(system_adapter::unwrap),
                    init_system
                )
            )
            .add_systems(Update, 
                (
                    server_event_system
                ).chain().in_set(ServerSystems))
            .add_systems(Update, 
                (
                    server_ability_response, 
                    movement_system, 
                    spawn_enemies,
                    collision_projectiles_enemy,
                    server_kill_dead_enemies,
                    kill_zero_healths,
                    bullet_authority_system,
                ).chain().in_set(AuthoritySystems)
            )
            .add_systems(Update, 
                (
                    client_movement_predict,
                    receive_enemies,
                    destroy_entites_without_match,
                ).chain().in_set(ClientSystems)
            )
            .add_systems(
                Update,
                (
                    ability_input_system,
                    movement_input_system
                ).chain().in_set(InputSystems)
            )
            .add_systems(
                Update,
                (
                    velocity_movement,
                    update_trans_system,
                    move_enemies,
                    bullet_extras_system,
                    update_bullet_text,
                ).chain().in_set(HostAndClientSystems)
            )
            .add_systems(PreUpdate, client_player_spawn_system.after(ClientSet::Receive));
    }
}

fn cli_system(
    mut commands: Commands,
    cli: Res<Cli>,
    network_channels: Res<NetworkChannels>,
) -> Result<(), Box<dyn Error>> {
    match *cli {
        Cli::SinglePlayer => {
            let ent = commands.spawn(PlayerServerBundle::new(SERVER_ID, Vec2::ZERO, Color::GREEN)).id();
            commands.insert_resource(LocalPlayerId{ id: SERVER_ID, entity: ent });
        }
        Cli::Server { port } => {
            info!("Starting a server on port {port}");
            let server_channels_config = network_channels.server_channels();
            let client_channels_config = network_channels.client_channels();

            let server = RenetServer::new(ConnectionConfig {
                server_channels_config,
                client_channels_config,
                ..Default::default()
            });

            let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
            let public_addr = SocketAddr::new(Ipv4Addr::LOCALHOST.into(), port);
            let socket = UdpSocket::bind(public_addr)?;
            let server_config = ServerConfig {
                max_clients: 10,
                protocol_id: PROTOCOL_ID,
                public_addr,
                authentication: ServerAuthentication::Unsecure,
            };
            let transport = NetcodeServerTransport::new(current_time, server_config, socket)?;

            commands.insert_resource(server);
            commands.insert_resource(transport);
            commands.insert_resource(LocalPlayerId{ id: SERVER_ID, entity: Entity::PLACEHOLDER });

            commands.spawn(TextBundle::from_section(
                "Server",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            commands.spawn(PlayerServerBundle::new(SERVER_ID, Vec2::ZERO, Color::GREEN));
        }
        Cli::Client { port, ip } => {
            info!("Starting a client connecting to: {ip:?}:{port}");
            let server_channels_config = network_channels.server_channels();
            let client_channels_config = network_channels.client_channels();

            let client = RenetClient::new(ConnectionConfig {
                server_channels_config,
                client_channels_config,
                ..Default::default()
            });

            let current_time = SystemTime::now().duration_since(SystemTime::UNIX_EPOCH)?;
            let client_id = current_time.as_millis() as u64;
            let server_addr = SocketAddr::new(ip, port);
            let socket = UdpSocket::bind((ip, 0))?;
            let authentication = ClientAuthentication::Unsecure {
                client_id,
                protocol_id: PROTOCOL_ID,
                server_addr,
                user_data: None,
            };
            let transport = NetcodeClientTransport::new(current_time, authentication, socket)?;

            commands.insert_resource(client);
            commands.insert_resource(transport);
            commands.insert_resource(LocalPlayerId{ id: client_id, entity: Entity::PLACEHOLDER });

            commands.spawn(TextBundle::from_section(
                format!("Client: {client_id:?}"),
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
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

fn update_trans_system(mut players: Query<(&Position, &mut Transform)>)
{
    for (player_pos, mut transform) in &mut players
    {
        transform.translation = player_pos.extend(0.0);
    }
}

pub fn velocity_movement(
    mut objects: Query<(&Velocity, &mut Position)>,
    time: Res<Time>
) {
    for (vel, mut pos) in &mut objects
    {
        pos.0 += vel.0 * time.delta_seconds();
    }
}

#[derive(Component)]
pub struct BulletText;

pub fn update_bullet_text(
    bullets: Query<(), With<Bullet>>,
    mut text: Query<&mut Text, (Without<Bullet>, With<BulletText>)>,
) {
    let Ok(mut text) = text.get_single_mut() else { return; };
    let bullet_count = bullets.iter().count();
    text.sections[0].value = String::from(format!("{bullet_count} bullets"));
}

