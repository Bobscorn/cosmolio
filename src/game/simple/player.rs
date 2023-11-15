use bevy::prelude::*;
use bevy_rapier2d::prelude::{Sensor, Collider, CollisionGroups};
use bevy_replicon::{prelude::*, renet::ServerEvent};

use serde::{Deserialize, Serialize};

use super::{abilities::{bullet::CanShootBullet, PlayerClass, ClassType}, common::*, consts::{PLAYER_MEMBER_GROUP, PLAYER_FILTER_GROUP}};


#[derive(Resource)]
pub struct LocalPlayerId
{
    pub id: u64,
    pub entity: Entity
}

#[derive(Component, Serialize, Deserialize)]
pub struct Player(pub u64);

#[derive(Component)]
pub struct LocalPlayer;

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor(pub Color);




#[derive(Bundle)]
pub struct PlayerClientBundle
{
    sprite_bundle: SpriteBundle
}

impl PlayerClientBundle
{
    pub fn new(color: Color, position: Vec2) -> Self
    {
        Self
        {
            sprite_bundle: SpriteBundle { sprite: Sprite { color, custom_size: Some(Vec2::new(25.0, 25.0)), ..default() }, transform: Transform::from_translation(position.extend(0.0)), ..default() }
        }
    }
}

#[derive(Bundle)]
pub struct PlayerServerBundle
{
    player: Player,
    position: Position,
    color: PlayerColor,
    class: PlayerClass,
    can_shoot: CanShootBullet,
    sensor: Sensor,
    collider: Collider,
    group: CollisionGroups,
    replication: Replication
}

impl PlayerServerBundle
{
    pub fn new(id: u64, position: Vec2, color: Color) -> Self
    {
        Self 
        { 
            player: Player(id), 
            position: Position(position), 
            color: PlayerColor(color), 
            class: PlayerClass { class: ClassType::DefaultClass },
            can_shoot: CanShootBullet,
            sensor: Sensor,
            collider: Collider::ball(12.5),
            group: CollisionGroups { memberships: PLAYER_MEMBER_GROUP, filters: PLAYER_FILTER_GROUP },
            replication: Replication
        }
    }
}



pub fn s_conn_events(
    mut commands: Commands, 
    mut server_event: EventReader<ServerEvent>,
    players: Query<(Entity, &Player)>,
) {
    for event in server_event.read()
    {
        match event
        {
            ServerEvent::ClientConnected { client_id } => {
                info!("Server: Client '{client_id}' has Connected");
                let client_id = client_id.raw();

                let r = ((client_id % 25) as f32) / 25.0;
                let g = ((client_id % 19) as f32) / 19.0;
                let b = ((client_id % 29) as f32) / 29.0;
                commands.spawn(PlayerServerBundle::new(
                    client_id,
                    Vec2::ZERO,
                    Color::rgb(r, g, b),
                ));
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Server: Client '{client_id}' disconnected: {reason}");

                for (entity, player) in &players
                {
                    if player.0 == client_id.raw()
                    {
                        commands.entity(entity).despawn_recursive();
                        break;
                    }
                }
            }
        }
    }
}


// Adds other non-replicated components to a Player entity when it has been replicated
pub fn c_player_spawns(
    mut commands: Commands, 
    query: Query<(Entity, &Player, &Position, &PlayerColor), Added<Replication>>,
    mut local_player: ResMut<LocalPlayerId>
) {
    for (entity, player, pos, color) in &query
    {
        let mut coms = commands.entity(entity);
        coms.insert(PlayerClientBundle::new(color.0, pos.0));
        let player_id = player.0;
        if player_id != local_player.id
        {
            continue;
        }
        
        info!("Inserting Local Player '{player_id}'");
        local_player.entity = coms.insert(LocalPlayer).id();
    }
}




