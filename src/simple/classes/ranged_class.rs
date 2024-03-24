use bevy::{prelude::*, window::PrimaryWindow};

use bevy_rapier2d::{plugin::RapierContext, pipeline::QueryFilter, geometry::CollisionGroups};
use bevy_replicon::{prelude::*, renet::ClientId};

use serde::{Deserialize, Serialize};

use crate::simple::{
    classes::bullet::BulletReplicationBundle, 
    behaviours::{
        laser::LaserReplicationBundle, 
        missile::{Missile, MissileReplicationBundle},
        damage::DamageKnockback,
    }, 
    common::{
        DestroyIfNoMatchWithin, 
        Knockback, 
        Position, 
        VelocityDamping
    }, 
    consts::*, 
    player::{LocalPlayer, LocalPlayerId, Player}, 
    util::{get_direction_to_cursor, get_screenspace_cursor_pos_from_queries}
};

use super::tags::CanUseAbilities;


#[derive(Component, Serialize, Deserialize)]
pub struct RangedClassData
{
    pub machine_gun_equipped: bool,
    pub fire_period_seconds: f32,
    pub last_fire_time_seconds: f32,
}

impl Default for RangedClassData
{
    fn default() -> Self {
        Self {
            machine_gun_equipped: false,
            fire_period_seconds: 0.1,
            last_fire_time_seconds: 0.0
        }
    }
}

pub fn s_ranged_class_setup(
    commands: &mut Commands,
    player_ent: Entity,
) {
    let Some(mut ent_coms) = commands.get_entity(player_ent) else { return; };

    ent_coms.insert(RangedClassData::default());
    debug!("{SERVER_STR} Setting up ranged class data");
}

pub fn s_ranged_class_teardown(
    commands: &mut Commands,
    player_ent: Entity
) {
    let Some(mut ent_coms) = commands.get_entity(player_ent) else { return; };

    ent_coms.remove::<RangedClassData>();
    debug!("{SERVER_STR} Tearing down ranged class data");
}

#[derive(Event, Serialize, Deserialize, Debug)]
pub enum RangedClassEvent
{
    BasicGunAttack{ dir: Vec2, prespawned: Option<Entity> },
    BasicGrenadeAttack{ dir: Vec2, prespawned: Option<Entity> },
    ShotgunBlast{ dir: Vec2, prespawned: Option<[Entity; 5]> },
    EquipMachineGun,
    MachineGunBullet{ dir: Vec2, prespawned: Option<Entity> },
    Boomerang{ dir: Vec2, prespawned: Option<Entity> },
    Missiles{ dir: Vec2, prespawned: Option<[Entity; 4]> },
}


pub fn s_ranged_class_response(
    mut commands: Commands,
    mut client_events: EventReader<FromClient<RangedClassEvent>>,
    mut client_map: ResMut<ClientEntityMap>,
    mut players: Query<(Entity, &Player, &Position, &mut Knockback, &mut RangedClassData)>,
    rapier_context: Res<RapierContext>,
    time: Res<Time>,
) {
    for FromClient { client_id, event } in client_events.read()
    {
        info!("{SERVER_STR} Received event {event:?} from client '{client_id}'");
        match event
        {
            RangedClassEvent::BasicGunAttack { dir, prespawned } =>
            {
                s_basic_gun_reponse(&mut commands, &mut client_map, &players, client_id.raw(), *dir, &prespawned);
            },
            RangedClassEvent::BasicGrenadeAttack { dir, prespawned } =>
            {
                s_basic_grenade_reponse(&mut commands, &mut client_map, &players, client_id.raw(), *dir, &prespawned);
            },
            RangedClassEvent::ShotgunBlast { dir, prespawned } =>
            {
                s_shotgun_reponse(&mut commands, &mut client_map, &mut players, &rapier_context, client_id.raw(), *dir, &prespawned);
            },
            RangedClassEvent::EquipMachineGun => 
            {
                s_equipmachine_gun(&mut players, client_id.raw());
            },
            RangedClassEvent::MachineGunBullet { dir, prespawned } =>
            {
                s_machine_gun_bullet(&mut commands, &mut client_map, &mut players, client_id.raw(), *dir, &prespawned, &time);
            },
            RangedClassEvent::Boomerang { dir, prespawned } =>
            {
                s_boomerang_reponse(&mut commands, &mut client_map, &players, client_id.raw(), *dir, &prespawned);
            },
            RangedClassEvent::Missiles { dir, prespawned } =>
            {
                s_missile_response(&mut commands, &mut client_map, &players, client_id.raw(), *dir, &prespawned);
            }
        }
    }
}

