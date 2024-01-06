use bevy::{prelude::*, window::PrimaryWindow};

use bevy_rapier2d::{plugin::RapierContext, pipeline::QueryFilter, geometry::CollisionGroups};
use bevy_replicon::{prelude::*, renet::ClientId};

use serde::{Deserialize, Serialize};

use crate::game::simple::{
    player::{Player, LocalPlayer},
    common::{Position, Knockback, DestroyIfNoMatchWithin, VelocityDamping}, 
    abilities::bullet::BulletReplicationBundle, 
    util::{get_screenspace_cursor_pos_from_queries, get_direction_to_cursor}, 
    consts::*, 
    behaviours::{laser::{LaserAuthorityBundle, LaserReplicationBundle}, missile::{MissileReplicationBundle, Missile}, effect::{Effect, SpawnType, Owner}}
};

use super::tags::CanUseAbilities;


#[derive(Component, Serialize, Deserialize)]
pub struct RangedClassData
{
    pub machine_gun_equipped: bool,
}

pub fn s_ranged_class_setup(
    commands: &mut Commands,
    player_ent: Entity
) {
    let Some(mut ent_coms) = commands.get_entity(player_ent) else { return; };

    ent_coms.insert(RangedClassData { machine_gun_equipped: false });
    info!("Setting up ranged class data");
}

pub fn s_ranged_class_teardown(
    commands: &mut Commands,
    player_ent: Entity
) {
    let Some(mut ent_coms) = commands.get_entity(player_ent) else { return; };

    ent_coms.remove::<RangedClassData>();
    info!("Tearing down ranged class data");
}

#[derive(Event, Serialize, Deserialize, Debug)]
pub enum RangedClassEvent
{
    BasicGunAttack{ dir: Vec2, prespawned: Entity },
    BasicGrenadeAttack{ dir: Vec2, prespawned: Entity },
    ShotgunBlast{ dir: Vec2, prespawned: [Entity; 5] },
    EquipMachineGun,
    Boomerang{ dir: Vec2, prespawned: Entity },
    Missiles{ dir: Vec2, prespawned: [Entity; 4] },
}


pub fn s_ranged_class_response(
    mut commands: Commands,
    mut client_events: EventReader<FromClient<RangedClassEvent>>,
    mut client_map: ResMut<ClientEntityMap>,
    mut players: Query<(&Player, &Position, &mut Knockback, &mut RangedClassData)>,
    rapier_context: Res<RapierContext>,
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
            RangedClassEvent::ShotgunBlast { dir, prespawned } =>
            {
                s_shotgun_reponse(&mut commands, &mut client_map, &mut players, &rapier_context, client_id.raw(), *dir, *prespawned, *tick);
            },
            RangedClassEvent::EquipMachineGun => 
            {
                s_equipmachine_gun(&mut players, client_id.raw());
            },
            RangedClassEvent::Boomerang { dir, prespawned } =>
            {
                s_boomerang_reponse(&mut commands, &mut client_map, &players, client_id.raw(), *dir, *prespawned, *tick);
            },
            RangedClassEvent::Missiles { dir, prespawned } =>
            {
                s_missile_response(&mut commands, &mut client_map, &players, client_id.raw(), *dir, *prespawned, *tick);
            }
        }
    }
}

fn s_basic_gun_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
    dir: Vec2,
    prespawned: Entity,
    tick: RepliconTick,
) {
    for (player, position, _, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        info!("Spawning bullet on the server");
        let server_entity = commands.spawn(
            BulletReplicationBundle::new(
                position.0, 
                RANGED_BULLET_COLOR,
                dir * RANGED_BULLET_SPEED,
                RANGED_BULLET_SIZE,
                Effect::Nothing
            )
        ).id();

        client_map.insert(ClientId::from_raw(client_id), ClientMapping { tick: tick, server_entity, client_entity: prespawned });
        break;
    }
}

