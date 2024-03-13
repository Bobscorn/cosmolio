use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simple::enemies::spawning::EnemyData;

use super::classes::class::ClassBaseData;

pub mod spawning;
pub mod moving;

#[derive(Reflect)]
pub enum SpawnType
{
    RegularEnemy,
}

#[derive(Reflect)]
pub struct WaveSpawnType
{
    pub target_count: u32,
    pub enemy_type: SpawnType,
}

#[derive(Resource, Reflect)]
pub struct WaveOverseer
{
    pub is_spawning: bool,
    pub wave_number: u32,
    pub point_rate: f32,
    pub points: f32, // Uses points to spawn enemies
    pub next_spawn: WaveSpawnType,
}

impl SpawnType
{
    pub fn points(&self) -> f32
    {
        match self
        {
            SpawnType::RegularEnemy => 5.0_f32,
        }
    }
}

impl WaveSpawnType
{
    pub fn required_points(&self) -> f32
    {
        let base_points = self.enemy_type.points();

        base_points * self.target_count as f32
    }
}

impl WaveOverseer
{
    pub fn new(point_rate: f32) -> Self
    {
        Self {
            is_spawning: false,
            wave_number: 0,
            point_rate,
            points: 0.0,
            next_spawn: WaveSpawnType { target_count: 1, enemy_type: SpawnType::RegularEnemy },
        }
    }

    pub fn tick_next_spawn(&mut self)
    {
        let target_points = 100.0;

        let target_type = SpawnType::RegularEnemy;

        self.next_spawn = WaveSpawnType { enemy_type: SpawnType::RegularEnemy, target_count: (target_points / target_type.points()).floor() as u32 };
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