fn s_basic_gun_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(Entity, &Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
    dir: Vec2,
    prespawned: &Option<Entity>,
) {
    for (player_ent, player, position, _, _) in players
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
                RANGED_BULLET_LIFETIME,
                50.0,
                player_ent,
            )
        ).id();
        
        let Some(client_entity) = prespawned else { break; };
        client_map.insert(ClientId::from_raw(client_id), ClientMapping { server_entity, client_entity: *client_entity });
        break;
    }
}

fn s_basic_grenade_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(Entity, &Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
    dir: Vec2,
    prespawned: &Option<Entity>,
) {
    for (player_ent, player, position, _, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        info!("{SERVER_STR} Spawning Grenade on server");
        let server_entity = commands.spawn(
            (
                BulletReplicationBundle::new(
                    position.0,
                    RANGED_GRENADE_COLOR,
                    dir * RANGED_GRENADE_SPEED,
                    RANGED_GRENADE_SIZE,
                    RANGED_GRENADE_FUSE_TIME,
                    50.0,
                    player_ent,
                ),
                VelocityDamping(0.1)
            )
        ).id();

        let Some(client_entity) = prespawned else { break; };
        client_map.insert(ClientId::from_raw(client_id), ClientMapping{ server_entity, client_entity: *client_entity });
        break;
    }
}

fn s_shotgun_reponse(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &mut Query<(Entity, &Player, &Position, &mut Knockback, &mut RangedClassData)>,
    rapier_context: &Res<RapierContext>,
    client_id: u64,
    dir: Vec2,
    prespawned: &Option<[Entity; 5]>,
) {
    for (_, player, pos, mut knockback, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        let angles = [-22.5_f32.to_radians(), -11.25_f32.to_radians(), 0.0, 11.25_f32.to_radians(), 22.5_f32.to_radians()];
    
        let max_toi = RANGED_SHOTGUN_DISTANCE;
        let filter = QueryFilter::new()
            .groups(CollisionGroups { memberships: PLAYER_GROUP, filters: PLAYER_PROJECTILE_GROUP });
    
        for (index, angle) in angles.iter().enumerate()
        {
            let ray_dir = (Quat::from_rotation_z(*angle) * dir.extend(0.0)).truncate();
    
            let toi = rapier_context.cast_ray(pos.0, ray_dir, max_toi, true, filter).map(|(_, toi)| toi).unwrap_or(max_toi);
    
            let entity = commands.spawn(
                LaserReplicationBundle::new(Color::RED, toi, pos.0, ray_dir, 2.5, RANGED_SHOTGUN_KNOCKBACK, PLAYER_PROJECTILE_GROUPS)
            ).id();
            
            let Some(client_entities) = prespawned else { continue; };
            client_map.insert(ClientId::from_raw(client_id), ClientMapping { server_entity: entity, client_entity: client_entities[index] });
        }

        *knockback = Knockback::new(-dir * RANGED_SHOTGUN_SELF_KNOCKBACK_SPEED, RANGED_SHOTGUN_SELF_KNOCKBACK_DURATION, Knockback::DEFAULT_CONTROL_POINTS);
        break;
    }
}

fn s_equipmachine_gun(
    players: &mut Query<(Entity, &Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
) {
    for (_, player, _, _, mut class_data) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        info!("Equip machine gun {client_id}");
        class_data.machine_gun_equipped = !class_data.machine_gun_equipped;
        break;
    }
}

