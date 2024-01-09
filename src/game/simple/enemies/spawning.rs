use bevy::prelude::*;
use bevy_rapier2d::prelude::{Sensor, Collider, CollisionGroups, ActiveCollisionTypes};
use bevy_replicon::prelude::Replication;

use crate::game::simple::{common::{Health, Position, Velocity}, consts::{ENEMY_COLOR, ENEMY_BASE_SPEED, ENEMY_BASE_HEALTH, ENEMY_SPAWN_SEPARATION_RADIANS, ENEMY_MEMBER_GROUP, ENEMY_FILTER_GROUP}, behaviours::collision::Damageable};

use super::{Enemy, EnemySpawning};

/// This authority bundle acts as the replication bundle as well, simply due to the fact only the server ever spawns enemies
/// This means any clients will see only the replicated components
#[derive(Bundle)]
pub struct EnemyAuthorityBundle
{
    pub enemy: Enemy,
    pub health: Health,
    pub damage: Damageable,
    pub position: Position,
    pub velocity: Velocity,
    pub replication: Replication,
    // ^ Replicated components
    // v Non replicated components
    pub sprite_bundle: SpriteBundle,
    pub sensor: Sensor,
    pub collider: Collider,
    group: CollisionGroups,
    collision_types: ActiveCollisionTypes,
}

impl EnemyAuthorityBundle
{
    pub fn new(speed: f32, health: f32, position: Vec2) -> Self
    {
        Self 
        {
            enemy: Enemy { speed },
            health: Health(health),
            damage: Damageable { invulnerability_duration: 0.25, invulnerability_remaining: 0.5 },
            position: Position(position),
            velocity: Velocity(Vec2::ZERO),
            replication: Replication,
            sprite_bundle: SpriteBundle 
            { 
                sprite: Sprite { color: ENEMY_COLOR, custom_size: Some(Vec2::new(35.0, 35.0)), ..default() }, 
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            },
            sensor: Sensor,
            collider: Collider::ball(35.0 / 2.0),
            group: CollisionGroups { memberships: ENEMY_MEMBER_GROUP, filters: ENEMY_FILTER_GROUP },
            collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC
        }
    }
}

#[derive(Bundle)]
pub struct EnemyExtrasBundle
{
    pub sprite_bundle: SpriteBundle
}

impl EnemyExtrasBundle
{
    pub fn new(position: Vec2) -> Self
    {
        Self
        {
            sprite_bundle: SpriteBundle 
            { 
                sprite: Sprite { color: ENEMY_COLOR, custom_size: Some(Vec2::new(35.0, 35.0)), ..default() }, 
                transform: Transform::from_translation(position.extend(0.0)),
                ..default()
            }
        }
    }
}

pub fn s_spawn_enemies(
    mut commands: Commands,
    mut spawning: ResMut<EnemySpawning>,
    time: Res<Time>
) {
    if spawning.spawn_rate == 0.0
    {
        return;
    }

    let period = 1.0 / spawning.spawn_rate;

    spawning.left_over_time += time.delta_seconds();

    while spawning.left_over_time > period 
    {
        const ENEMY_SPAWN_DISTANCE: f32 = 150.0;

        let position = Vec2::new(spawning.last_spawn_radians.cos() * ENEMY_SPAWN_DISTANCE, spawning.last_spawn_radians.sin() * ENEMY_SPAWN_DISTANCE);

        info!("Spawning a new Enemy!");
        commands.spawn(EnemyAuthorityBundle::new(ENEMY_BASE_SPEED, ENEMY_BASE_HEALTH, position));

        spawning.last_spawn_radians += ENEMY_SPAWN_SEPARATION_RADIANS;
        spawning.left_over_time -= period;
    }
}

pub fn c_enemies_extras(
    mut commands: Commands,
    new_ents: Query<(Entity, &Position), (With<Enemy>, Without<Transform>, Added<Replication>)>
) {
    for (entity, position) in &new_ents
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue };

        info!("Received new Enemy!");
        ent_coms.insert(EnemyExtrasBundle::new(position.0));
    }
}


