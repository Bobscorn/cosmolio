use bevy::{asset::{AssetLoader, AsyncReadExt}, prelude::*};
use serde::{Deserialize, Serialize};
use std::{error::Error, fmt::Display};

use crate::simple::enemies::spawning::EnemyData;

pub mod spawning;
pub mod moving;

#[derive(Clone, Copy, Debug, Serialize, Deserialize, Reflect, PartialEq)]
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

/// Resource containing all the data required for wave spawning enemies and scaling them with waves
#[derive(Asset, Debug, Serialize, Deserialize, Reflect, PartialEq)]
pub struct WaveData
{
    pub base_point_amount: f32,
    pub point_growth_per_wave: f32,
    pub base_point_rate: f32,
    pub point_rate_growth_per_wave: f32,
    pub max_enemy_cost_threshold_growth_per_wave: f32,
    pub available_enemies: Vec<SpawnType>,
}

#[derive(Resource, Reflect)]
pub struct WaveDataResus
{
    pub dat: Handle<WaveData>,
}

#[derive(Default)]
pub struct WaveDataLoader;

#[derive(Debug)]
pub enum WaveDataLoadError
{
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
}

impl From<std::io::Error> for WaveDataLoadError
{
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for WaveDataLoadError
{
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

impl Display for WaveDataLoadError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            WaveDataLoadError::Io(e) => f.write_fmt(format_args!("Io error: {}", e)),
            WaveDataLoadError::Ron(e) => f.write_fmt(format_args!("Ron error: {}", e)),
        }
    }
}

impl Error for WaveDataLoadError {}

impl AssetLoader for WaveDataLoader
{
    type Asset = WaveData;
    type Settings = ();
    type Error = WaveDataLoadError;

    fn load<'a>(
            &'a self,
            reader: &'a mut bevy::asset::io::Reader,
            _settings: &'a Self::Settings,
            _load_context: &'a mut bevy::asset::LoadContext,
        ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<WaveData>(&bytes)?.into();
            Ok(custom_asset)
        })
    }
    fn extensions(&self) -> &[&str] {
        &["wave_dat"]
    }
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
    pub fn new() -> Self
    {
        Self {
            is_spawning: false,
            points: 0.0,
            used_points: 0.0,
            next_spawn: WaveSpawnType { target_count: 1, enemy_type: SpawnType::RegularEnemy },
        }
    }

    pub fn tick_next_spawn(&mut self, _dat: &WaveData, _maximum_points: f32)
    {
        let target_points = 50.0;

        let target_type = SpawnType::RegularEnemy;

        self.next_spawn = WaveSpawnType { enemy_type: SpawnType::RegularEnemy, target_count: (target_points / target_type.points()).floor() as u32 };
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

pub fn setup_enemies(
    world: &mut World
) -> Vec<UntypedHandle> {

    let asset_server = world.resource::<AssetServer>();

    let regular_enemy_handle = asset_server.load("regular_enemy_data.cbd");

    world.insert_resource(EnemyData{ regular_enemy_data: regular_enemy_handle.clone() });

    vec![regular_enemy_handle.untyped()]
}

pub fn setup_wave_data(
    world: &mut World
) -> Vec<UntypedHandle> {
    let asset_server = world.resource::<AssetServer>();

    let wave_data = asset_server.load("default_wave_data.wave_dat");

    world.insert_resource(WaveDataResus{ dat: wave_data.clone() });

    vec![wave_data.untyped()]
}

#[cfg(test)]
mod tests
{
    
    use std::{fs::File, io::{Read, Write}};

    use super::WaveData;

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
