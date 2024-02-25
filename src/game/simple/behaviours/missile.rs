use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::geometry::{CollisionGroups, Collider, ActiveCollisionTypes};
use bevy_replicon::replicon_core::replication_rules::Replication;

use crate::game::simple::{
    common::{
        Position, 
        Velocity, 
        Orientation, 
        DestroyIfNoMatchWithin, 
        Lifetime, 
        VelocityDamping
    }, 
    consts::{
        RANGED_MAX_MISSILE_SPEED, 
        RANGED_MAX_MISSILE_ACCELERATION, 
        RANGED_MAX_MISSILE_ANGULAR_ACCELERATION, 
        RANGED_MISSILE_LIFETIME, 
        RANGED_MISSILE_LENGTH, 
        RANGED_MISSILE_WIDTH, 
        RANGED_MISSILE_COLOR
    }, 
    behaviours::{
        effect::ActorChild,
        damage::{Damage, DamageKnockback}
    }
};


#[derive(Component)]
pub struct Missile
{
    pub max_speed: f32,
    pub max_acceleration: f32,
    pub max_angular_acceleration: f32,
    pub owner: Entity,
}

#[derive(Bundle)]
pub struct MissileReplicationBundle
{
    pub missile: Missile,
    pub groups: CollisionGroups,
    pub position: Position,
    pub orientation: Orientation,
    pub velocity: Velocity,
    pub damage: Damage,
    pub replication: Replication,
}

#[derive(Bundle)]
pub struct MissileExtrasBundle
{
    pub sprite_bundle: SpriteBundle,
    pub validation: DestroyIfNoMatchWithin,
}

#[derive(Bundle)]
pub struct MissileAuthorityBundle
{
    pub transform: TransformBundle,
    pub ability_owner: ActorChild,
    pub lifetime: Lifetime,
    pub collider: Collider,
    pub damping: VelocityDamping,
    pub collision_types: ActiveCollisionTypes,
}

impl Missile
{
    pub fn from_owner(owner: Entity) -> Self {
        Self {
            max_speed: RANGED_MAX_MISSILE_SPEED,
            max_acceleration: RANGED_MAX_MISSILE_ACCELERATION,
            max_angular_acceleration: RANGED_MAX_MISSILE_ANGULAR_ACCELERATION,
            owner,
        }
    }
}

impl MissileReplicationBundle
{
    pub fn new(missile: Missile, position: Vec2, velocity: Vec2, damage: f32, groups: CollisionGroups, knockback: Option<DamageKnockback>) -> Self
    {
        Self {
            missile,
            position: Position(position),
            velocity: Velocity(velocity),
            orientation: Orientation(velocity.y.atan2(velocity.x)),
            groups,
            damage: Damage::new(damage, true, true, knockback),
            replication: Replication
        }
    }
}

impl MissileExtrasBundle
{
    pub fn new(transform: Transform) -> Self
    {
        Self {
            sprite_bundle: SpriteBundle {
                sprite: Sprite { color: RANGED_MISSILE_COLOR, custom_size: Some(Vec2::new(RANGED_MISSILE_LENGTH * 2.0, RANGED_MISSILE_WIDTH * 2.0)), ..default() },
                transform,
                ..default()
            },
            validation: DestroyIfNoMatchWithin::default()
        }
    }
}

impl MissileAuthorityBundle
{
    pub fn new(transform: Transform, owning_actor: Entity) -> Self
    {
        Self {
            transform: TransformBundle::from_transform(transform),
            ability_owner: ActorChild{ ability_type: super::effect::AbilityType::Missile, parent_actor: owning_actor },
            lifetime: Lifetime(RANGED_MISSILE_LIFETIME),
            damping: VelocityDamping(0.9),
            collider: Collider::cuboid(RANGED_MISSILE_LENGTH, RANGED_MISSILE_WIDTH),
            collision_types: ActiveCollisionTypes::STATIC_STATIC,
        }
    }
}


pub fn s_missile_authority(
    mut commands: Commands,
    new_missiles: Query<(Entity, &Position, &Orientation, &Missile), Added<Replication>>,
) {
    for (entity, position, orientation, missile) in &new_missiles
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue; };

        ent_coms.insert(MissileAuthorityBundle::new(
            Transform::from_translation(position.0.extend(0.0)) * Transform::from_rotation(Quat::from_rotation_z(orientation.0)),
            missile.owner
        ));
    }
}

pub fn c_missile_extras(
    mut commands: Commands,
    new_missiles: Query<(Entity, &Position, &Orientation), (With<Missile>, Added<Replication>)>,
) {
    for (entity, position, orientation) in &new_missiles
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue; };

        ent_coms.insert(MissileExtrasBundle::new(
            Transform::from_translation(position.0.extend(0.0)) * Transform::from_rotation(Quat::from_rotation_z(orientation.0))
        ));
    }
}

pub fn s_move_missiles(
    mut commands: Commands,
    mut missiles: Query<(&Position, &mut Velocity, &mut Orientation, &CollisionGroups, &Missile)>,
    stuff: Query<(&Position, &CollisionGroups), Without<Missile>>, 
    time: Res<Time>, 
) {
    for (
        missile_pos,
        mut missile_vel, 
        mut missile_orientation, 
        missile_group, 
        missile
    ) in &mut missiles
    {
        let mut nearest_dist_sq = f32::MAX;
        let mut nearest_pos = Vec2::ZERO;
        for (other_pos, other_group) in &stuff
        {
            if (missile_group.filters.bits() & other_group.memberships.bits()) == 0 ||
               (other_group.filters.bits()   & missile_group.memberships.bits()) == 0
            {
                continue;
            }

            let dist_sq = (missile_pos.0 - other_pos.0).length_squared();
            if dist_sq < nearest_dist_sq
            {
                nearest_dist_sq = dist_sq;
                nearest_pos = other_pos.0;
            }
        }

        if nearest_dist_sq != f32::MAX
        {
            // Rotation v
            let missile_to_pos = (nearest_pos - missile_pos.0).try_normalize().unwrap_or_default();
            
            let target_dir = missile_to_pos;
            let target_orientation = target_dir.y.atan2(target_dir.x);
            let angle_diff = (target_orientation - missile_orientation.0 + PI) % (2.0 * PI) - PI;
            let angular_acceleration = missile.max_angular_acceleration * angle_diff.signum();
            let mut increase = angular_acceleration * time.delta_seconds();
            if increase.abs() > angle_diff.abs()
            {
                increase = angle_diff;
            }
            missile_orientation.0 += increase;
        }

        
        
        // Velocity v
        let cur_speed = missile_vel.0.length();
        let missile_forward = (Quat::from_rotation_z(missile_orientation.0) * Vec3::X).truncate();
        let acceleration = (missile.max_acceleration).clamp(0.0, (missile.max_speed - cur_speed).max(0.0));

        missile_vel.0 = missile_forward * (cur_speed + acceleration * time.delta_seconds());
    }
}
