use std::thread::spawn;

use bevy::prelude::*;
use bevy_replicon::prelude::Replication;
use serde::{Serialize, Deserialize};

use crate::game::simple::{common::{Health, Position}, consts::{ENEMY_COLOR, ENEMY_BASE_SPEED, ENEMY_BASE_HEALTH, ENEMY_SPAWN_SEPARATION_RADIANS}};

#[derive(Resource)]
pub struct EnemySpawning
{
    pub spawn_rate: f32,
    pub left_over_time: f32,
    pub last_spawn_radians: f32
}

impl EnemySpawning
{
    pub fn new(spawn_rate: f32) -> Self
    {
        Self {
            spawn_rate,
            left_over_time: 0.0,
            last_spawn_radians: 0.0
        }
    }
}

#[derive(Component, Serialize, Deserialize)]
pub struct Enemy
{
    pub speed: f32
}

#[derive(Bundle)]
pub struct EnemyAuthorityBundle
{
    pub enemy: Enemy,
    pub health: Health,
    pub position: Position,
    pub replication: Replication,
    pub sprite_bundle: SpriteBundle
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
            }
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

        ent_coms.insert(EnemyReceiveBundle::new(position.0));
    }
}


