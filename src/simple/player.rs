use bevy::prelude::*;
use bevy_rapier2d::{dynamics::Velocity, prelude::{Collider, CollisionGroups, Sensor}};
use bevy_replicon::{prelude::*, renet::ServerEvent};

use serde::{Deserialize, Serialize};

use super::{
    behaviours::{collision::Damageable, effect::{ActorContext, ActorSensors}}, 
    classes::{bullet::CanShootBullet, class::{ActorClass, ClassType}, tags::CanUseAbilities}, 
    common::*, 
    consts::{PLAYER_GROUP, PLAYER_SENSOR_FILTER}, visuals::healthbar::HealthBar
};


#[derive(Resource, Reflect)]
pub struct LocalPlayerId
{
    pub is_host: bool,
    pub id: u64,
    pub entity: Entity
}

impl LocalPlayerId
{
    pub fn should_predict(&self) -> bool
    {
        !self.is_host
    }
}

#[derive(Component, Serialize, Deserialize, Reflect)]
pub struct Player(pub u64);

#[derive(Component, Reflect)]
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
    velocity: Velocity,
    color: PlayerColor,
    class: ActorClass,
    actor: ActorContext,
    knockback: Knockback,
    damageable: Damageable,
    can_shoot: CanShootBullet,
    can_use_abilities: CanUseAbilities,
    sensor: Sensor,
    actor_sensors: ActorSensors,
    collider: Collider,
    group: CollisionGroups,
    healthbar: HealthBar,
    name: Name,
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
            velocity: Velocity::zero(),
            color: PlayerColor(color), 
            class: ActorClass::new(ClassType::MeleeClass),
            actor: ActorContext::default(),
            knockback: Knockback::default(),
            damageable: Damageable { invulnerability_remaining: 0.0, invulnerability_duration: 0.5 },
            can_shoot: CanShootBullet,
            can_use_abilities: CanUseAbilities,
            sensor: Sensor,
            actor_sensors: ActorSensors { sensors: vec![] },
            collider: Collider::ball(12.5),
            group: CollisionGroups { memberships: PLAYER_GROUP, filters: PLAYER_SENSOR_FILTER },
            healthbar: HealthBar::default(),
            name: Name::new(format!("Player {id}")),
            replication: Replication,
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




