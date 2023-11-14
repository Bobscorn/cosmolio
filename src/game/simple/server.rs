use bevy::prelude::*;
use bevy_replicon::prelude::*;

use super::{
    plugin::*,
    common::*,
    player::*
};


pub fn movement_system(
    time: Res<Time>,
    mut move_events: EventReader<FromClient<MoveDirection>>,
    mut players: Query<(&Player, &mut Position)>,
) {
    for FromClient { client_id, event } in move_events.read()
    {
        for (player, mut position) in &mut players
        {
            if client_id.raw() == player.0 {
                let movement = event.0 * time.delta_seconds() * MOVE_SPEED;
                **position += movement;
            }
        }
    }
}
