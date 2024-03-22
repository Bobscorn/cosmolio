use bevy::prelude::*;
use bevy_replicon::prelude::*;
use bevy_rapier2d::prelude::*;

use serde::{Deserialize, Serialize};

use crate::simple::{
    common::{Position, Lifetime, DestroyIfNoMatchWithin}, 
    behaviours::damage::{Damage, DamageKnockback}, 
    consts::{PLAYER_PROJECTILE_GROUP, ENEMY_COLLISION_GROUP}
};

#[derive(Serialize, Deserialize)]
pub enum MeleeAttackType
{
    Stab{ direction: Vec2, position: Vec2, length: f32, width: f32 },
    Circular{ position: Vec2, radius: f32 },
}

pub struct MeleeAttackData
{
    pub owning_client: u64,
    pub damage: f32,
    pub position: Vec2,
    pub direction: Vec2,
    pub attack_type: MeleeAttackType,
}

#[derive(Component, Deserialize, Serialize)]
pub struct MeleeAttack
{
    pub owning_client: u64,
    pub damage: f32,
    pub direction: Vec2,
    pub attack_type: MeleeAttackType,
}

/// This bundle should contain all the components a melee attack needs to send across the wire from server to clients.
/// 
/// This is the bundle to use to create a new melee attack. s_melee_authority and c_melee_extras will attach the other bundles as appropriate
/// 
/// Replication bundles contain the bare minimum required information.
/// All other components/bundles required for a bullet should be creatable using components in this bundle
#[derive(Bundle)]
pub struct MeleeReplicationBundle 
{
    melee: MeleeAttack,
    position: Position,
    replication: Replication, // Always add a replication component
}

/// This bullet bundle contains all the components a bullet needs on the server to work properly
/// 
/// This bundle should only really be added inside an authority_system
#[derive(Bundle)]
struct MeleeAuthorityBundle
{
    transform: TransformBundle, // e.g. a transform (but not sprite) bundle
    damage: Damage, 
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
struct MeleeExtrasBundle
{
    sprite_bundle: SpriteBundle,
    validation: DestroyIfNoMatchWithin,
}

impl MeleeReplicationBundle
{
    pub fn new(melee_attack: MeleeAttackData) -> Self
    {
        Self
        {
            melee: MeleeAttack 
            { 
                owning_client: melee_attack.owning_client, 
                damage: melee_attack.damage, 
                direction: melee_attack.direction,
                attack_type: melee_attack.attack_type
            },
            position: Position(melee_attack.position + melee_attack.direction * 20.0),
            replication: Replication,
        }
    }
}

impl MeleeAuthorityBundle
{
    pub fn new(pos: Vec2, direction: Vec2, damage: f32, ) -> Self
    {
        Self
        {
            transform: TransformBundle { local: Transform::from_translation(pos.extend(0.0)), ..default() },
            damage: Damage::new(damage, true, false, Some(DamageKnockback::Impulse(direction * 350.0))),
            lifetime: Lifetime(0.15),
            collider: Collider::ball(15.0),
            group: CollisionGroups { memberships: PLAYER_PROJECTILE_GROUP, filters: ENEMY_COLLISION_GROUP },
            collision_types: ActiveCollisionTypes::STATIC_STATIC
        }
    }
}

impl MeleeExtrasBundle
{
    pub fn new(pos: Vec2) -> Self
    {
        Self { 
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { color: Color::rgb(0.3, 0.3, 0.7), custom_size: Some(Vec2::new(15.0, 15.0)), ..default() }, 
                transform: Transform::from_translation(pos.extend(0.0)), 
                ..default() 
            },
            validation: DestroyIfNoMatchWithin::default(),
        }
    }
}

/// This melee authority system is responsible for adding the Authority bundle to a newly created entity
/// This will only happen on the server
pub fn s_melee_authority(
    mut commands: Commands,
    received_bullets: Query<(Entity, &MeleeAttack, &Position), Added<Replication>>
) {
    for (entity, melee, position) in &received_bullets
    {
        // SAFETY: entity is from a query and should never not exist
        commands.entity(entity).insert(MeleeAuthorityBundle::new(position.0, melee.direction, melee.damage));
    }
}

/// This template extras system is responsible for adding the Extras bundle to newly created/replicated entities
/// This will happen on clients and hosts (not dedicated servers)
pub fn c_melee_extras(
    mut commands: Commands,
    received_bullets: Query<(Entity, &Position), (With<MeleeAttack>, Added<Replication>)>,
) {
    for (entity,pos) in &received_bullets
    {
        // SAFETY: entity is from a query and should never not exist
        commands.entity(entity).insert(MeleeExtrasBundle::new(pos.0));
    }
}



