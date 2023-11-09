use bevy::prelude::*;
use bevy_rapier2d::prelude::{Sensor, Collider, CollisionGroups, ActiveCollisionTypes};
use bevy_replicon::prelude::Replication;

use crate::game::simple::{common::{Health, Position}, consts::{ENEMY_COLOR, ENEMY_BASE_SPEED, ENEMY_BASE_HEALTH, ENEMY_SPAWN_SEPARATION_RADIANS, ENEMY_MEMBER_GROUP, ENEMY_FILTER_GROUP}};

use super::{Enemy, EnemySpawning};

#[derive(Bundle)]
pub struct EnemyAuthorityBundle
{
    pub enemy: Enemy,
    pub health: Health,
    pub position: Position,
    pub replication: Replication,
    pub sprite_bundle: SpriteBundle,
    pub sensor: Sensor,
    pub collider: Collider,
    group: CollisionGroups,
    collision_types: ActiveCollisionTypes,
}

impl EnemyAuthorityBundle
{
    pub fn new(speed: f32, health: f32, position: Vec2) -> Self
    {
        Self 
        {
            enemy: Enemy { speed },
            health: Health(health),
            position: Position(position),
            replication: Replication,
            sprite_bundle: SpriteBundle 
            { 
                sprite: Sprite { color: ENEMY_COLOR, custom_size: Some(Vec2::new(35.0, 35.0)), ..default() }, 
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            },
            sensor: Sensor,
            collider: Collider::ball(35.0 / 2.0),
            group: CollisionGroups { memberships: ENEMY_MEMBER_GROUP, filters: ENEMY_FILTER_GROUP },
            collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC
        }
    }
}

#[derive(Bundle)]
pub struct EnemyReceiveBundle
{
    pub sprite_bundle: SpriteBundle
}

impl EnemyReceiveBundle
{
    pub fn new(position: Vec2) -> Self
    {
        Self
        {
            sprite_bundle: SpriteBundle 
            { 
                sprite: Sprite { color: ENEMY_COLOR, custom_size: Some(Vec2::new(35.0, 35.0)), ..default() }, 
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            }
        }
    }
}

pub fn spawn_enemies(
    mut commands: Commands,
    mut spawning: ResMut<EnemySpawning>,
    time: Res<Time>
) {
    if spawning.spawn_rate == 0.0
    {
        return;
    }

    let period = 1.0 / spawning.spawn_rate;

    spawning.left_over_time += time.delta_seconds();

    while spawning.left_over_time > period 
    {
        const ENEMY_SPAWN_DISTANCE: f32 = 150.0;

        let position = Vec2::new(spawning.last_spawn_radians.cos() * ENEMY_SPAWN_DISTANCE, spawning.last_spawn_radians.sin() * ENEMY_SPAWN_DISTANCE);

        info!("Spawning a new Enemy!");
        commands.spawn(EnemyAuthorityBundle::new(ENEMY_BASE_SPEED, ENEMY_BASE_HEALTH, position));

        spawning.last_spawn_radians += ENEMY_SPAWN_SEPARATION_RADIANS;
        spawning.left_over_time -= period;
    }
}

pub fn receive_enemies(
    mut commands: Commands,
    new_ents: Query<(Entity, &Position), (With<Enemy>, Without<Transform>, Added<Replication>)>
) {
    for (entity, position) in &new_ents
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue };

        info!("Received new Enemy!");
        ent_coms.insert(EnemyReceiveBundle::new(position.0));
    }
}


