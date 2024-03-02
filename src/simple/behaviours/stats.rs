use std::{error::Error, fmt::Display};

use bevy::{asset::{AssetLoader, AsyncReadExt}, prelude::*, utils::hashbrown::HashMap};
use serde::{Deserialize, Serialize};


// TODO: Confirm this design of stat
// some alternatives could be: hashmap<str, f32> (stat name indexes a float values of the stats)
// Vector<struct Stat> -> struct Stat { name: str, value: f32 }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq)]
pub enum Stat
{
    Health, // The health of the actor
    Armor, // A damage reduction stat, not implemented TODO: this stat
    Damage, // A damage stat that scales (almost) all damage
    MovementSpeed, // How many units an actor moves whilst walking per second
    CooldownRate, // How fast a cooldown finishes, total duration will be: normal_duration / CooldownRate
}

/// A modification to one of an actor's Stats.
/// This can be temporary, or permanent.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct StatusEffect
{
    pub timeout: Option<f32>,
    pub stat: Stat,
    pub modification: StatModification,
}

#[derive(Asset, Clone, TypePath)]
pub struct BaseStats
{
    pub stats: HashMap<Stat, f32>,
}

#[derive(Serialize, Deserialize)]
pub struct BaseStatsSerial
{
    pub stats: Vec<SerializedStat>
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum StatModification
{
    Multiply{ factor: f32 },
    Add{ amount: f32 },
    Exponent{ power: f32 }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedStat
{
    pub stat: Stat,
    pub value: f32,
}


impl Into<BaseStats> for BaseStatsSerial
{
    fn into(self) -> BaseStats {
        BaseStats { stats: HashMap::from_iter(self.stats.iter().map(|x| { (x.stat, x.value) })) }
    }
}

#[derive(Default)]
pub struct BaseStatsDataLoader;

#[derive(Debug)]
pub enum BaseStatsLoadError
{
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
}

impl From<std::io::Error> for BaseStatsLoadError
{
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for BaseStatsLoadError
{
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

impl Display for BaseStatsLoadError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            BaseStatsLoadError::Io(e) => f.write_fmt(format_args!("Io error: {}", e)),
            BaseStatsLoadError::Ron(e) => f.write_fmt(format_args!("Ron error: {}", e)),
        }
    }
}

impl Error for BaseStatsLoadError {}

impl AssetLoader for BaseStatsDataLoader
{
    type Asset = BaseStats;
    type Settings = ();
    type Error = BaseStatsLoadError;

    fn load<'a>(
            &'a self,
            reader: &'a mut bevy::asset::io::Reader,
            _settings: &'a Self::Settings,
            _load_context: &'a mut bevy::asset::LoadContext,
        ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<BaseStatsSerial>(&bytes)?.into();
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &[".cbd"]
    }
}