fn s_basic_grenade_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
    dir: Vec2,
    prespawned: Entity,
    tick: RepliconTick,
) {
    for (player, position, _, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        info!("Spawning Grenade on server");
        let server_entity = commands.spawn(
            (
                BulletReplicationBundle::new(
                    position.0,
                    RANGED_GRENADE_COLOR,
                    dir * RANGED_GRENADE_SPEED,
                    RANGED_GRENADE_SIZE,
                    Effect::SpawnEntity(SpawnType::Explosion { radius: 50.0, damage: 5.0, owner: Owner::Player { id: client_id } })
                ),
                VelocityDamping(0.1)
            )
        ).id();

        client_map.insert(ClientId::from_raw(client_id), ClientMapping{ tick, server_entity, client_entity: prespawned });
        break;
    }
}

fn s_shotgun_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &mut Query<(&Player, &Position, &mut Knockback, &mut RangedClassData)>,
    rapier_context: &Res<RapierContext>,
    client_id: u64,
    dir: Vec2,
    prespawned: [Entity; 5],
    tick: RepliconTick,
) {
    for (player, pos, mut knockback, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        let angles = [-22.5_f32.to_radians(), -11.25_f32.to_radians(), 0.0, 11.25_f32.to_radians(), 22.5_f32.to_radians()];
    
        let max_toi = RANGED_SHOTGUN_DISTANCE;
        let filter = QueryFilter::new()
            .groups(CollisionGroups { memberships: PLAYER_MEMBER_GROUP, filters: PLAYER_PROJECTILE_GROUP });
    
        for (index, angle) in angles.iter().enumerate()
        {
            let ray_dir = (Quat::from_rotation_z(*angle) * dir.extend(0.0)).truncate();
    
            let toi = rapier_context.cast_ray(pos.0, ray_dir, max_toi, true, filter).map(|(_, toi)| toi).unwrap_or(max_toi);
    
            let entity = commands.spawn(
                LaserReplicationBundle::new(Color::RED, toi, pos.0, ray_dir, 2.5, RANGED_SHOTGUN_KNOCKBACK, PLAYER_PROJECTILE_GROUPS)
            ).id();
    
            client_map.insert(ClientId::from_raw(client_id), ClientMapping { tick, server_entity: entity, client_entity: prespawned[index] });
        }

        *knockback = Knockback::new(-dir * RANGED_SHOTGUN_SELF_KNOCKBACK_SPEED, RANGED_SHOTGUN_SELF_KNOCKBACK_DURATION, Knockback::DEFAULT_CONTROL_POINTS);
        break;
    }
}

fn s_equipmachine_gun(
    players: &mut Query<(&Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
) {
    for (player, _, _, mut class_data) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        class_data.machine_gun_equipped = !class_data.machine_gun_equipped;
        break;
    }
}

fn s_boomerang_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
    dir: Vec2,
    prespawned: Entity,
    tick: RepliconTick,
) {
    info!("Unimplemented ability 'boomerang' triggered for client {client_id}");
}

fn s_missile_response(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(&Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
    dir: Vec2,
    prespawned: [Entity; 4],
    tick: RepliconTick,
) {
    for (player, pos, _, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        let angles = [-110.0f32.to_radians(), -55.0f32.to_radians(), 55.0f32.to_radians(), 110.0f32.to_radians()];

        info!("Spawning 4 missiles at {0}", pos.0);
        for (index, angle) in angles.iter().enumerate()
        {
            let missile_dir = (Quat::from_rotation_z(*angle) * dir.extend(0.0)).truncate();

            let server_entity = commands.spawn(
                MissileReplicationBundle::new(
                    Missile::default(),
                    pos.0, 
                    missile_dir * RANGED_MISSILE_INITIAL_SPEED, 
                    5.0, 
                    PLAYER_PROJECTILE_GROUPS
                )).id();

            client_map.insert(ClientId::from_raw(client_id), ClientMapping { client_entity: prespawned[index], server_entity, tick });
        }
        break;
    }
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
            BulletReplicationBundle::new(
                player_pos,
                RANGED_BULLET_COLOR,
                ability_direction * RANGED_BULLET_SPEED,
                RANGED_BULLET_SIZE,
                Effect::Nothing
            )
    ).id();

    ability_events.send(RangedClassEvent::BasicGunAttack { dir: ability_direction, prespawned: prespawned_entity });
}

