use bevy::prelude::*;
use bevy_replicon::prelude::{MapNetworkEntities, ServerEntityMap};
use serde::{Deserialize, Serialize};


#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct Position(pub Vec2);

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Default, Debug, Serialize, Deserialize, DerefMut, Deref)]
pub struct Health(pub f32);

#[derive(Component, Serialize, Deserialize)]
pub struct Dead;

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Lifetime(pub f32);

#[derive(Event, Deserialize, Serialize)]
pub struct DestroyEntity(pub Entity);

/// Use this component to mark an entity as 'waiting for a server mapping'.
/// Any entity with this component that has no server mapping once it's lifetime has expired will be deleted
/// This component will be removed if a mapping is found
#[derive(Component)]
pub struct DestroyIfNoMatchWithin
{
    pub remaining_time: f32,
}

impl MapNetworkEntities for DestroyEntity
{
    fn map_entities<T: bevy_replicon::prelude::Mapper>(&mut self, mapper: &mut T) {
        self.0 = mapper.map(self.0);
    }
}

pub fn kill_zero_healths(
    mut commands: Commands,
    health_havers: Query<(Entity, &Health), Without<Dead>>
) {
    for (entity, health) in &health_havers
    {
        if health.0 > 0.0
        {
            continue;
        }

        let Some(mut ent_coms) = commands.get_entity(entity) else { continue };

        ent_coms.insert(Dead);
    }
}

/// This systems monitors entities with the [`DestroyIfNoMatch`] component and destroys them if no match is found before they expire
pub fn destroy_entites_without_match(
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
            info!("Client: Entity found match");
            commands.entity(entity).remove::<DestroyIfNoMatchWithin>();
            return;
        }

        if lifetime.remaining_time <= 0.0
        {
            info!("Client: Destroyed Entity with no match");
            commands.entity(entity).despawn_recursive();
        }
    }
}

pub fn update_and_destroy_lifetimes(
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

        commands.entity(entity).despawn_recursive();
    }
}
