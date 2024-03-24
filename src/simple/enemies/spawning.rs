use std::f32::consts::PI;

use bevy::prelude::*;
use bevy_rapier2d::prelude::*;
use bevy_replicon::prelude::Replication;
use rand::prelude::*;

use crate::simple::{
    behaviours::{collision::Damageable, damage::Damage, effect::{ActorChild, ActorContext, ActorSensors}}, 
    classes::class::ClassBaseData, 
    common::{Knockback, Position}, 
    consts::{CLIENT_STR, ENEMY_BASE_SPEED, ENEMY_COLLISION_FILTER, ENEMY_COLOR, ENEMY_GROUP, ENEMY_SENSOR_FILTER}, 
    visuals::healthbar::HealthBar
};

use super::{Enemy, WaveOverseer};

#[derive(Resource)]
pub struct EnemyData
{
    pub regular_enemy_data: Handle<ClassBaseData>,
}

/// This authority bundle acts as the replication bundle as well, simply due to the fact only the server ever spawns enemies
/// This means any clients will see only the replicated components
#[derive(Bundle)]
pub struct EnemyAuthorityBundle
{
    pub enemy: Enemy,
    pub actor: ActorContext,
    pub damage: Damageable,
    pub position: Position,
    pub replication: Replication,
    // ^ Replicated components
    // v Non replicated components
    pub sensors: ActorSensors,
    pub collider: Collider,
    pub velocity: Velocity, // Rapier velocity NOT super::common:Velocity
    pub knockback: Knockback,
    pub rigid_body: RigidBody,
    pub axis_lock: LockedAxes,
    group: CollisionGroups,
    collision_types: ActiveCollisionTypes,
    name: Name,
}

impl EnemyAuthorityBundle
{
    pub fn new(speed: f32, position: Vec2, actor: ActorContext, sensors: Vec<Entity>) -> Self
    {
        Self 
        {
            enemy: Enemy { speed },
            actor,
            damage: Damageable { invulnerability_duration: 0.25, invulnerability_remaining: 0.5 },
            position: Position(position),
            replication: Replication,
            sensors: ActorSensors { sensors },
            collider: Collider::ball(35.0 / 2.0),
            rigid_body: RigidBody::Dynamic,
            axis_lock: LockedAxes::ROTATION_LOCKED,
            velocity: Velocity::zero(),
            knockback: Knockback::default(),
            group: CollisionGroups { memberships: ENEMY_GROUP, filters: ENEMY_COLLISION_FILTER },
            collision_types: ActiveCollisionTypes::default(),
            name: Name::new("Regular Enemy"),
        }
    }
}

#[derive(Bundle)]
pub struct EnemyExtrasBundle
{
    pub sprite_bundle: SpriteBundle,
    pub healthbar: HealthBar,
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
            },
            healthbar: HealthBar::default(),
        }
    }
}

fn spawn_enemy(commands: &mut Commands, position: Vec2, actor: ActorContext)
{
    let sensor_id = commands.spawn((
        TransformBundle::IDENTITY,
        Sensor,
        Collider::ball(40.0 / 2.0),
        Damage { damage: 5.0, destroy_on_damage: false, deal_damage_once: false, knockback: Some(crate::simple::behaviours::damage::DamageKnockback::RepulsionFromSelf { strength: 300.0 }), did_damage: false },
        CollisionGroups { memberships: ENEMY_GROUP, filters: ENEMY_SENSOR_FILTER },
        ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC,
        Name::new("Enemy attack hitbox"),
    )).id();

    let enemy_root = commands.spawn(
        EnemyAuthorityBundle::new(ENEMY_BASE_SPEED, position, actor, vec![sensor_id])
    ).add_child(sensor_id).id();

    commands.entity(sensor_id).insert(ActorChild { ability_type: crate::simple::behaviours::effect::ChildType::Melee, parent_actor: enemy_root });
}


fn generate_enemy_position(distance: f32) -> Vec2
{
    let rotation_rand: f32 = random();

    let rotation_rads = rotation_rand * 2.0 * PI;

    Vec2::new(rotation_rads.cos() * distance, rotation_rads.sin() * distance)
}

fn vary_positions_about(pos: Vec2, count: u32) -> Vec<Vec2>
{
    let mut positions = Vec::new();

    for index in 0..count
    {
        let rotation = index as f32 * 0.2;
        let distance = index as f32 * 25.0;
        positions.push(pos + Vec2::new(distance * rotation.sin(), distance * rotation.cos()));
    }

    return positions;
}

pub fn s_tick_wave_overseer(
    mut commands: Commands,
    mut spawning: ResMut<WaveOverseer>,
    actor_data: Res<Assets<ClassBaseData>>,
    enemy_data: Res<EnemyData>,
    time: Res<Time>,
) {
    if spawning.point_rate <= 0.0 || !spawning.is_spawning
    {
        return;
    }

    spawning.points += spawning.point_rate * time.delta_seconds();

    while spawning.next_spawn.required_points() < spawning.points
    {
        const ENEMY_SPAWN_DISTANCE: f32 = 350.0;

        let position = generate_enemy_position(ENEMY_SPAWN_DISTANCE);

        info!("Spawning {} new enemies!", spawning.next_spawn.target_count);
        for pos in vary_positions_about(position, spawning.next_spawn.target_count)
        {
            let enemy_actor: ActorContext = actor_data.get(&enemy_data.regular_enemy_data).expect("did not find enemy base data").clone().into();
            spawn_enemy(&mut commands, pos, enemy_actor);
        }

        spawning.points -= spawning.next_spawn.required_points();
        spawning.tick_next_spawn();
    }
}

pub fn c_enemies_extras(
    mut commands: Commands,
    new_ents: Query<(Entity, &Position), (With<Enemy>, Added<Replication>)>
) {
    for (entity, position) in &new_ents
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue };

        debug!("{CLIENT_STR} Found new enemy");
        ent_coms.insert(EnemyExtrasBundle::new(position.0));
    }
}


