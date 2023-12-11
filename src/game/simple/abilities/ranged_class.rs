use bevy::{prelude::*, window::PrimaryWindow};

use bevy_replicon::prelude::*;

use serde::{Deserialize, Serialize};

use crate::game::simple::{player::{Player, LocalPlayer}, common::{Position, Knockback, DestroyIfNoMatchWithin, VelocityDamping}, abilities::bullet::BulletReplicationBundle, util::get_screenspace_cursor_pos_from_queries, consts::RANGED_BULLET_SPEED};

use super::tags::CanUseAbilities;


#[derive(Event, Serialize, Deserialize, Debug)]
pub enum RangedClassEvent
{
    BasicGunAttack{ dir: Vec2, prespawned: Entity },
    BasicGrenadeAttack{ dir: Vec2, prespawned: Entity },
    ShotgunBlast{ dir: Vec2 },
    EquipMachineGun,
    Boomerang{ dir: Vec2, prespawned: Entity },
    Missiles{ dir: Vec2 },
}


pub fn s_ranged_class_response(
    mut commands: Commands,
    mut client_events: EventReader<FromClient<RangedClassEvent>>,
    mut client_map: ResMut<ClientEntityMap>,
    mut players: Query<(&Player, &Position, &mut Knockback)>,
    tick: Res<RepliconTick>,
) {
    for FromClient { client_id, event } in client_events.read()
    {
        if *client_id == SERVER_ID
        {
            continue; // Skip events already predicted on the server's 'client'
        }
        info!("Received event {event:?} from client '{client_id}'");
        match event
        {
            RangedClassEvent::BasicGunAttack { dir, prespawned } =>
            {
                s_basic_gun_reponse(&mut commands, &mut client_map, &players, client_id.raw(), *dir, *prespawned, *tick);
            },
            RangedClassEvent::BasicGrenadeAttack { dir, prespawned } =>
            {
                s_basic_grenade_reponse(&mut commands, &mut client_map, &players, client_id.raw(), *dir, *prespawned, *tick);
            },
            RangedClassEvent::ShotgunBlast { dir } =>
            {
                s_shotgun_reponse(&mut commands, &mut client_map, &players, client_id.raw(), *dir, *tick);
            },
            RangedClassEvent::EquipMachineGun => 
            {
                s_equipmachine_gun();
            },
            RangedClassEvent::Boomerang { dir, prespawned } =>
            {
                s_boomerang_reponse(&mut commands, &mut client_map, &players, client_id.raw(), *dir, *prespawned, *tick);
            },
            RangedClassEvent::Missiles { dir } =>
            {
                s_missile_response(&mut commands, &players, client_id.raw(), *dir, *tick);
            }
        }
    }
}

fn s_basic_gun_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
    dir: Vec2,
    prespawned: Entity,
    tick: RepliconTick,
) {

}

fn s_basic_grenade_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
    dir: Vec2,
    prespawned: Entity,
    tick: RepliconTick,
) {

}

fn s_shotgun_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
    dir: Vec2,
    tick: RepliconTick,
) {

}

fn s_equipmachine_gun(

) {

}

fn s_boomerang_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
    dir: Vec2,
    prespawned: Entity,
    tick: RepliconTick,
) {

}

fn s_missile_response(
    commands: &mut Commands,
    players: &Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
    dir: Vec2,
    tick: RepliconTick,
) {

}

pub fn c_basic_gun_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    mut ability_events: EventWriter<RangedClassEvent>,
) {
    info!("Client: Doing basic gun attack...");
    let Ok(player_trans) = transform_q.get_single() else { return; };
    let player_pos = player_trans.translation().truncate();

    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return };

    let ability_direction = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::Y);

    let prespawned_entity = commands.spawn(
        (
            BulletReplicationBundle::new(
                player_pos,
                Color::rgb(0.15, 0.5, 0.69),
                ability_direction * RANGED_BULLET_SPEED,
                7.5
            ),
            DestroyIfNoMatchWithin::default(),
        )
    ).id();

    ability_events.send(RangedClassEvent::BasicGunAttack { dir: ability_direction, prespawned: prespawned_entity });
}

pub fn c_basic_grenade_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    mut ability_events: EventWriter<RangedClassEvent>,
) {
    info!("Client: Doing basic grenade attack...");
    let Ok(player_trans) = transform_q.get_single() else { return; };
    let player_pos = player_trans.translation().truncate();

    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return };

    let ability_direction = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::Y);

    let prespawned_entity = commands.spawn(
        (
            BulletReplicationBundle::new(
                player_pos,
                Color::rgb(0.15, 0.5, 0.69),
                ability_direction * RANGED_BULLET_SPEED,
                7.5
            ),
            VelocityDamping(0.1),
            DestroyIfNoMatchWithin::default(),
        )
    ).id();

    ability_events.send(RangedClassEvent::BasicGrenadeAttack { dir: ability_direction, prespawned: prespawned_entity });
}

pub fn c_shotgun_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    mut ability_events: EventWriter<RangedClassEvent>
) {

}

pub fn c_equipmachine_gun_ability(

) {
 todo!()
}

pub fn c_missile_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    mut ability_events: EventWriter<RangedClassEvent>
) {
    
}
