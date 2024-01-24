use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_rapier2d::prelude::*;

use serde::{Deserialize, Serialize};

use crate::game::simple::{common::{Position, Velocity, Lifetime, DestroyIfNoMatchWithin}, behaviours::projectile::{Projectile, ProjectileDamage}, consts::{PLAYER_PROJECTILE_GROUP, ENEMY_MEMBER_GROUP}, behaviours::effect::{Effect, OnDestroy}};




#[derive(Component, Deserialize, Serialize)]
pub struct Bullet
{
    pub size: f32,
    pub color: Color,
    pub lifetime: f32,
    pub on_destroy: Effect,
}

/// This bullet bundle contains all the components a bullet has that will be sent across the wire from server to clients.
/// 
/// This is the bundle to use to create a new bullet, bullet_authority_system and bullet_extras_system will attach any other necessary components on appropriate players
/// 
/// Replication bundles contain the bare minimum required information.
/// All other components/bundles required for a bullet should be creatable using components in this bundle
#[derive(Bundle)]
pub struct BulletReplicationBundle 
{
    bullet: Bullet,
    position: Position,
    velocity: Velocity,
    replication: Replication,
}

/// This bullet bundle contains all the components a bullet needs on the server to work properly
/// 
/// This bundle should only really be added inside the bullet_authority_system
#[derive(Bundle)]
struct BulletAuthorityBundle
{
    transform: TransformBundle,
    projectile: Projectile,
    damage: ProjectileDamage,
    lifetime: Lifetime,
    on_destroy: OnDestroy,
    collider: Collider,
    group: CollisionGroups,
    collision_types: ActiveCollisionTypes
}

/// This bullet bundle contains all the extra components created from the replication components
/// 
/// This bundle should only be added inside the bullet_extras_system
#[derive(Bundle)]
struct BulletExtrasBundle
{
    sprite_bundle: SpriteBundle,
    validation: DestroyIfNoMatchWithin,
}

impl BulletReplicationBundle
{
    pub fn new(pos: Vec2, color: Color, velocity: Vec2, size: f32, lifetime: f32, on_destroy: Effect) -> Self
    {
        Self
        {
            bullet: Bullet { size, color, lifetime, on_destroy },
            position: Position(pos),
            velocity: Velocity(velocity),
            replication: Replication,
        }
    }
}

impl BulletAuthorityBundle
{
    pub fn new(pos: Vec2, size: f32, lifetime: f32, on_destroy_effect: Effect) -> Self
    {
        Self
        {
            transform: TransformBundle { local: Transform::from_translation(pos.extend(0.0)), ..default() },
            projectile: Projectile::default(),
            damage: ProjectileDamage::new(5.0, true, true),
            lifetime: Lifetime(lifetime),
            on_destroy: OnDestroy { effect: on_destroy_effect },
            collider: Collider::ball(size),
            group: CollisionGroups { memberships: PLAYER_PROJECTILE_GROUP, filters: ENEMY_MEMBER_GROUP },
            collision_types: ActiveCollisionTypes::STATIC_STATIC
        }
    }
}

impl BulletExtrasBundle
{
    pub fn new(pos: Vec2, color: Color, size: f32) -> Self
    {
        Self { 
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { color, custom_size: Some(Vec2::new(size, size)), ..default() }, 
                transform: Transform::from_translation(pos.extend(0.0)), 
                ..default() 
            },
            validation: DestroyIfNoMatchWithin::default()
        }
    }
}


#[derive(Component, Serialize, Deserialize)]
pub struct CanShootBullet;

/// This system (Authority only) adds the BulletAuthorityBundle to newly created bullets on the Server/Singleplayer
pub fn s_bullet_authority(
    mut commands: Commands,
    received_bullets: Query<(Entity, &Bullet, &Position), Added<Replication>>
) {
    for (entity, bullet, position) in &received_bullets
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue; };

        ent_coms.insert(BulletAuthorityBundle::new(position.0, bullet.size, bullet.lifetime, bullet.on_destroy));
    }
}

/// This system adds the Extras bundle to bullets that were recently spawned
/// It is assumed all new bullet entities do not already have the extras bundle
pub fn c_bullet_extras(
    mut commands: Commands,
    received_bullets: Query<(Entity, &Bullet, &Position), Added<Replication>>,
) {
    for (entity, bullet, pos) in &received_bullets
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue };

        ent_coms.insert(BulletExtrasBundle::new(pos.0, bullet.color, bullet.size));
        info!("New Bullet Found: {entity:?}");
    }
}



