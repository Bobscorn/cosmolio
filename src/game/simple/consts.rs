use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::Group;

pub const ENEMY_COLOR: Color = Color::rgb(0.25, 0.65, 0.1);
pub const ENEMY_BASE_SPEED: f32 = 50.0;
pub const ENEMY_BASE_HEALTH: f32 = 5.0;
pub const ENEMY_SPAWN_SEPARATION_RADIANS: f32 = PI * 0.25;

pub const PLAYER_MEMBER_GROUP: Group = Group::GROUP_1;
pub const PLAYER_FILTER_GROUP: Group = ENEMY_MEMBER_GROUP.union(ENEMY_PROJECTILE_GROUP);
pub const ENEMY_MEMBER_GROUP: Group = Group::GROUP_2;
pub const ENEMY_FILTER_GROUP: Group = PLAYER_MEMBER_GROUP.union(PLAYER_PROJECTILE_GROUP);

pub const ENEMY_PROJECTILE_GROUP: Group = PLAYER_MEMBER_GROUP;
pub const PLAYER_PROJECTILE_GROUP: Group = ENEMY_MEMBER_GROUP;


pub const BASE_BULLET_SPEED: f32 = 75.0;

pub const MELEE_DASH_SPEED: f32 = 350.0;
pub const MELEE_DASH_DURATION: f32 = 0.3;

