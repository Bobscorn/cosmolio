use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use crate::simple::{
    plugin::*,
    common::*,
    player::*
};

use super::classes::class::{ClassType, ActorClass};

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
    mut players: Query<(&mut Position, &Knockback), With<LocalPlayer>>,
    time: Res<Time>
) {
    for dir in move_events.read()
    {
        for (mut player_pos, knockback) in &mut players
        {
            if knockback.has_knockback()
            {
                continue;
            }
            player_pos.0 += dir.0 * time.delta_seconds() * MOVE_SPEED;
        }
    }
}

pub fn c_class_change(
    mut general_client_events: EventWriter<GeneralClientEvents>,
    player_q: Query<&ActorClass, With<LocalPlayer>>,
    input: Res<Input<KeyCode>>,
) {

    if input.just_pressed(KeyCode::Semicolon)
    {
        info!("Sending Swap Class Request");
        
        let Ok(p_class) = player_q.get_single() else { return; };

        let class_cycle = [ClassType::DefaultClass, ClassType::MeleeClass, ClassType::RangedClass];

        let Some(index) = class_cycle.iter().position(|x| *x == p_class.get_class()) else { error!("Could not find class type {0:?}!", p_class.get_class()); return; };
        let index = (index + 1) % 3;
        
        general_client_events.send(GeneralClientEvents::ChangeClass(class_cycle[index]));
    }
}
