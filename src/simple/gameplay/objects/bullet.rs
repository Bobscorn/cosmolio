use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_rapier2d::prelude::*;

use serde::{Deserialize, Serialize};

use crate::simple::{
    gameplay::{actor::{Damage, DamageKnockback, ActorChild, ChildType}, Position, Velocity, DestroyIfNoMatchWithin, Lifetime},
    consts::{PLAYER_PROJECTILE_GROUP, PLAYER_PROJECTILE_FILTER}
};




#[derive(Component, Deserialize, Serialize)]
pub struct Bullet
{
    pub size: f32,
    pub color: Color,
    pub lifetime: f32,
    pub knockback: f32,
    pub owner: Entity,
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
    damage: Damage,
    child: ActorChild,
    lifetime: Lifetime,
    collider: Collider,
    group: CollisionGroups,
    sensor: Sensor,
    collision_types: ActiveCollisionTypes,
    name: Name,
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
    pub fn new(pos: Vec2, color: Color, velocity: Vec2, size: f32, lifetime: f32, knockback: f32, owner: Entity) -> Self
    {
        Self
        {
            bullet: Bullet { size, color, lifetime, knockback, owner },
            position: Position(pos),
            velocity: Velocity(velocity),
            replication: Replication,
        }
    }
}

impl BulletAuthorityBundle
{
    pub fn new(pos: Vec2, size: f32, lifetime: f32, knockback: Vec2, owner: Entity) -> Self
    {
        Self
        {
            transform: TransformBundle { local: Transform::from_translation(pos.extend(0.0)), ..default() },
            damage: Damage::new(5.0, true, true, Some(DamageKnockback::Impulse(knockback))),
            child: ActorChild { ability_type:ChildType::Projectile, parent_actor: owner },
            lifetime: Lifetime(lifetime),
            collider: Collider::ball(size),
            group: CollisionGroups { memberships: PLAYER_PROJECTILE_GROUP, filters: PLAYER_PROJECTILE_FILTER },
            sensor: Sensor,
            collision_types: ActiveCollisionTypes::STATIC_STATIC,
            name: Name::new(format!("Bullet of {owner:?}")),
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
    received_bullets: Query<(Entity, &Bullet, &Position, &Velocity), Added<Replication>>
) {
    for (entity, bullet, position, vel) in &received_bullets
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue; };

        ent_coms.insert(BulletAuthorityBundle::new(position.0, bullet.size, bullet.lifetime, vel.normalize_or_zero() * bullet.knockback, bullet.owner));
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



