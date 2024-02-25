use bevy::prelude::*;
use bevy_rapier2d::geometry::{CollisionGroups, Collider, ActiveCollisionTypes};
use bevy_replicon::replicon_core::replication_rules::Replication;

use serde::{Serialize, Deserialize};

use crate::game::simple::{
    common::{Position, Lifetime, DestroyIfNoMatchWithin}, 
    behaviours::damage::{Damage, DamageKnockback}
};


#[derive(Component, Serialize, Deserialize)]
pub struct Explosion
{
    pub radius: f32,
    pub knockback_strength: f32,
}

#[derive(Bundle)]
pub struct ExplosionReplicationBundle
{
    pub explosion: Explosion,
    pub position: Position,
    pub damage: Damage,
    pub groups: CollisionGroups,
    pub replication: Replication,
}


#[derive(Bundle)]
pub struct ExplosionAuthorityBundle
{
    pub transform: TransformBundle,
    pub lifetime: Lifetime,
    pub collider: Collider,
    pub collision_types: ActiveCollisionTypes,
}


#[derive(Bundle)]
pub struct ExplosionExtrasBundle
{
    pub sprite_bundle: SpriteBundle,
    pub valdiation: DestroyIfNoMatchWithin,
}


impl ExplosionReplicationBundle
{
    pub fn new(radius: f32, knockback_strength: f32, position: Vec2, damage: f32, groups: CollisionGroups, knockback: Option<DamageKnockback>) -> Self
    {
        Self
        {
            explosion: Explosion { radius, knockback_strength },
            position: Position(position),
            damage: Damage::new(damage, false, false, knockback),
            groups,
            replication: Replication
        }
    }
}

impl ExplosionAuthorityBundle
{
    pub fn new(radius: f32, position: Vec2) -> Self
    {
        Self
        {
            transform: TransformBundle::from_transform(Transform::from_translation(position.extend(0.0))),
            lifetime: Lifetime(1.0),
            collider: Collider::ball(radius),
            collision_types: ActiveCollisionTypes::STATIC_STATIC,
        }
    }
}

impl ExplosionExtrasBundle
{
    pub fn new(radius: f32, position: Vec2) -> Self
    {
        Self
        {
            sprite_bundle: SpriteBundle
            {
                sprite: Sprite { custom_size: Some(Vec2::new(radius * 2.0, radius * 2.0)), ..default() },
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            },
            valdiation: DestroyIfNoMatchWithin::default()
        }
    }
}


pub fn s_explosion_authority(
    mut commands: Commands,
    new_explosions: Query<(Entity, &Explosion, &Position), (Added<Replication>)>,
) {
    for (entity, expl, pos) in &new_explosions
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue; };

        ent_coms.insert(ExplosionAuthorityBundle::new(expl.radius, pos.0));
    }
}

pub fn c_explosion_extras(
    mut commands: Commands,
    new_explosions: Query<(Entity, &Explosion, &Position), Added<Replication>>,
) {
    for (entity, expl, pos) in &new_explosions
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue; };

        ent_coms.insert(ExplosionExtrasBundle::new(expl.radius, pos.0));
    }
}

