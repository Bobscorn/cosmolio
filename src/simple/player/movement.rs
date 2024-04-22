use bevy::prelude::*;
use bevy_replicon::prelude::*;

use crate::simple::gameplay::{Position, Knockback};

use super::{input::MoveDirection, Player, LocalPlayer};

pub const MOVE_SPEED: f32 = 300.0; // TODO: NOT THIS!! FK GLOBALS

pub fn s_movement_events(
    time: Res<Time>,
    mut move_events: EventReader<FromClient<MoveDirection>>,
    mut players: Query<(&Player, &mut Position, &Knockback)>,
) {
    for FromClient { client_id, event } in move_events.read()
    {
        for (player, mut position, knockback) in &mut players
        {
            if knockback.has_knockback()
            {
                continue;
            }
            if client_id == &player.0 {
                let movement = event.0 * time.delta_seconds() * MOVE_SPEED;
                **position += movement;
            }
        }
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