fn s_machine_gun_bullet(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &mut Query<(Entity, &Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
    dir: Vec2,
    prespawned: &Option<Entity>,
    time: &Res<Time>,
) {
    for (player_ent, player, pos, _, mut ranged_data) in players
    {
        if player.0 != client_id
        {
            continue;
        }
        if !ranged_data.machine_gun_equipped
        {
            return;
        }

        if (time.elapsed_seconds_wrapped() - ranged_data.last_fire_time_seconds) < ranged_data.fire_period_seconds
        {
            info!("{SERVER_STR}: Skipping Machine Gun event due to insufficient time passing: Elapsed {}s, Last fire: {}", time.elapsed_seconds_wrapped(), ranged_data.last_fire_time_seconds);
            return;
        }

        info!("{SERVER_STR} Shooting Machine Gun Bullets");
        // Wrap the last_fire_time_seconds again, this is to prevent the last_fire_time_seconds being >= (time.wrap_period - fire_period_seconds).
        // If last_fire_time_seconds did end up being that close to time.wrap_period, the player would never be able to shoot again
        ranged_data.last_fire_time_seconds = (time.elapsed_seconds_wrapped() + ranged_data.fire_period_seconds) % time.wrap_period().as_secs_f32() - ranged_data.fire_period_seconds;

        let entity = commands.spawn(
            BulletReplicationBundle::new(
                pos.0,
                RANGED_BULLET_COLOR,
                dir * RANGED_BULLET_SPEED, 
                RANGED_BULLET_SIZE, 
                RANGED_BULLET_LIFETIME,
                50.0,
                player_ent,
            )
        ).id();

        let Some(client_entity) = prespawned else { continue; };
        client_map.insert(ClientId::from_raw(client_id), ClientMapping { server_entity: entity, client_entity: *client_entity });
        break;
    }
}

fn s_boomerang_reponse(
    _commands: &mut Commands,
    _client_map: &mut ClientEntityMap,
    _players: &Query<(Entity, &Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
    _dir: Vec2,
    _prespawned: &Option<Entity>,
) {
    info!("{SERVER_STR} Unimplemented ability 'boomerang' triggered for client {client_id}");
    todo!();
}

fn s_missile_response(
    commands: &mut Commands,
    client_map: &mut ClientEntityMap,
    players: &Query<(Entity, &Player, &Position, &mut Knockback, &mut RangedClassData)>,
    client_id: u64,
    dir: Vec2,
    prespawned: &Option<[Entity; 4]>,
) {
    for (player_ent, player, pos, _, _) in players
    {
        if player.0 != client_id
        {
            continue;
        }

        let angles = [-110.0f32.to_radians(), -55.0f32.to_radians(), 55.0f32.to_radians(), 110.0f32.to_radians()];

        info!("{SERVER_STR} Spawning 4 missiles at {0} with owner: {1:?}", pos.0, player_ent);
        for (index, angle) in angles.iter().enumerate()
        {
            let missile_dir = (Quat::from_rotation_z(*angle) * dir.extend(0.0)).truncate();

            let server_entity = commands.spawn(
                MissileReplicationBundle::new(
                    Missile::from_owner(player_ent),
                    pos.0, 
                    missile_dir * RANGED_MISSILE_INITIAL_SPEED, 
                    RANGED_MISSILE_DAMAGE, 
                    PLAYER_PROJECTILE_GROUPS,
                    Some(DamageKnockback::RepulsionFromSelf { strength: 100.0 }),
                )).id();
            
            let Some(client_entities) = prespawned else { continue; };
            client_map.insert(ClientId::from_raw(client_id), ClientMapping { client_entity: client_entities[index], server_entity });
        }
        break;
    }
}

pub fn c_basic_gun_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    player_q: Query<(&GlobalTransform, &RangedClassData), (With<LocalPlayer>, With<CanUseAbilities>)>,
    local_player: Res<LocalPlayerId>,
    mut ability_events: EventWriter<RangedClassEvent>,
) {
    let Ok((player_trans, ranged_data)) = player_q.get_single() else { warn!("{CLIENT_STR} Could not find local player entity!"); return; };
    if ranged_data.machine_gun_equipped
    {
        debug!("{CLIENT_STR} Not shooting basic gun as machine gun is equipped");
        return;
    }
    info!("{CLIENT_STR} Doing basic gun attack...");
    let player_pos = player_trans.translation().truncate();
    
    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return };
    
    let ability_direction = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::Y);
    
    let mut prespawned = None;
    
    if local_player.should_predict()
    {
        let prespawned_entity = commands.spawn(
            BulletReplicationBundle::new(
                player_pos,
                RANGED_BULLET_COLOR,
                ability_direction * RANGED_BULLET_SPEED,
                RANGED_BULLET_SIZE,
                RANGED_BULLET_LIFETIME,
                50.0,
                local_player.entity,
            )
        ).id();
        
        prespawned = Some(prespawned_entity);
    }
    
    ability_events.send(RangedClassEvent::BasicGunAttack { dir: ability_direction, prespawned });
}

