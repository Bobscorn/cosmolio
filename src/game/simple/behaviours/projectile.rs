use bevy::prelude::*;
use bevy_rapier2d::prelude::{Sensor, Collider, Group};
use bevy_replicon::prelude::Replication;

use serde::{Serialize, Deserialize};

use crate::game::simple::common::Position;


#[derive(Bundle)]
pub struct ProjectileServerBundle
{
    pub projectile: Projectile,
    pub position: Position,
    pub sprite_bundle: SpriteBundle,
    pub sensor: Sensor,
    pub collider: Collider,
    pub group: Group,
    pub replication: Replication
}

impl ProjectileServerBundle
{
    pub fn new(position: Vec2, group: Group) -> Self
    {
        Self
        {
            projectile: Projectile::default(),
            position: Position(position),
            sprite_bundle: SpriteBundle 
            { 
                sprite: Sprite { custom_size: Some(Vec2::new(15.0, 15.0)), ..default() }, 
                transform: Transform::from_translation(position.extend(0.0)), 
                ..default() 
            },
            sensor: Sensor,
            collider: Collider::ball(7.5),
            group,
            replication: Replication
        }
    }
}

#[derive(Component, Default)]
pub struct Projectile
{
    pub knockback: Vec2,
}

#[derive(Component, Serialize, Deserialize)]
pub struct ProjectileDamage
{
    pub damage: f32,
    pub destroy_on_damage: bool,
    pub did_damage: bool,
}

impl ProjectileDamage
{
    pub fn new(damage: f32, destroy_on_collision: bool) -> Self
    {
        Self
        {
            damage,
            destroy_on_damage: destroy_on_collision,
            did_damage: false
        }
    }

    pub fn should_destroy(&self) -> bool
    {
        self.destroy_on_damage
    }
}

