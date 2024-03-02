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
    MaxHealth, // The max health of the actor
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

