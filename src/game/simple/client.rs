use bevy::prelude::*;

use crate::game::simple::{
    plugin::*,
    common::*,
    player::*
};


pub fn movement_input_system(mut move_events: EventWriter<MoveDirection>, input: Res<Input<KeyCode>>)
{
    let mut direction = Vec2::ZERO;
    if input.pressed(KeyCode::D)
    {
        direction.x += 1.0;
    }
    if input.pressed(KeyCode::A)
    {
        direction.x -= 1.0;
    }
    if input.pressed(KeyCode::W)
    {
        direction.y += 1.0;
    }
    if input.pressed(KeyCode::S)
    {
        direction.y -= 1.0;
    }
    if direction != Vec2::ZERO
    {
        move_events.send(MoveDirection(direction.normalize_or_zero()));
    }
}

pub fn client_movement_predict(
    mut move_events: EventReader<MoveDirection>, 
    mut players: Query<&mut Position, With<LocalPlayer>>,
    time: Res<Time>
) {
    for dir in &mut move_events
    {
        for mut player_pos in &mut players
        {
            player_pos.0 += dir.0 * time.delta_seconds() * MOVE_SPEED;
        }
    }
}
