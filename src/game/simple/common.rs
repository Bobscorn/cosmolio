use bevy::prelude::*;
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
