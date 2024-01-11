use bevy::{prelude::*, window::PrimaryWindow};

use bevy_replicon::{prelude::*, renet::ClientId};
use serde::{Serialize, Deserialize};

use crate::game::simple::{
    player::{Player, LocalPlayer, LocalPlayerId}, 
    common::{Position, Knockback}, 
    util::get_screenspace_cursor_pos_from_queries, 
    abilities::bullet::BulletReplicationBundle, 
    consts::{MELEE_DASH_SPEED, MELEE_DASH_DURATION, MELEE_ATTACK_LIFETIME}, behaviours::effect::Effect
};

use super::{tags::CanUseAbilities, melee::{MeleeReplicationBundle, MeleeAttackData, MeleeAttackType}};


#[derive(Event, Serialize, Deserialize, Debug)]
pub enum MeleeClassEvent
{
    NormalAttack{ dir: Vec2, prespawned: Option<Entity> },
    BigSwing{ dir: Vec2, prespawned: Option<Entity> },
    SlicingProjectile{ dir: Vec2, prespawned: Option<Entity> },
    SpinAttack{ prespawned: Option<Entity> },
    Dash{ dir: Vec2 },
}

pub fn s_melee_class_ability_response(
    mut commands: Commands,
    mut client_events: EventReader<FromClient<MeleeClassEvent>>,
    mut client_map: ResMut<ClientEntityMap>,
    mut players: Query<(&Player, &Position, &mut Knockback)>,
    tick: Res<RepliconTick>,
) {
    for FromClient { client_id, event } in client_events.read()
    {
        if *client_id == SERVER_ID
        {
            continue; // Skip Predicted events
        }
        info!("Received Event {event:?} from client '{client_id}'");
        match event
        {
            MeleeClassEvent::NormalAttack { dir, prespawned } => 
            {
                s_normal_attack_response(&mut commands, &mut client_map, &players, client_id.raw(), *dir, &prespawned, *tick);
            },
            MeleeClassEvent::BigSwing { dir, prespawned } => 
            {
                s_big_swing_response(&mut commands, &mut client_map, &players, client_id.raw(), *dir, &prespawned, *tick);
            },
            MeleeClassEvent::SlicingProjectile { dir, prespawned } => 
            {
                s_slicing_projectile_response(&mut commands, &mut client_map, &players, client_id.raw(), *dir, &prespawned, *tick);                
            },
            MeleeClassEvent::SpinAttack { prespawned } => 
            {
                s_spin_attack_response(&mut commands, &mut client_map, &players, client_id.raw(), &prespawned, *tick);
            },
            MeleeClassEvent::Dash { dir } =>
            {
                s_dash_response(*dir, &mut players, client_id.raw());
            }
        };
    }
}

fn s_normal_attack_response(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
    dir: Vec2,
    prespawned: &Option<Entity>,
    tick: RepliconTick,
) {
    for (player, position, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        let server_attack_entity = commands.spawn(
            MeleeReplicationBundle::new(MeleeAttackData 
                { 
                    owning_client: client_id, 
                    damage: 1.0, 
                    position: position.0, 
                    direction: dir, 
                    attack_type: MeleeAttackType::Stab { direction: dir, position: position.0, length: 15.0, width: 5.0 },
                })
        ).id();

        let Some(prespawned) = prespawned else { break; };
        client_map.insert(ClientId::from_raw(client_id), ClientMapping { tick, server_entity: server_attack_entity, client_entity: *prespawned });
        break;
    }
}

fn s_big_swing_response(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
    dir: Vec2,
    prespawned: &Option<Entity>,
    tick: RepliconTick,
) {
    for (player, position, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        let server_attack_entity = commands.spawn(
            MeleeReplicationBundle::new(MeleeAttackData 
                { 
                    owning_client: client_id, 
                    damage: 1.0, 
                    position: position.0, 
                    direction: dir, 
                    attack_type: MeleeAttackType::Stab { direction: dir, position: position.0, length: 15.0, width: 25.0 },
                })
        ).id();

        let Some(prespawned) = prespawned else { break; };
        client_map.insert(ClientId::from_raw(client_id), ClientMapping { tick, server_entity: server_attack_entity, client_entity: *prespawned });
        break;
    }
}

const BASE_SLICING_PROJECTILE: f32 = 125.0;

fn s_slicing_projectile_response(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
    dir: Vec2,
    prespawned: &Option<Entity>,
    tick: RepliconTick,
) {
    for (player, position, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        let server_attack_entity = commands.spawn(
            BulletReplicationBundle::new(
                position.0, 
                Color::rgb(0.5, 0.25, 0.65), 
                dir * BASE_SLICING_PROJECTILE, 
                5.0, 
                MELEE_ATTACK_LIFETIME,
                Effect::Nothing)
        ).id();

        let Some(prespawned) = prespawned else { break; };
        client_map.insert(ClientId::from_raw(client_id), ClientMapping { tick, server_entity: server_attack_entity, client_entity: *prespawned });
        break;
    }
}

fn s_spin_attack_response(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
    prespawned: &Option<Entity>,
    tick: RepliconTick,
) {
    for (player, position, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        let server_attack_entity = commands.spawn(
            MeleeReplicationBundle::new(MeleeAttackData 
                { 
                    owning_client: client_id, 
                    damage: 1.0, 
                    position: position.0, 
                    direction: Vec2::ZERO, 
                    attack_type: MeleeAttackType::Circular { position: position.0, radius: 50.0 },
                })
        ).id();

        let Some(prespawned) = prespawned else { break; };
        client_map.insert(ClientId::from_raw(client_id), ClientMapping { tick, server_entity: server_attack_entity, client_entity: *prespawned });
        break;
    }
}