pub fn c_basic_grenade_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<(&Player, &GlobalTransform), (With<LocalPlayer>, With<CanUseAbilities>)>,
    mut ability_events: EventWriter<RangedClassEvent>,
) {
    info!("Client: Doing basic grenade attack...");
    let Ok((player, player_trans)) = transform_q.get_single() else { return; };
    let player_pos = player_trans.translation().truncate();

    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return };

    let ability_direction = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::Y);

    let prespawned_entity = commands.spawn(
        (
            BulletReplicationBundle::new(
                player_pos,
                RANGED_GRENADE_COLOR,
                ability_direction * RANGED_BULLET_SPEED,
                RANGED_GRENADE_SIZE,
                Effect::SpawnEntity(SpawnType::Explosion { radius: 50.0, damage: 5.0, owner: Owner::Player { id: player.0 } })
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
    mut transform_q: Query<(&GlobalTransform, &mut Knockback), (With<LocalPlayer>, With<CanUseAbilities>)>,
    mut ability_events: EventWriter<RangedClassEvent>,
    rapier_context: Res<RapierContext>
) {
    let Ok((player_trans, mut knockback)) = transform_q.get_single_mut() else { return; };
    let player_pos = player_trans.translation().truncate();
    let Some(direction) = get_direction_to_cursor(&window_q, &camera_q, player_pos) else { return; };
    let direction = direction.normalize_or_zero();

    let angles = [-22.5_f32.to_radians(), -11.25_f32.to_radians(), 0.0, 11.25_f32.to_radians(), 22.5_f32.to_radians()];
    let mut entities: [Entity; 5] = [Entity::PLACEHOLDER; 5];

    let max_toi = RANGED_SHOTGUN_DISTANCE;
    let filter = QueryFilter::new()
        .groups(CollisionGroups { memberships: PLAYER_MEMBER_GROUP, filters: PLAYER_PROJECTILE_GROUP });

    for (index, angle) in angles.iter().enumerate()
    {
        let ray_dir = (Quat::from_rotation_z(*angle) * direction.extend(0.0)).truncate();

        let toi = rapier_context.cast_ray(player_pos, ray_dir, max_toi, true, filter).map(|(_, toi)| toi).unwrap_or(max_toi);

        let entity = commands.spawn(
            LaserReplicationBundle::new(Color::RED, toi, player_pos, ray_dir, 2.5, RANGED_SHOTGUN_KNOCKBACK, PLAYER_PROJECTILE_GROUPS)
        ).id();

        entities[index] = entity;
    }

    ability_events.send(RangedClassEvent::ShotgunBlast { dir: direction, prespawned: entities });
}

pub fn c_equipmachine_gun_ability(
    mut player_q: Query<&mut RangedClassData, With<LocalPlayer>>,
) {
    for mut data in &mut player_q
    {   
        data.machine_gun_equipped = !data.machine_gun_equipped;
    }
}

pub fn c_missile_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    mut ability_events: EventWriter<RangedClassEvent>
) {
    let player_pos = match transform_q.get_single() { Ok(p) => p, Err(_) => return }.translation().truncate();

    let Some(ability_dir) = get_direction_to_cursor(&window_q, &camera_q, player_pos) else { return; };
    let ability_dir = ability_dir.try_normalize().unwrap_or(Vec2::Y);

    let mut entities = [Entity::PLACEHOLDER; 4];

    let angles = [-110.0f32.to_radians(), -55.0f32.to_radians(), 55.0f32.to_radians(), 110.0f32.to_radians()];

    info!("Spawning the 4 missiles!");
    for (index, angle) in angles.iter().enumerate()
    {
        let missile_dir = (Quat::from_rotation_z(*angle) * ability_dir.extend(0.0)).truncate();

        let entity = commands.spawn(
            MissileReplicationBundle::new(
                Missile::default(),
                player_pos, 
                missile_dir * RANGED_MISSILE_INITIAL_SPEED, 
                5.0, 
                PLAYER_PROJECTILE_GROUPS
            )).id();

        entities[index] = entity;
    }

    ability_events.send(RangedClassEvent::Missiles { dir: ability_dir, prespawned: entities });
}
