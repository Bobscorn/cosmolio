use bevy::{prelude::*, window::PrimaryWindow};
use bevy_rapier2d::prelude::{Collider, Group, CollisionGroups, ActiveCollisionTypes};
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::game::simple::{util::get_screenspace_cursor_pos, player::{Player, LocalPlayer, LocalPlayerId}, projectile::{Projectile, ProjectileDamage}, consts::{PLAYER_PROJECTILE_GROUP, ENEMY_MEMBER_GROUP}};

use super::AbilityActivation;
use crate::game::simple::common::*;


#[derive(Component, Deserialize, Serialize)]
pub struct Bullet
{
    pub size: Vec2
}

#[derive(Bundle)]
pub struct BulletAuthorityBundle
{
    pub bullet: Bullet,
    pub position: Position,
    pub velocity: Velocity,
    pub sprite_bundle: SpriteBundle,
    pub rep: Replication,
    proj: Projectile,
    pub damage: ProjectileDamage,
    pub collider: Collider,
    group: CollisionGroups,
    collision_types: ActiveCollisionTypes
}

impl BulletAuthorityBundle
{
    pub fn new(pos: Vec2, velocity: Vec2, size: Vec2) -> Self
    {
        Self 
        { 
            bullet: Bullet { size }, 
            position: Position(pos),
            velocity: Velocity(velocity),
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { color: Color::rgb(0.5, 0.25, 0.15), custom_size: Some(size), ..default() }, 
                transform: Transform::from_translation(pos.extend(0.0)), 
                ..default() 
            },
            rep: Replication,
            proj: Projectile,
            damage: ProjectileDamage(1.0),
            collider: Collider::ball(size.x / 2.0),
            group: CollisionGroups { memberships: PLAYER_PROJECTILE_GROUP, filters: ENEMY_MEMBER_GROUP },
            collision_types: ActiveCollisionTypes::default() | ActiveCollisionTypes::STATIC_STATIC
        }
    }
}

#[derive(Bundle)]
pub struct BulletReceiveBundle
{
    pub sprite_bundle: SpriteBundle
}

impl BulletReceiveBundle
{
    pub fn new(pos: Vec2, size: Vec2) -> Self
    {
        Self { 
            sprite_bundle: SpriteBundle { 
                sprite: Sprite { color: Color::rgb(0.5, 0.25, 0.15), custom_size: Some(size), ..default() }, 
                transform: Transform::from_translation(pos.extend(0.0)), 
                ..default() 
            }
        }
    }
}


#[derive(Component, Serialize, Deserialize)]
pub struct CanShootBullet;


const BULLET_SPEED: f32 = 25.0;

pub fn ability_input_system(
    transform_query: Query<&Transform, (With<LocalPlayer>, With<CanShootBullet>)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ability_events: EventWriter<AbilityActivation>, 
    mut commands: Commands, 
    input: Res<Input<KeyCode>>,
    player: Res<LocalPlayerId>
) {
    if !input.just_pressed(KeyCode::Space)
    {
        return;
    }
    let Ok(window) = window_q.get_single() else { return };
    let Ok((camera, camera_trans)) = camera_q.get_single() else { return };
    let Some(cursor_pos) = get_screenspace_cursor_pos(window, camera, camera_trans) else { return };

    let Ok(player_trans) = transform_query.get(player.entity) else { return };
    let player_pos = player_trans.translation.truncate();

    let bullet_dir = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::new(1.0, 0.0));

    info!("Pre-spawning bullet for client");
    let bullet_entity = commands.spawn(BulletAuthorityBundle::new(player_pos, bullet_dir * BULLET_SPEED, Vec2::new(5.0, 5.0))).id();
    ability_events.send(AbilityActivation::ShootBullet(bullet_dir, bullet_entity));
}



pub fn client_bullet_receive_system(
    mut commands: Commands,
    received_bullets: Query<(Entity, &Bullet, &Position), (Without<Transform>, Added<Replication>)>,
) {
    for (entity, bullet, pos) in &received_bullets
    {
        let Some(mut ent_coms) = commands.get_entity(entity) else { continue };

        ent_coms.insert(BulletReceiveBundle::new(pos.0, bullet.size));
    }
}



pub fn server_ability_response(
    mut ability_events: EventReader<FromClient<AbilityActivation>>,
    players: Query<(&Player, &Position)>,
    mut commands: Commands,
    mut client_map: ResMut<ClientEntityMap>,
    tick: Res<RepliconTick>
) {
    for FromClient { client_id, event } in &mut ability_events
    {
        if *client_id == SERVER_ID
        {
            // Skip events from the "server's client" 
            // This assumes the client predicts what the server will do, meaning if the server did it, it would happen twice for the local player
            continue;
        }
        info!("Client '{client_id}' sent event {event:?}");
        match event
        {
            AbilityActivation::None => { info!("Client '{client_id}' send empty ability event") },
            AbilityActivation::ShootBullet(dir, client_bullet) => 
            {
                for (player, pos) in &players
                {
                    if *client_id != player.0
                    {
                        continue;
                    }

                    let server_bullet = commands.spawn(BulletAuthorityBundle::new(pos.0, *dir * BULLET_SPEED, Vec2::new(5.0, 5.0))).id();

                    info!("Spawning server bullet for client '{client_id}'");
                    client_map.insert(*client_id, ClientMapping { tick: *tick, server_entity: server_bullet, client_entity: *client_bullet });
                }
            }
        }
    }
}
