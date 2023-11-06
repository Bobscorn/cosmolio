use std::{net::{IpAddr, UdpSocket, SocketAddr, Ipv4Addr}, time::{SystemTime, Duration}, error::Error};

use bevy::prelude::*;
use bevy_replicon::{prelude::*, renet::{ServerEvent, transport::{NetcodeClientTransport, ClientAuthentication, ServerConfig, ServerAuthentication, NetcodeServerTransport}, ConnectionConfig, SendType}};

use clap::Parser;
use serde::{Deserialize, Serialize};

use super::client::*;
use super::server::*;

pub const MOVE_SPEED: f32 = 300.0;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ServerSystems;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct ClientSystems;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct AuthoritySystems;

#[derive(SystemSet, Clone, Debug, PartialEq, Eq, Hash)]
pub struct HostAndClientSystems;

pub struct SimpleGame;

impl Plugin for SimpleGame
{
    fn build(&self, app: &mut App) {
        app.configure_sets(Update, (
                ServerSystems.run_if(resource_exists::<RenetServer>()),
                AuthoritySystems.run_if(has_authority()),
                ClientSystems.run_if(resource_exists::<RenetClient>()),
                HostAndClientSystems.run_if(has_authority().or_else(resource_exists::<RenetClient>()))
            ))
            .replicate::<Position>()
            .replicate::<PlayerColor>()
            .replicate::<Player>()
            .replicate::<Bullet>()
            .replicate::<Velocity>()
            .add_client_event::<MoveDirection>(SendType::ReliableOrdered { resend_time: Duration::from_millis(500) })
            .add_client_event::<AbilityActivation>(SendType::ReliableOrdered { resend_time: Duration::from_millis(500) })
            .add_systems(
                Startup,
            (
                    cli_system.pipe(system_adapter::unwrap),
                    init_system
                )
            )
            .add_systems(Update, (server_event_system).chain().in_set(ServerSystems))
            .add_systems(Update, (server_ability_response, movement_system).chain().in_set(AuthoritySystems))
            .add_systems(Update, (client_movement_predict, client_bullet_receive_system).chain().in_set(ClientSystems))
            .add_systems(
                Update,
                (
                    velocity_movement,
                    ability_input_system,
                    movement_input_system,
                    update_trans_system
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

#[derive(Resource)]
pub struct LocalPlayerId
{
    pub id: u64,
    pub entity: Entity
}



#[derive(Component, Serialize, Deserialize)]
pub struct Player(pub u64);

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct Position(pub Vec2);

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor(pub Color);

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Deserialize, Serialize)]
pub struct Bullet
{
    pub size: Vec2
}

#[derive(Bundle)]
pub struct BulletBundle
{
    pub bullet: Bullet,
    pub position: Position,
    pub velocity: Velocity,
    pub sprite_bundle: SpriteBundle,
    pub rep: Replication
}

impl BulletBundle
{
    pub fn new(pos: Vec2, velocity: Vec2, size: Vec2) -> Self
    {
        Self 
        { 
            bullet: Bullet { size }, 
            position: Position(pos),
            velocity: Velocity(velocity),
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { color: Color::rgb(0.5, 0.25, 0.15), custom_size: Some(size), ..default() }, 
                transform: Transform::from_translation(pos.extend(0.0)), 
                ..default() 
            },
            rep: Replication
        }
    }
}

#[derive(Bundle)]
pub struct BulletReceiveBundle
{
    pub sprite_bundle: SpriteBundle
}

impl BulletReceiveBundle
{
    pub fn new(pos: Vec2, size: Vec2) -> Self
    {
        Self { 
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { color: Color::rgb(0.5, 0.25, 0.15), custom_size: Some(size), ..default() }, 
                transform: Transform::from_translation(pos.extend(0.0)), 
                ..default() 
            }
        }
    }
}

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub enum AbilityActivation
{
    #[default]
    None,
    ShootBullet(Entity)
}
