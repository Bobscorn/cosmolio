use bevy::prelude::*;
use serde::{Deserialize, Serialize};

#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub struct MoveDirection(pub Vec2);

pub fn c_movement_input(mut move_events: EventWriter<MoveDirection>, input: Res<ButtonInput<KeyCode>>)
{
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::KeyD)
    {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::KeyA)
    {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::KeyW)
    {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::KeyS)
    {
        direction.y -= 1.0;
    }
    if direction != Vec2::ZERO
    {
        move_events.send(MoveDirection(direction.normalize_or_zero()));
    }
}