pub fn c_machine_gun_shoot_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut player_q: Query<(&GlobalTransform, &mut RangedClassData), (With<LocalPlayer>, With<CanUseAbilities>)>,
    local_player: Res<LocalPlayerId>,
    time: Res<Time>,
    mut ability_events: EventWriter<RangedClassEvent>,
) {
    let Ok((player_trans, mut ranged_data)) = player_q.get_single_mut() else { return; };
    if !ranged_data.machine_gun_equipped
    {
        return;
    }

    if (time.elapsed_seconds_wrapped() - ranged_data.last_fire_time_seconds) < ranged_data.fire_period_seconds
    {
        return;
    }
    info!("{CLIENT_STR} Firing machine gun");

    let player_pos = player_trans.translation().truncate();

    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return; };

    let ability_direction = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::Y);

    let mut prespawned = None;

    if local_player.should_predict()
    {
        let prespawned_entity = commands.spawn(
            BulletReplicationBundle::new(
                player_pos,
                RANGED_BULLET_COLOR,
                ability_direction * RANGED_BULLET_SPEED,
                RANGED_BULLET_SIZE,
                RANGED_BULLET_LIFETIME,
                50.0,
                local_player.entity,
            )
        ).id();

        prespawned = Some(prespawned_entity);

        ranged_data.last_fire_time_seconds = (time.elapsed_seconds_wrapped() + ranged_data.fire_period_seconds) % time.wrap_period().as_secs_f32() - ranged_data.fire_period_seconds;
    }    

    ability_events.send(RangedClassEvent::MachineGunBullet { dir: ability_direction, prespawned });
}

pub fn c_basic_grenade_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<&GlobalTransform, (With<LocalPlayer>, With<CanUseAbilities>)>,
    local_player: Res<LocalPlayerId>,
    mut ability_events: EventWriter<RangedClassEvent>,
) {
    info!("{CLIENT_STR} Doing basic grenade attack...");
    let Ok(player_trans) = transform_q.get_single() else { return; };
    let player_pos = player_trans.translation().truncate();

    let Some(cursor_pos) = get_screenspace_cursor_pos_from_queries(&window_q, &camera_q) else { return };

    let ability_direction = (cursor_pos - player_pos).try_normalize().unwrap_or(Vec2::Y);

    let mut prespawned = None;
    if local_player.should_predict()
    {
        let prespawned_entity = commands.spawn(
            (
                BulletReplicationBundle::new(
                    player_pos,
                    RANGED_GRENADE_COLOR,
                    ability_direction * RANGED_BULLET_SPEED,
                    RANGED_GRENADE_SIZE,
                    RANGED_GRENADE_FUSE_TIME,
                    50.0,
                    local_player.entity,
                ),
                VelocityDamping(0.1),
                DestroyIfNoMatchWithin::default(),
            )
        ).id();

        prespawned = Some(prespawned_entity);
    }

    ability_events.send(RangedClassEvent::BasicGrenadeAttack { dir: ability_direction, prespawned });
}

