use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::{prelude::Group, geometry::CollisionGroups};

pub const SERVER_STR: &str = "\x1b[94mServer\x1b[0m";
pub const CLIENT_STR: &str = "\x1b[93mClient:\x1b[0m";

pub const ENEMY_COLOR: Color = Color::rgb(0.25, 0.65, 0.1);
pub const ENEMY_BASE_SPEED: f32 = 100.0;
pub const ENEMY_BASE_HEALTH: f32 = 5.0;
pub const ENEMY_SPAWN_SEPARATION_RADIANS: f32 = PI * 0.25;

pub const PLAYER_GROUP: Group = Group::GROUP_1; // Layer that players exist on

pub const PLAYER_COLLISION_FILTER: Group = PLAYER_COLLISION_GROUP.union(ENEMY_COLLISION_GROUP); // Layers that players collide with

pub const PLAYER_SENSOR_FILTER: Group = ENEMY_SENSOR_GROUP.union(ENEMY_PROJECTILE_GROUP); // Layers that Player abilities and sensors should intersect with


pub const ENEMY_GROUP: Group = Group::GROUP_3; // The group that enemy hitboxes exist on

pub const ENEMY_COLLISION_FILTER: Group = PLAYER_COLLISION_GROUP.union(PLAYER_PROJECTILE_GROUP).union(ENEMY_COLLISION_GROUP); // Stuff that enemies collide with

pub const ENEMY_SENSOR_FILTER: Group = PLAYER_SENSOR_GROUP.union(PLAYER_PROJECTILE_GROUP); // Layers that enemies can 'hit'


pub const ENEMY_PROJECTILE_GROUP: Group = Group::GROUP_5;
pub const PLAYER_PROJECTILE_GROUP: Group = Group::GROUP_6;
pub const PLAYER_PROJECTILE_GROUPS: CollisionGroups = CollisionGroups { memberships: PLAYER_PROJECTILE_GROUP, filters: ENEMY_COLLISION_GROUP };


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

