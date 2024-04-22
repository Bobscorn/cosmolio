use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simple::gameplay::EnemySpawnType;

use super::ron_asset::RonSerializedAsset;

/// Resource containing all the data required for wave spawning enemies and scaling them with waves
#[derive(Asset, Debug, Default, Serialize, Deserialize, Reflect, PartialEq)]
pub struct WaveData
{
    pub base_point_amount: f32,
    pub point_growth_per_wave: f32,
    pub base_point_rate: f32,
    pub point_rate_growth_per_wave: f32,
    pub max_enemy_cost_threshold_growth_per_wave: f32,
    pub available_enemies: Vec<EnemySpawnType>,
}

impl RonSerializedAsset for WaveData
{
    fn extensions() -> &'static [&'static str] {
        &[".wave_dat"]
    }
}

#[derive(Resource, Reflect)]
pub struct WaveDataResus
{
    pub dat: Handle<WaveData>,
}

pub fn setup_wave_data(
    world: &mut World
) -> Vec<UntypedHandle> {
    let asset_server = world.resource::<AssetServer>();

    let wave_data = asset_server.load("default_wave_data.wave_dat");

    world.insert_resource(WaveDataResus{ dat: wave_data.clone() });

    vec![wave_data.untyped()]
}
