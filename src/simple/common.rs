use bevy::prelude::*;
use bevy_replicon::prelude::ServerEntityMap;
use serde::{Deserialize, Serialize};

use crate::simple::consts::CLIENT_STR;

use super::{behaviours::effect::ActorContext, player::LocalPlayer};


#[derive(Component, Deserialize, Serialize, Deref, DerefMut, Reflect)]
pub struct Position(pub Vec2);

/// A component describing an orientation about an axis (typically the z-axis)
/// 
/// Unit is radians
#[derive(Component, Deserialize, Serialize, Deref, DerefMut, Reflect)]
pub struct Orientation(pub f32);

#[derive(Component, Deserialize, Serialize, Deref, DerefMut, Reflect)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct VelocityDamping(pub f32);

#[derive(Component, Clone, Copy, Default, Deserialize, Serialize, Reflect)]
pub struct Knockback
{
    pub velocity: Vec2,
    pub control_points: [f32; 3], // These control points indicate what percentage of velocity should be applied at any given point in time, see the knockback_system for more info.
    pub time_remaining: f32,
    pub total_time: f32,
}

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Serialize, Deserialize)]
pub struct Dead;

#[derive(Component, Debug, Deref, DerefMut, Reflect)]
pub struct Lifetime(pub f32);

/// Use this component to mark an entity as 'waiting for a server mapping'.
/// Any entity with this component that has no server mapping once it's lifetime has expired will be deleted
/// This component will be removed if a mapping is found
#[derive(Component)]
pub struct DestroyIfNoMatchWithin
{
    pub remaining_time: f32,
}

impl Default for DestroyIfNoMatchWithin
{
    fn default() -> Self {
        Self { remaining_time: 0.15 }
    }
}


impl Velocity
{
    pub fn apply_impulse(&mut self, impulse: Vec2)
    {
        self.0 += impulse;
    }
}

impl Knockback
{
    pub const DEFAULT_CONTROL_POINTS: [f32; 3] = [1.0, 1.0, 1.0];

    pub fn new(velocity: Vec2, duration: f32, control_points: [f32; 3]) -> Self
    {
        Self { velocity, control_points, time_remaining: duration, total_time: duration }
    }

    pub fn has_knockback(&self) -> bool
    {
        self.time_remaining > 0.0 && self.total_time > 0.0
    }

    pub fn get_current_knockback(&self) -> Option<Vec2>
    {
        if self.time_remaining <= 0.0 || self.total_time <= 0.0
        {
            return None;
        }

        let normalised_point = (self.time_remaining / self.total_time).clamp(0.0, 1.0);

        let scale = 2.0;
        let mut upper_point = 1;
        let mut lower_point = 0;
        
        if normalised_point < 0.5
        {
            upper_point = 2;
            lower_point = 1;
        }
        let upper_point = upper_point;
        let lower_point = lower_point;

        let control_point_factor = self.control_points[lower_point] + (self.control_points[upper_point] - self.control_points[lower_point]) * normalised_point * scale;

        Some(self.velocity * control_point_factor)
    }

    pub fn tick_knockback_time(&mut self, delta_time: f32)
    {
        self.time_remaining = (self.time_remaining - delta_time).max(0.0);
    }
}

pub fn s_kill_zero_healths(
    mut commands: Commands,
    health_havers: Query<(Entity, &ActorContext), Without<Dead>>
) {
    for (entity, actor) in &health_havers
    {
        let Some(health) = actor.get_stat(&super::behaviours::stats::Stat::Health) else { continue; };
        if health > 0.0
        {
            continue;
        }

        let Some(mut ent_coms) = commands.get_entity(entity) else { continue };

        ent_coms.insert(Dead);
    }
}

/// This (client-only - not host) system monitors entities with the [`DestroyIfNoMatch`] component and destroys them if no match is found before they expire
/// This was used to prevent duplicates existing of predicted entities, but now bevy_replicon seems to have fixed that
pub fn c_destroy_entites_without_match(
    mut commands: Commands,
    mut match_seekers: Query<(Entity, &mut DestroyIfNoMatchWithin)>,
    time: Res<Time<Real>>, 
    mappings: Res<ServerEntityMap>,
) {
    for (entity, mut lifetime) in &mut match_seekers
    {
        lifetime.remaining_time -= time.delta_seconds();
        if mappings.to_server().contains_key(&entity)
        {
            info!("{CLIENT_STR}: Entity found match");
            commands.entity(entity).remove::<DestroyIfNoMatchWithin>();
            continue;
        }

        if lifetime.remaining_time <= 0.0
        {
            info!("{CLIENT_STR}: Destroyed Entity with no match");
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn s_update_and_destroy_lifetimes(
    mut commands: Commands,
    mut entities: Query<(Entity, &mut Lifetime)>,
    time: Res<Time>,
) {
    for (entity, mut lifetime) in &mut entities
    {
        lifetime.0 -= time.delta_seconds();
        
        if lifetime.0 > 0.0
        {
            continue;
        }

        commands.entity(entity).insert(Dead);
    }
}

pub fn s_knockback(
    mut knockables: Query<(&mut Knockback, &mut Position), Without<bevy_rapier2d::prelude::RigidBody>>,
    mut knock_bodies: Query<(&mut Knockback, &mut bevy_rapier2d::prelude::Velocity), With<bevy_rapier2d::prelude::RigidBody>>,
    time: Res<Time>,
) {
    // info!("There are {} knockables", knockables.iter().count());
    for (mut knockback, mut position) in &mut knockables
    {
        let Some(knockback_velocity) = knockback.get_current_knockback() else { continue; };

        knockback.tick_knockback_time(time.delta_seconds());

        position.0 += knockback_velocity * time.delta_seconds();
    }

    // info!("There are {} knock bodies", knock_bodies.iter().count());
    for (mut knockback, mut vel) in &mut knock_bodies
    {
        // Possibly bad to just set velocity directly?
        let Some(knockback_velocity) = knockback.get_current_knockback() else { continue; };

        knockback.tick_knockback_time(time.delta_seconds());

        vel.linvel = knockback_velocity;
    }
}

pub fn c_predict_knockback(
    mut player: Query<(&mut Knockback, &mut Position), With<LocalPlayer>>,
    time: Res<Time>,
) {
    for (mut knockback, mut position) in &mut player
    {
        let Some(knockback_velocity) = knockback.get_current_knockback() else { continue; };

        knockback.tick_knockback_time(time.delta_seconds());

        position.0 += knockback_velocity * time.delta_seconds();
    }
}
