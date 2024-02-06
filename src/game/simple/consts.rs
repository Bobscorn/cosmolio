use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::{prelude::Group, geometry::CollisionGroups};

pub const SERVER_STR: &str = "\x1b[94mServer\x1b[0m";
pub const CLIENT_STR: &str = "\x1b[93mClient:\x1b[0m";

pub const ENEMY_COLOR: Color = Color::rgb(0.25, 0.65, 0.1);
pub const ENEMY_BASE_SPEED: f32 = 100.0;
pub const ENEMY_BASE_HEALTH: f32 = 5.0;
pub const ENEMY_SPAWN_SEPARATION_RADIANS: f32 = PI * 0.25;

pub const PLAYER_MEMBER_GROUP: Group = Group::GROUP_1;
pub const PLAYER_FILTER_GROUP: Group = ENEMY_MEMBER_GROUP.union(ENEMY_PROJECTILE_GROUP);
pub const PLAYER_GROUPS: CollisionGroups = CollisionGroups { memberships: PLAYER_MEMBER_GROUP, filters: PLAYER_FILTER_GROUP };
pub const ENEMY_MEMBER_GROUP: Group = Group::GROUP_2;
pub const ENEMY_FILTER_GROUP: Group = PLAYER_MEMBER_GROUP.union(PLAYER_PROJECTILE_GROUP);
pub const ENEMY_GROUPS: CollisionGroups = CollisionGroups { memberships: ENEMY_MEMBER_GROUP, filters: ENEMY_FILTER_GROUP };

pub const ENEMY_PROJECTILE_GROUP: Group = Group::GROUP_3;
pub const PLAYER_PROJECTILE_GROUP: Group = Group::GROUP_4;
pub const PLAYER_PROJECTILE_GROUPS: CollisionGroups = CollisionGroups { memberships: PLAYER_PROJECTILE_GROUP, filters: ENEMY_MEMBER_GROUP };


pub const BASE_BULLET_SPEED: f32 = 75.0;
pub const DEFAULT_LASER_WIDTH: f32 = 4.5f32;

pub const DEFAULT_CLASS_BULLET_LIFETIME: f32 = 4.0;

pub const MELEE_DASH_SPEED: f32 = 350.0;
pub const MELEE_DASH_DURATION: f32 = 0.3;
pub const MELEE_ATTACK_LIFETIME: f32 = 0.25;

pub const RANGED_BULLET_SPEED: f32 = 175.0;
pub const RANGED_BULLET_SIZE: f32 = 7.5;
pub const RANGED_BULLET_LIFETIME: f32 = 4.0;
pub const RANGED_GRENADE_SPEED: f32 = 75.0;
pub const RANGED_GRENADE_SIZE: f32 = 7.5;
pub const RANGED_GRENADE_EXPLOSION_SIZE: f32 = 50.0;
pub const RANGED_GRENADE_FUSE_TIME: f32 = 2.5;
pub const RANGED_GRENADE_EXPLOSION_KNOCKBACK_STRENGTH: f32 = 450.0;
pub const RANGED_GRENADE_DAMAGE: f32 = 1.0;
pub const RANGED_SHOTGUN_DISTANCE: f32 = 75.0;
pub const RANGED_SHOTGUN_KNOCKBACK: f32 = 350.0;
pub const RANGED_SHOTGUN_SELF_KNOCKBACK_SPEED: f32 = 350.0;
pub const RANGED_SHOTGUN_SELF_KNOCKBACK_DURATION: f32 = 0.15;

pub const RANGED_BULLET_COLOR: Color = Color::rgb(0.15, 0.5, 0.69);
pub const RANGED_GRENADE_COLOR: Color = Color::rgb(0.6, 0.6, 0.4);

pub const RANGED_MAX_MISSILE_SPEED: f32 = 1200.0;
pub const RANGED_MISSILE_INITIAL_SPEED: f32 = 50.0;
pub const RANGED_MAX_MISSILE_ACCELERATION: f32 = 1000.0;
pub const RANGED_MAX_MISSILE_ANGULAR_ACCELERATION: f32 = 2.0 * PI; // Radians/s
pub const RANGED_MISSILE_LIFETIME: f32 = 3.0;
pub const RANGED_MISSILE_WIDTH: f32 = 5.0;
pub const RANGED_MISSILE_LENGTH: f32 = 10.0;
pub const RANGED_MISSILE_DAMAGE: f32 = 1.0;
pub const RANGED_MISSILE_EXPLOSION_RADIUS: f32 = 15.0;
pub const RANGED_MISSILE_EXPLOSION_DAMAGE: f32 = 1.5;
pub const RANGED_MISSILE_EXPLOSION_KNOCKBACK_STRENGTH: f32 = 200.0;

pub const RANGED_MISSILE_COLOR: Color = Color::rgb(0.8, 0.4, 0.4);

