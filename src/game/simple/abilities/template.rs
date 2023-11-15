use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_rapier2d::prelude::*;

use serde::{Deserialize, Serialize};

use crate::game::simple::{common::{Position, Velocity, Lifetime}, projectile::{Projectile, ProjectileDamage}, consts::{PLAYER_PROJECTILE_GROUP, ENEMY_MEMBER_GROUP}};




#[derive(Component, Deserialize, Serialize)]
pub struct TemplateComponentOne
{
    pub size: f32,
    pub color: Color,
}

#[derive(Component, Deserialize, Serialize)]
pub struct TemplateComponentTwo
{
    pub next: i32,
    pub is_cool: bool,
}

/// This bundle should contain all the components an entity needs to send across the wire from server to clients.
/// 
/// This is the bundle to use to create a new entity, a corresponding authority_system and extras_system should attach any other necessary components on appropriate players
/// 
/// Replication bundles contain the bare minimum required information.
/// All other components/bundles required for an entity should be creatable using components in this bundle
#[derive(Bundle)]
pub struct TemplateReplicationBundle 
{
    comp1: TemplateComponentOne,
    comp2: TemplateComponentTwo,
    position: Position,
    velocity: Velocity,
    replication: Replication, // Always add a replication component
}

/// This bullet bundle contains all the components a bullet needs on the server to work properly
/// 
/// This bundle should only really be added inside an authority_system
#[derive(Bundle)]
struct TemplateAuthorityBundle
{
    transform: TransformBundle, // e.g. a transform (but not sprite) bundle
    projectile: Projectile, // Necessary tag components
    damage: ProjectileDamage, // And necessary data components for the server to exhibit the intended behaviour
    lifetime: Lifetime,
    collider: Collider,
    group: CollisionGroups,
    collision_types: ActiveCollisionTypes
}

/// This bundle should contain all the extra components needed for use on clients (and hosts)
/// 
/// All components should in this bundle should be sourceable from the replication bundle
/// 
/// This bundle should only be added inside an extras_system
#[derive(Bundle)]
struct TemplateExtrasBundle
{
    sprite_bundle: SpriteBundle,
}

impl TemplateReplicationBundle
{
    pub fn new(pos: Vec2, color: Color, velocity: Vec2, size: f32) -> Self
    {
        Self
        {
            bullet: Bullet { size, color },
            position: Position(pos),
            velocity: Velocity(velocity),
            replication: Replication,
        }
    }
}

impl TemplateAuthorityBundle
{
    pub fn new(pos: Vec2, size: f32) -> Self
    {
        Self
        {
            transform: TransformBundle { local: Transform::from_translation(pos.extend(0.0)), ..default() },
            projectile: Projectile,
            damage: ProjectileDamage(5.0),
            lifetime: Lifetime(15.0),
            collider: Collider::ball(size),
            group: CollisionGroups { memberships: PLAYER_PROJECTILE_GROUP, filters: ENEMY_MEMBER_GROUP },
            collision_types: ActiveCollisionTypes::STATIC_STATIC
        }
    }
}

impl TemplateExtrasBundle
{
    pub fn new(pos: Vec2, color: Color, size: f32) -> Self
    {
        Self { 
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { color, custom_size: Some(Vec2::new(size, size)), ..default() }, 
                transform: Transform::from_translation(pos.extend(0.0)), 
                ..default() 
            }
        }
    }
}

/// This template authority system is responsible for adding the Authority bundle to a newly created entity
/// This will only happen on the server
pub fn template_authority_system(
    mut commands: Commands,
    received_bullets: Query<(Entity, &TemplateComponentOne, &Position), Added<Replication>>
) {
    for (entity, bullet, position) in &received_bullets
    {
        // SAFETY: entity is from a query and should never not exist
        commands.entity(entity).insert(TemplateAuthorityBundle::new(position.0, bullet.size));
    }
}

/// This template extras system is responsible for adding the Extras bundle to newly created/replicated entities
/// This will happen on clients and hosts (not dedicated servers)
pub fn bullet_extras_system(
    mut commands: Commands,
    received_bullets: Query<(Entity, &Bullet, &Position), Added<Replication>>,
) {
    for (entity, bullet, pos) in &received_bullets
    {
        // SAFETY: entity is from a query and should never not exist
        commands.entity(entity).insert(TemplateExtrasBundle::new(pos.0, bullet.color, bullet.size));
    }
}



