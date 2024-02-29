use std::ops::{Add, Div, Mul, Sub};

use bevy::{prelude::*, utils::hashbrown::HashMap};
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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, DerefMut, Deref)]
pub struct StatValue
{
    pub base_value: f32,
    #[deref]
    pub value: f32
}

impl Mul<f32> for StatValue
{
    type Output = Self;
    fn mul(self, rhs: f32) -> Self::Output {
        Self {
            base_value: self.base_value,
            value: self.value * rhs
        }
    }
}

impl Div<f32> for StatValue
{
    type Output = Self;
    fn div(self, rhs: f32) -> Self::Output {
        Self {
            base_value: self.base_value,
            value: self.value / rhs
        }
    }
}

impl Add<f32> for StatValue
{
    type Output = Self;
    fn add(self, rhs: f32) -> Self::Output {
        Self {
            base_value: self.base_value,
            value: self.value + rhs
        }
    }
}

impl Sub<f32> for StatValue
{
    type Output = Self;
    fn sub(self, rhs: f32) -> Self::Output {
        Self {
            base_value: self.base_value,
            value: self.value - rhs
        }
    }
}

impl StatValue
{
    pub fn new(base_value: f32) -> Self
    {
        Self { base_value, value: base_value }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StatModification
{
    Multiply{ factor: f32 },
    Add{ amount: f32 },
    Exponent{ power: f32 }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StatusEffect
{
    pub timeout: Option<f32>,
    pub stat: Stat,
    pub modification: StatModification,
}

#[derive(Clone, Copy, Serialize, Deserialize)]
pub struct SerializedBaseStat
{
    pub stat: Stat,
    pub value: f32,
}
