use bevy::prelude::*;
use bevy_rapier2d::prelude::{Sensor, Collider, Group};
use bevy_replicon::prelude::Replication;

use serde::{Serialize, Deserialize};

use crate::simple::common::Position;


#[derive(Bundle)]
pub struct ProjectileServerBundle
{
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