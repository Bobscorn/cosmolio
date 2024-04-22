use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_replicon_renet::renet::{RenetServer, RenetClient, transport::*, ConnectionConfig};
use bevy_replicon_renet::RenetChannelsExt;
use clap::Parser;
use std::{error::Error, net::{IpAddr, Ipv4Addr, SocketAddr, UdpSocket}, time::SystemTime};

use crate::simple::{
    state::{GameState, SetupSystems},
    data::WaitingHandles,
    player::{PlayerServerBundle, LocalPlayerId},
    ui::InfoText,
};

pub struct SetupPlugin;

impl Plugin for SetupPlugin
{
    fn build(&self, app: &mut App)
    {
        app
            .init_resource::<Cli>()
            .add_systems(Startup, (cli_system.map(Result::unwrap), init_system))
            .add_systems(Update, wait_for_assets.in_set(SetupSystems))
            ;
    }
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

fn wait_for_assets(
    mut next_state: ResMut<NextState<GameState>>,
    asset_server: Res<AssetServer>,
    waiting_handles: Res<WaitingHandles>,
) {
    if waiting_handles.handles.iter().all(|x| { asset_server.get_load_state(x.id()) == Some(bevy::asset::LoadState::Loaded) })
    {
        info!("All outstanding handles are ready, moving to in game");
        next_state.set(GameState::ChoosingClass);
    }
}


fn cli_system(
    mut commands: Commands,
    cli: Res<Cli>,
    network_channels: Res<RepliconChannels>,
) -> Result<(), Box<dyn Error>> {
    match *cli {
        Cli::SinglePlayer => {
            let ent = commands.spawn(PlayerServerBundle::new(ClientId::SERVER, Vec2::ZERO, Color::GREEN)).id();
            commands.insert_resource(LocalPlayerId{ is_host: true, id: ClientId::SERVER.get(), entity: ent });
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
            commands.insert_resource(LocalPlayerId{ is_host: true, id: ClientId::SERVER.get(), entity: Entity::PLACEHOLDER });

            commands.spawn(TextBundle::from_section(
                "Server",
                TextStyle {
                    font_size: 30.0,
                    color: Color::WHITE,
                    ..default()
                },
            ));
            commands.spawn(PlayerServerBundle::new(ClientId::SERVER, Vec2::ZERO, Color::GREEN));
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

    let txt_style = TextStyle {
        color: Color::WHITE,
        font_size: 30.0,
        ..default()
    };
    commands.spawn((
        TextBundle {
            style: Style { 
                align_self: AlignSelf::FlexEnd, justify_self: JustifySelf::Start, flex_direction: FlexDirection::Column, ..default() 
            },
            text: Text { 
                sections: vec![
                    TextSection::new("No enemies", txt_style.clone()), 
                    TextSection::new("\n", txt_style.clone()),
                    TextSection::new("You are dead", txt_style.clone())
                    ], 
                linebreak_behavior: bevy::text::BreakLineOn::WordBoundary,
                justify: JustifyText::Left,
            },
            ..default()
        },
        InfoText,
        Name::new("Info Text"),
    ));
}