pub fn c_shotgun_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    mut transform_q: Query<(&GlobalTransform, &mut Knockback), (With<LocalPlayer>, With<CanUseAbilities>)>,
    mut ability_events: EventWriter<RangedClassEvent>,
    local_player: Res<LocalPlayerId>,
    rapier_context: Res<RapierContext>
) {
    let Ok((player_trans, _)) = transform_q.get_single_mut() else { return; };
    let player_pos = player_trans.translation().truncate();
    let Some(direction) = get_direction_to_cursor(&window_q, &camera_q, player_pos) else { return; };
    let direction = direction.normalize_or_zero();

    let mut prespawned = None;
    if local_player.should_predict()
    {
        let angles = [-22.5_f32.to_radians(), -11.25_f32.to_radians(), 0.0, 11.25_f32.to_radians(), 22.5_f32.to_radians()];
        let mut entities: [Entity; 5] = [Entity::PLACEHOLDER; 5];

        let max_toi = RANGED_SHOTGUN_DISTANCE;
        let filter = QueryFilter::new()
            .groups(CollisionGroups { memberships: PLAYER_GROUP, filters: PLAYER_PROJECTILE_GROUP });

        for (index, angle) in angles.iter().enumerate()
        {
            let ray_dir = (Quat::from_rotation_z(*angle) * direction.extend(0.0)).truncate();

            let toi = rapier_context.cast_ray(player_pos, ray_dir, max_toi, true, filter).map(|(_, toi)| toi).unwrap_or(max_toi);

            let entity = commands.spawn(
                LaserReplicationBundle::new(Color::RED, toi, player_pos, ray_dir, 2.5, RANGED_SHOTGUN_KNOCKBACK, PLAYER_PROJECTILE_GROUPS)
            ).id();

            entities[index] = entity;
        }

        prespawned = Some(entities);
    }

    ability_events.send(RangedClassEvent::ShotgunBlast { dir: direction, prespawned });
}

pub fn c_equipmachine_gun_ability(
    mut player_q: Query<&mut RangedClassData, With<LocalPlayer>>,
    local_player: Res<LocalPlayerId>,
    mut ability_events: EventWriter<RangedClassEvent>,
) {
    let Ok(mut data) = player_q.get_single_mut() else { return; };
        // TODO: Eventually display a sprite indicating the machine gun is equipped
    if !local_player.is_host
    {
        data.machine_gun_equipped = !data.machine_gun_equipped;
    }
    info!("{CLIENT_STR} Sending equip machine gun event");
    ability_events.send(RangedClassEvent::EquipMachineGun);
}

pub fn c_missile_ability(
    mut commands: Commands,
    window_q: Query<&Window, With<PrimaryWindow>>,
    camera_q: Query<(&Camera, &GlobalTransform)>,
    transform_q: Query<(&GlobalTransform, &Player), (With<LocalPlayer>, With<CanUseAbilities>)>,
    local_player: Res<LocalPlayerId>,
    mut ability_events: EventWriter<RangedClassEvent>
) {
    let (player_pos, _) = match transform_q.get_single() { Ok(p) => (p.0.translation().truncate(), p.1), Err(_) => return };

    let Some(ability_dir) = get_direction_to_cursor(&window_q, &camera_q, player_pos) else { return; };
    let ability_dir = ability_dir.try_normalize().unwrap_or(Vec2::Y);

    let prespawned: Option<[Entity; 4]>;
    if local_player.should_predict()
    {
        let mut entities = [Entity::PLACEHOLDER; 4];

        let angles = [-110.0f32.to_radians(), -55.0f32.to_radians(), 55.0f32.to_radians(), 110.0f32.to_radians()];

        info!("{CLIENT_STR} Spawning the 4 missiles!");
        for (index, angle) in angles.iter().enumerate()
        {
            let missile_dir = (Quat::from_rotation_z(*angle) * ability_dir.extend(0.0)).truncate();

            let entity = commands.spawn(
                MissileReplicationBundle::new(
                    Missile::from_owner(local_player.entity),
                    player_pos, 
                    missile_dir * RANGED_MISSILE_INITIAL_SPEED, 
                    RANGED_MISSILE_DAMAGE,
                    PLAYER_PROJECTILE_GROUPS,
                    Some(DamageKnockback::RepulsionFromSelf { strength: 100.0 }),
                )).id();

            entities[index] = entity;
        }

        prespawned = Some(entities);
    }
    else
    {
        prespawned = None;
    }

    ability_events.send(RangedClassEvent::Missiles { dir: ability_dir, prespawned });
}
