use std::{net::{IpAddr, UdpSocket, SocketAddr, Ipv4Addr}, time::{SystemTime, Duration}, error::Error};

use bevy::prelude::*;
use bevy_replicon::{prelude::*, renet::{ServerEvent, transport::{NetcodeClientTransport, ClientAuthentication, ServerConfig, ServerAuthentication, NetcodeServerTransport}, ConnectionConfig, SendType}};

use clap::Parser;
use serde::{Deserialize, Serialize};

use super::client::*;
use super::server::*;

pub const MOVE_SPEED: f32 = 300.0;

pub struct SimpleGame;

impl Plugin for SimpleGame
{
    fn build(&self, app: &mut App) {
        app.replicate::<PlayerPosition>()
            .replicate::<PlayerColor>()
            .replicate::<Player>()
            .replicate::<Bullet>()
            .add_client_event::<MoveDirection>(SendType::ReliableOrdered { resend_time: Duration::from_millis(500) })
            .add_client_event::<AbilityActivation>(SendType::ReliableOrdered { resend_time: Duration::from_millis(500) })
            .add_systems(
                Startup,
            (
                    cli_system.pipe(system_adapter::unwrap),
                    init_system
                )
            )
            .add_systems(
                Update,
                (
                    movement_system.run_if(has_authority()),
                    client_movement_predict.run_if(resource_exists::<RenetClient>()),
                    server_event_system.run_if(resource_exists::<RenetServer>()),
                    server_ability_response.run_if(has_authority()),
                    server_bullet_movement.run_if(has_authority()),
                    ability_input_system,
                    movement_input_system,
                    update_trans_system
                )
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

fn update_trans_system(mut players: Query<(&PlayerPosition, &mut Transform)>)
{
    for (player_pos, mut transform) in &mut players
    {
        transform.translation = player_pos.extend(0.0);
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
pub struct PlayerPosition(pub Vec2);

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor(pub Color);

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Deserialize, Serialize)]
pub struct Bullet
{
    pub velocity: Vec2,
}

#[derive(Bundle)]
pub struct BulletBundle
{
    pub bullet: Bullet,
    pub sprite_bundle: SpriteBundle
}

impl BulletBundle
{
    pub fn new(pos: Vec2, velocity: Vec2, size: Vec2) -> Self
    {
        Self 
        { 
            bullet: Bullet{ velocity }, 
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
