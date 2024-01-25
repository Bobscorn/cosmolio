use bevy::prelude::*;
use bevy_rapier2d::prelude::{Sensor, Collider, Group};
use bevy_replicon::prelude::Replication;

use serde::{Serialize, Deserialize};

use crate::game::simple::common::Position;


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

#[derive(Serialize, Deserialize)]
pub enum ProjectileKnockbackType
{
    Impulse(Vec2), // Push an object in a direction
    Repulsion{ center: Vec2, strength: f32 }, // Repel an object away from a point
    Attraction{ center: Vec2, strength: f32 }, // Attract an object towards a point
}

#[derive(Component, Serialize, Deserialize)]
pub struct ProjectileDamage
{
    pub damage: f32,
    pub destroy_on_damage: bool,
    pub deal_damage_once: bool,
    pub knockback: Option<ProjectileKnockbackType>,
    pub did_damage: bool,
}

impl ProjectileDamage
{
    pub fn new(damage: f32, destroy_on_damage: bool, deal_damage_once: bool, knockback: Option<ProjectileKnockbackType>) -> Self
    {
        Self
        {
            damage,
            destroy_on_damage,
            deal_damage_once,
            knockback,
            did_damage: false
        }
    }

    pub fn should_destroy(&self) -> bool
    {
        self.destroy_on_damage
    }

    pub fn new_typical_bullet(damage: f32) -> Self
    {
        Self
        {
            damage,
            destroy_on_damage: true,
            deal_damage_once: true,
            knockback: None,
            did_damage: false,
        }
    }

    pub fn new_typical_explosion(damage: f32) -> Self
    {
        Self
        {
            damage,
            destroy_on_damage: false,
            deal_damage_once: false,
            knockback: None,
            did_damage: false,
        }
    }

    pub fn new_typical_laser(damage: f32) -> Self
    {
        Self
        {
            damage, 
            destroy_on_damage: false,
            deal_damage_once: false,
            knockback: None,
            did_damage: false,
        }
    }

    pub fn new_typical_hitscan(damage: f32) -> Self
    {
        Self
        {
            damage,
            destroy_on_damage: false,
            deal_damage_once: true,
            knockback: None,
            did_damage: false,
        }
    }

    pub fn with_destroy_on_damage(&mut self, destroy_on_damage: bool) -> &mut Self
    {
        self.destroy_on_damage = destroy_on_damage;
        self
    }

    pub fn with_knockback(&mut self, knockback: Option<ProjectileKnockbackType>) -> &mut Self
    {
        self.knockback = knockback;
        self
    }
}

