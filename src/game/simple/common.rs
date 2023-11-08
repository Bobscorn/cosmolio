use bevy::prelude::*;
use serde::{Deserialize, Serialize};


#[derive(Resource)]
pub struct LocalPlayerId
{
    pub id: u64,
    pub entity: Entity
}



#[derive(Component, Serialize, Deserialize)]
pub struct Player(pub u64);

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct Position(pub Vec2);

#[derive(Component, Deserialize, Serialize, Deref, DerefMut)]
pub struct Velocity(pub Vec2);

#[derive(Component, Deserialize, Serialize)]
pub struct PlayerColor(pub Color);

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(pub Vec2);

#[derive(Component, Default, Debug, Serialize, Deserialize, DerefMut, Deref)]
pub struct Health(pub f32);

#[derive(Component, Debug, Deref, DerefMut)]
pub struct Lifetime(pub f32);
