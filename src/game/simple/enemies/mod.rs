use bevy::prelude::*;
use serde::{Deserialize, Serialize};

pub mod spawning;
pub mod moving;
pub mod collision;
pub mod kill;


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
