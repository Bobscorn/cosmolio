use bevy::prelude::*;
use bevy_rapier2d::geometry::{CollisionGroups, Collider, ActiveCollisionTypes};
use bevy_replicon::replicon_core::replication_rules::Replication;
use serde::{Serialize, Deserialize};

use crate::game::simple::{
    behaviours::damage::{Damage, DamageKnockback},
    common::{
        Position, 
        Lifetime, 
        DestroyIfNoMatchWithin
    }, 
    consts::DEFAULT_LASER_WIDTH
};

#[derive(Bundle)]
pub struct LaserReplicationBundle
{
    pub laser: Laser,
    pub position: Position,
    pub groups: CollisionGroups,
    pub damage: Damage,
    pub replication: Replication,
}

#[derive(Bundle)]
pub struct LaserExtrasBundle
{
    pub sprite_bundle: SpriteBundle,
    pub validation: DestroyIfNoMatchWithin,
}

#[derive(Bundle)]
pub struct LaserAuthorityBundle
{
    pub transform: TransformBundle,
    pub lifetime: Lifetime,
    pub collider: Collider,
    pub collision_types: ActiveCollisionTypes,
}

#[derive(Component, Serialize, Deserialize)]
pub struct Laser
{
    pub color: Color,
    pub length: f32,
    pub direction: Vec2,
    pub knockback: f32,
}

impl LaserReplicationBundle
{
    pub fn new(color: Color, length: f32, position: Vec2, direction: Vec2, damage: f32, knockback: f32, groups: CollisionGroups) -> Self
    {
        let real_pos = position + direction * length * 0.5;
        Self { 
            laser: Laser { color, length, direction, knockback }, 
            position: Position(real_pos),
            groups,
            damage: Damage::new(damage, false, false, Some(DamageKnockback::Impulse(direction * knockback))),
            replication: Replication
        }
    }
}

impl LaserAuthorityBundle
{
    pub fn new(length: f32, direction: Vec2, position: Vec2, knockback: f32) -> Self
    {
        Self {
            transform: TransformBundle { 
                local: Transform::from_translation(position.extend(0.0)) * Transform::from_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x))), 
                ..default()
            },
            lifetime: Lifetime(0.5),
            collider: Collider::cuboid(length * 0.5, DEFAULT_LASER_WIDTH * 0.5),
            collision_types: ActiveCollisionTypes::STATIC_STATIC,
        }
    }
}

impl LaserExtrasBundle
{
    pub fn new(color: Color, length: f32, direction: Vec2, position: Vec2) -> Self
    {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite { color, custom_size: Some(Vec2::new(length, DEFAULT_LASER_WIDTH)), ..default() },
                transform: Transform::from_translation(position.extend(0.0)) * Transform::from_rotation(Quat::from_rotation_z(direction.y.atan2(direction.x))),
                ..default()
            },
            validation: DestroyIfNoMatchWithin::default(),
        }
    }
}


pub fn s_laser_authority(
    mut commands: Commands,
    new_lasers: Query<(Entity, &Laser, &Position), Added<Replication>>,
) {
    for (entity, laser, position) in &new_lasers
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue; };

        ent_coms.insert(LaserAuthorityBundle::new(laser.length, laser.direction, position.0, laser.knockback));
    }
}

pub fn c_laser_extras(
    mut commands: Commands,
    new_lasers: Query<(Entity, &Laser, &Position), Added<Replication>>,
) {
    for (entity, laser, position) in &new_lasers
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue; };

        ent_coms.insert(LaserExtrasBundle::new(laser.color, laser.length, laser.direction, position.0));
    }
}

