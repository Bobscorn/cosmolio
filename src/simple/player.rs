use bevy::prelude::*;
use bevy_rapier2d::{dynamics::Velocity, prelude::{Collider, CollisionGroups, Sensor}};
use bevy_replicon::prelude::*;

use serde::{Deserialize, Serialize};

use super::{
    behaviours::{collision::Damageable, effect::{ActorContext, ActorSensors}}, 
    classes::{bullet::CanShootBullet, class::{ActorClass, ClassType}, tags::CanUseAbilities}, 
    common::*, 
    consts::{PLAYER_GROUP, PLAYER_SENSOR_FILTER}, visuals::{healthbar::HealthBar, Images}
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

#[derive(Component, Serialize, Deserialize)]
pub struct Player(pub ClientId);

#[derive(Component, Reflect)]
pub struct LocalPlayer;

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor(pub Color);




#[derive(Bundle)]
pub struct PlayerClientBundle
{
    sprite_bundle: SpriteBundle,
    healthbar: HealthBar,
    name: Name,
}

impl PlayerClientBundle
{
    pub fn new(id: ClientId, color: Color, position: Vec2, tex: Handle<Image>) -> Self
    {
        Self
        {
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { color, custom_size: Some(Vec2::new(25.0, 25.0)), ..default() }, 
                texture: tex,
                transform: Transform::from_translation(position.extend(0.0)), 
                ..default() 
            },
            healthbar: HealthBar::default(),
            name: Name::new(format!("Player {}", id.get())),
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
    replication: Replication
}

impl PlayerServerBundle
{
    pub fn new(id: ClientId, position: Vec2, color: Color) -> Self
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
                info!("Server: Client '{}' has Connected", client_id.get());
                let p_id = client_id.get();

                let r = ((p_id % 25) as f32) / 25.0;
                let g = ((p_id % 19) as f32) / 19.0;
                let b = ((p_id % 29) as f32) / 29.0;
                commands.spawn(PlayerServerBundle::new(
                    *client_id,
                    Vec2::ZERO,
                    Color::rgb(r, g, b),
                ));
            }
            ServerEvent::ClientDisconnected { client_id, reason } => {
                info!("Server: Client '{}' disconnected: {reason}", client_id.get());

                for (entity, player) in &players
                {
                    if &player.0 == client_id
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
    imgs: Res<Images>,
    query: Query<(Entity, &Player, &Position, &PlayerColor), Added<Replication>>,
    mut local_player: ResMut<LocalPlayerId>
) {
    for (entity, player, pos, color) in &query
    {
        let mut coms = commands.entity(entity);
        coms.insert(PlayerClientBundle::new(player.0, color.0, pos.0, imgs.player_img.clone()));
        let player_id = player.0;
        if player_id.get() != local_player.id
        {
            continue;
        }
        
        info!("Inserting Local Player '{}'", player_id.get());
        local_player.entity = coms.insert(LocalPlayer).id();
    }
}




