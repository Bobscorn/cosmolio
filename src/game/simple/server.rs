use bevy::prelude::*;
use bevy_replicon::prelude::*;

use super::{
    plugin::*,
    common::*,
    player::*, abilities::{PlayerClass, ClassType}, client::GeneralClientEvents
};


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
            if client_id.raw() == player.0 {
                let movement = event.0 * time.delta_seconds() * MOVE_SPEED;
                **position += movement;
            }
        }
    }
}

pub fn s_general_client_events(
    mut players: Query<(&Player, &mut PlayerClass)>,
    mut client_events: EventReader<FromClient<GeneralClientEvents>>,
) {
    for FromClient { client_id, event } in client_events.read()
    {
        match event
        {
            GeneralClientEvents::ChangeClass(_new_class) => {},
            GeneralClientEvents::SwapClass => swap_class(&mut players, client_id.raw()),
        }
    }
}

fn swap_class(
    players: &mut Query<(&Player, &mut PlayerClass)>,
    player_id: u64
) {
    for (player, mut player_class) in players
    {
        if player.0 != player_id
        {
            continue;
        }

        info!("Swapping client '{player_id}'s class");
        if player_class.class == ClassType::DefaultClass
        {
            player_class.class = ClassType::MeleeClass;
        }
        else
        {
            player_class.class = ClassType::DefaultClass;
        }
        break;
    }
}
