use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simple::enemies::spawning::EnemyData;

use super::classes::class::ClassBaseData;

pub mod spawning;
pub mod moving;

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

pub fn setup_enemies(
    world: &mut World
) -> Vec<Handle<ClassBaseData>> {

    let asset_server = world.resource::<AssetServer>();

    let regular_enemy_handle = asset_server.load("regular_enemy_data.cbd");

    world.insert_resource(EnemyData{ regular_enemy_data: regular_enemy_handle.clone() });

    vec![regular_enemy_handle]
}
