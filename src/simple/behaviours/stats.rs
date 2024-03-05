use bevy::prelude::*;
use serde::{Deserialize, Serialize};


// TODO: Confirm this design of stat
// some alternatives could be: hashmap<str, f32> (stat name indexes a float values of the stats)
// Vector<struct Stat> -> struct Stat { name: str, value: f32 }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Reflect)]
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
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub struct StatusEffect
{
    pub timeout: Option<f32>,
    pub stat: Stat,
    pub modification: StatModification,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
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


