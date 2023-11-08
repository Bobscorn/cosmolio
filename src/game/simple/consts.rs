use std::f32::consts::PI;

use bevy::prelude::*;

pub const ENEMY_COLOR: Color = Color::rgb(0.25, 0.65, 0.1);
pub const ENEMY_BASE_SPEED: f32 = 2.0;
pub const ENEMY_BASE_HEALTH: f32 = 5.0;
pub const ENEMY_SPAWN_SEPARATION_RADIANS: f32 = PI * 0.25;

