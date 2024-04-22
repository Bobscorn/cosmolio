use bevy::{prelude::*, window::PrimaryWindow};

use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simple::{
    gameplay::{Position, DestroyIfNoMatchWithin, objects::BulletReplicationBundle},
    player::{Player, LocalPlayer, LocalPlayerId}, 
    consts::{BASE_BULLET_SPEED, DEFAULT_CLASS_BULLET_LIFETIME}, 
    util::{get_screenspace_cursor_pos, get_screenspace_cursor_pos_from_queries}
};

use super::super::objects::{MeleeReplicationBundle, MeleeAttackData, MeleeAttackType};

#[derive(Event, Serialize, Deserialize)]
pub enum DefaultClassAbility
{
    ShootAbility{ dir: Vec2, color: Color, prespawned: Option<Entity> },
    MeleeAbility{ dir: Vec2, prespawned: Option<Entity> },
}

pub fn s_default_class_ability_response(
    mut commands: Commands,
    mut client_events: EventReader<FromClient<DefaultClassAbility>>,
    mut client_mapping: ResMut<ClientEntityMap>,
    players: Query<(Entity, &Player, &Position)>,
) {
    for FromClient { client_id, event } in client_events.read()
    {
        if *client_id == ClientId::SERVER
        {
            continue;
        }

        match event
        {
            DefaultClassAbility::ShootAbility { dir, color, prespawned } =>
            {
                s_shoot_ability(&mut commands, &mut client_mapping, &players, client_id, *dir, *color, &prespawned);
            },
            DefaultClassAbility::MeleeAbility { dir, prespawned } =>
            {
                s_melee_ability(&mut commands, &mut client_mapping, &players, client_id, *dir, &prespawned);
            }
        }
    }
}

fn s_shoot_ability(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(Entity, &Player, &Position)>,
    client_id: &ClientId,
    dir: Vec2,
    color: Color,
    prespawned: &Option<Entity>,
) {
    for (entity, player, pos) in players
    {
        if client_id != &player.0
        {
            continue;
        }

        let server_bullet = commands.spawn(BulletReplicationBundle::new(
            pos.0, 
            color, 
            dir * BASE_BULLET_SPEED, 
            5.0, 
            DEFAULT_CLASS_BULLET_LIFETIME,
            100.0,
            entity,
        )).id();

        info!("Server: Spawning ({server_bullet:?}) for client '{}'", client_id.get());
        let Some(prespawned) = prespawned else { break; };
        client_map.insert(*client_id, ClientMapping { server_entity: server_bullet, client_entity: *prespawned });
        break;
    }
}

fn s_melee_ability(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(Entity, &Player, &Position)>,
    client_id: &ClientId,
    dir: Vec2,
    prespawned: &Option<Entity>,
) {
    for (_, player, pos) in players
    {
        if client_id != &player.0
        {
            continue;
        }

        let server_entity = commands.spawn(MeleeReplicationBundle::new(MeleeAttackData 
            { 
                owning_client: *client_id, 
                damage: 0.5, 
                position: pos.0, 
                direction: dir, 
                attack_type: MeleeAttackType::Stab { direction: dir, position: pos.0, length: 15.0, width: 5.0 },
            })).id();

        info!("Server: Spawning ({server_entity:?}) for client '{}'", client_id.get());
        let Some(prespawned) = prespawned else { break; };
        client_map.insert(*client_id, ClientMapping { server_entity, client_entity: *prespawned });
        break;
    }
}


pub trait GetColor
{
    fn get_color() -> Color;
}

pub fn c_shoot_ability<T: GetColor>(
    transform_query: Query<&Transform, With<LocalPlayer>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ability_events: EventWriter<DefaultClassAbility>, 
    mut commands: Commands, 
    player: Res<LocalPlayerId>
) {
    let Ok(window) = window_q.get_single() else { return };
    let Ok((camera, camera_trans)) = camera_q.get_single() else { return };
    let Some(cursor_pos) = get_screenspace_cursor_pos(window, camera, camera_trans) else { return };

    let Ok(player_trans) = transform_query.get(player.entity) else { return };
    let player_pos = player_trans.translation.truncate();

    let bullet_dir = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::new(1.0, 0.0));

    let mut prespawned = None;
    if !player.is_host
    {
        let bullet_entity = commands.spawn((
            BulletReplicationBundle::new(
                player_pos, 
                T::get_color(), 
                bullet_dir * BASE_BULLET_SPEED, 
                5.0, 
                DEFAULT_CLASS_BULLET_LIFETIME,
                100.0,
                player.entity,
            ), 
            DestroyIfNoMatchWithin{ remaining_time: 0.2 }
        )).id();
        info!("Client: Spawning Bullet Entity ({bullet_entity:?}) from Input");
        prespawned = Some(bullet_entity);
    }

    ability_events.send(DefaultClassAbility::ShootAbility{ dir: bullet_dir, color: T::get_color(), prespawned });
}

pub fn c_melee_ability(
    mut commands: Commands,
    transform_q: Query<&Transform, With<LocalPlayer>>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut ability_events: EventWriter<DefaultClassAbility>,
    local_player: Res<LocalPlayerId>,
) {
    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return };

    let Ok(player_trans) = transform_q.get_single() else { return };
    let player_pos = player_trans.translation.truncate();

    let melee_dir = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::new(1.0, 0.0));

    let mut prespawned = None;
    if !local_player.is_host
    {
        let melee_entity = commands.spawn(
            (
                MeleeReplicationBundle::new(MeleeAttackData
                    { 
                        owning_client: ClientId::new(local_player.id), 
                        damage: 0.5,
                        position: player_pos, 
                        direction: melee_dir,
                        attack_type: MeleeAttackType::Stab { direction: melee_dir, position: player_pos, length: 15.0, width: 5.0 },
                    }),
                DestroyIfNoMatchWithin{ remaining_time: 0.2 }
            )
        ).id();
        info!("Client: Spawning Melee Attack Entity ({melee_entity:?}) from Input");

        prespawned = Some(melee_entity);
    }

    ability_events.send(DefaultClassAbility::MeleeAbility { dir: melee_dir, prespawned });
}
