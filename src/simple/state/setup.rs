use bevy::{prelude::*, utils::hashbrown::HashMap};
use bevy_replicon::{renet::{transport::{ClientAuthentication, NetcodeClientTransport, NetcodeServerTransport, ServerAuthentication, ServerConfig}, ConnectionConfig, RenetClient, RenetServer}, replicon_core::NetworkChannels, server::SERVER_ID};
use clap::Parser;
use std::{error::Error, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};

use crate::simple::{classes::{bullet::Bullet, class::{Class, ClassBaseData, ClassType, Classes}, ranged_class, setup_classes}, enemies::setup_enemies, player::{LocalPlayerId, PlayerServerBundle}};

use super::GameState;

#[derive(Resource)]
pub struct WaitingHandles
{
    pub handles: Vec<Handle<ClassBaseData>>,
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


pub fn setup_class_assets(
    world: &mut World,
) {
    let mut handles = setup_classes(world);
    handles.append(&mut setup_enemies(world));
    
    world.insert_resource(WaitingHandles { handles })
}

pub fn wait_for_assets(
    mut next_state: ResMut<NextState<GameState>>,
    class_data_assets: Res<Assets<ClassBaseData>>,
    waiting_handles: Res<WaitingHandles>,
) {
    if waiting_handles.handles.iter().all(|x| { class_data_assets.get(x).is_some() })
    {
        info!("Class datas are ready, moving to in game");
        next_state.set(GameState::InGame);
    }
}

#[derive(Component)]
pub struct BulletText;


pub fn cli_system(
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

pub fn init_system(mut commands: Commands)
{
    commands.spawn(Camera2dBundle::default());

    commands.spawn((TextBundle::from_section(
        "0 Bullets", 
        TextStyle { font_size: 30.0, color: Color::WHITE, ..default() }
    ).with_style(Style { 
        align_self: AlignSelf::FlexEnd, justify_self: JustifySelf::Start, flex_direction: FlexDirection::Column, ..default() 
    }), BulletText));
}

pub fn c_update_bullet_text(
    bullets: Query<(), With<Bullet>>,
    mut text: Query<&mut Text, (Without<Bullet>, With<BulletText>)>,
) {
    let Ok(mut text) = text.get_single_mut() else { return; };
    let bullet_count = bullets.iter().count();
    text.sections[0].value = String::from(format!("{bullet_count} bullets"));
}
