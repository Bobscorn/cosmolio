use bevy::{asset::{AssetLoader, AsyncReadExt}, prelude::*};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

use crate::simple::data::WaveData;

pub mod spawning;
pub mod moving;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Reflect, PartialEq)]
pub enum EnemySpawnType
{
    RegularEnemy,
}

#[derive(Reflect)]
pub struct WaveSpawnType
{
    pub target_count: u32,
    pub enemy_type: EnemySpawnType,
}

/// Resource containing the current wave number
#[derive(Resource, Reflect)]
pub struct CurrentWave
{
    pub wave: u32,
}

/// Event sent to clients when the wave has changed
#[derive(Clone, Copy, Debug, Event, Serialize, Deserialize)]
pub struct NewWave
{
    pub new_wave: u32,
}

/// This resource is used by the enemy spawning system to spawn new enemies
/// It uses the WaveData asset stored in `WaveDataResus` for logic.
#[derive(Resource, Reflect)]
pub struct WaveOverseer
{
    pub is_spawning: bool,
    pub points: f32, // Uses points to spawn enemies
    pub used_points: f32,
    pub next_spawn: WaveSpawnType,
}

impl EnemySpawnType
{
    pub fn points(&self) -> f32
    {
        match self
        {
            EnemySpawnType::RegularEnemy => 5.0_f32,
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
    pub fn new() -> Self
    {
        Self {
            is_spawning: false,
            points: 0.0,
            used_points: 0.0,
            next_spawn: WaveSpawnType { target_count: 1, enemy_type: EnemySpawnType::RegularEnemy },
        }
    }

    pub fn tick_next_spawn(&mut self, _dat: &WaveData, _maximum_points: f32)
    {
        let target_points = 50.0;

        let target_type = EnemySpawnType::RegularEnemy;

        self.next_spawn = WaveSpawnType { enemy_type: EnemySpawnType::RegularEnemy, target_count: (target_points / target_type.points()).floor() as u32 };
    }

    pub fn reset(&mut self)
    {
        self.points = 0.0;
        self.used_points = 0.0;
    }
}

#[derive(Component, Serialize, Deserialize)]
pub struct Enemy
{
    pub speed: f32
}

#[cfg(test)]
mod tests
{
    
    use std::{fs::File, io::{Read, Write}};

    use crate::simple::data::WaveData;

    const TEST_FILE_PATH: &str = "test_wave_data.wave_dat";

    #[test]
    fn test_wave_data()
    {
        let base_data = WaveData
        {
            point_growth_per_wave: 150.0,
            point_rate_growth_per_wave: 15.0,
            base_point_amount: 400.0,
            base_point_rate: 40.0,
            max_enemy_cost_threshold_growth_per_wave: 10.0,
            available_enemies: vec![],
        };

        let mut f = File::create(TEST_FILE_PATH).unwrap();

        ron::ser::to_writer(&f, &base_data).expect("could not serialize wave data");

        f.flush().unwrap();

        let mut f = File::open(TEST_FILE_PATH).unwrap();

        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes).expect("could not read bytes of serialized file");

        let new_data = ron::de::from_bytes::<WaveData>(&bytes).expect("could not deserialize wave data");
        
        assert_eq!(new_data, base_data);
        
        let _  = std::fs::remove_file(TEST_FILE_PATH);
    }
}
