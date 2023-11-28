use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use crate::game::simple::{
    plugin::*,
    common::*,
    player::*
};

use super::abilities::ClassType;

#[derive(Event, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneralClientEvents
{
    ChangeClass(ClassType),
    SwapClass,
}

pub fn c_movement_input(mut move_events: EventWriter<MoveDirection>, input: Res<Input<KeyCode>>)
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

pub fn c_movement_predict(
    mut move_events: EventReader<MoveDirection>, 
    mut players: Query<&mut Position, With<LocalPlayer>>,
    time: Res<Time>
) {
    for dir in move_events.read()
    {
        for mut player_pos in &mut players
        {
            player_pos.0 += dir.0 * time.delta_seconds() * MOVE_SPEED;
        }
    }
}

pub fn c_class_change(
    mut general_client_events: EventWriter<GeneralClientEvents>,
    input: Res<Input<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Semicolon)
    {
        info!("Sending Swap Class Request");
        general_client_events.send(GeneralClientEvents::SwapClass);
    }
}