pub fn s_dash_response(
    direction: Vec2,
    players: &mut Query<(&Player, &Position, &mut Knockback)>,
    client_id: u64,
) {
    for (player, _, mut knockback) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        *knockback = Knockback::new(direction * MELEE_DASH_SPEED, MELEE_DASH_DURATION, Knockback::DEFAULT_CONTROL_POINTS);
        break;
    }
}

// Client Abilities v

pub fn c_normal_attack(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    local_player: Res<LocalPlayerId>,
    mut ability_events: EventWriter<MeleeClassEvent>,
) {
    info!("Client: Doing normal melee attack...");
    let Ok(player_trans) = transform_q.get_single() else { return; };
    let player_pos = player_trans.translation().truncate();

    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return };

    let ability_direction = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::Y);

    let mut prespawned = None;
    if !local_player.is_host
    {
        let prespawned_entity = commands.spawn(
                MeleeReplicationBundle::new(MeleeAttackData
                {
                    owning_client: 0,
                    damage: 1.0,
                    position: player_pos,
                    direction: ability_direction,
                    attack_type: MeleeAttackType::Stab { direction: ability_direction, position: player_pos, length: 15.0, width: 5.0 },
                })
        ).id();

        prespawned = Some(prespawned_entity);
    }

    ability_events.send(MeleeClassEvent::NormalAttack { dir: ability_direction, prespawned });
}

pub fn c_big_swing(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    local_player: Res<LocalPlayerId>,
    mut ability_events: EventWriter<MeleeClassEvent>,
) {
    info!("Client: Doing big swing melee attack...");
    let Ok(player_trans) = transform_q.get_single() else { return; };
    let player_pos = player_trans.translation().truncate();

    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return };

    let ability_direction = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::Y);
    
    let mut prespawned = None;
    if !local_player.is_host
    {
        let prespawned_entity = commands.spawn(
                MeleeReplicationBundle::new(MeleeAttackData
                {
                    owning_client: 0,
                    damage: 1.0,
                    position: player_pos,
                    direction: ability_direction,
                    attack_type: MeleeAttackType::Stab { direction: ability_direction, position: player_pos, length: 15.0, width: 25.0 },
                })
        ).id();

        prespawned = Some(prespawned_entity);
    }

    ability_events.send(MeleeClassEvent::BigSwing { dir: ability_direction, prespawned });
}

pub fn c_dash(
    mut player_q: Query<(&GlobalTransform, &mut Knockback), (With<LocalPlayer>, With<CanUseAbilities>)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    local_player: Res<LocalPlayerId>,
    mut ability_events: EventWriter<MeleeClassEvent>,
) {
    info!("Client: Doing Melee Dash Attack");

    let Ok((player_trans, mut knockback)) = player_q.get_single_mut() else { return; };
    
    let player_pos = player_trans.translation().truncate();

    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return; };

    let dash_direction = (cursor_pos - player_pos).normalize_or_zero();

    if !local_player.is_host 
    {
        *knockback = Knockback::new(dash_direction * MELEE_DASH_SPEED, MELEE_DASH_DURATION, Knockback::DEFAULT_CONTROL_POINTS);
    }

    ability_events.send(MeleeClassEvent::Dash { dir: dash_direction });
}

pub fn c_slicing_projectile(
    mut commands: Commands, 
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    local_player: Res<LocalPlayerId>,
    mut ability_events: EventWriter<MeleeClassEvent>, 
) {
    info!("Client: Doing Slicing Projectile melee attack...");
    let Ok(player_trans) = transform_q.get_single() else { return; };
    let player_pos = player_trans.translation().truncate();

    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return; };

    let bullet_dir = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::new(1.0, 0.0));

    let mut prespawned = None;
    if !local_player.is_host
    {
        let bullet_entity = commands.spawn(
            BulletReplicationBundle::new(
                player_pos, 
                Color::rgb(0.5, 0.25, 0.65), 
                bullet_dir * BASE_SLICING_PROJECTILE, 
                5.0, 
                MELEE_ATTACK_LIFETIME,
                Effect::Nothing
            )
        ).id();
        info!("Client: Spawning Bullet Entity ({bullet_entity:?}) from Input");

        prespawned = Some(bullet_entity);
    }

    ability_events.send(MeleeClassEvent::SlicingProjectile { dir: bullet_dir, prespawned });
}

pub fn c_spin_attack(
    mut commands: Commands,
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    local_player: Res<LocalPlayerId>,
    mut ability_events: EventWriter<MeleeClassEvent>,
) {
    info!("Client: Doing Spin melee attack...");
    if local_player.is_host
    {
        ability_events.send(MeleeClassEvent::SpinAttack { prespawned: None });
    }

    let Ok(player_trans) = transform_q.get_single() else { return; };
    let player_pos = player_trans.translation().truncate();

    let prespawned_entity = commands.spawn(
            MeleeReplicationBundle::new(MeleeAttackData
            {
                owning_client: 0,
                damage: 1.0,
                position: player_pos,
                direction: Vec2::ZERO,
                attack_type: MeleeAttackType::Circular { position: player_pos, radius: 50.0 },
            })
    ).id();

    ability_events.send(MeleeClassEvent::SpinAttack { prespawned: Some(prespawned_entity) });
}

