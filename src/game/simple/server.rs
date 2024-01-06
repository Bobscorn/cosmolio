use bevy::prelude::*;
use bevy_replicon::prelude::*;

use super::{
    plugin::*,
    common::*,
    player::*, abilities::{PlayerClass, ClassType, Classes}, client::GeneralClientEvents
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
    mut commands: Commands,
    mut players: Query<(Entity, &Player, &mut PlayerClass)>,
    mut classes: ResMut<Classes>,
    mut client_events: EventReader<FromClient<GeneralClientEvents>>,
) {
    for FromClient { client_id, event } in client_events.read()
    {
        match event
        {
            GeneralClientEvents::ChangeClass(new_class) => change_class(&mut commands, &mut players, &mut classes, client_id.raw(), *new_class),
            GeneralClientEvents::SwapClass => swap_class(&mut players, client_id.raw()),
        }
    }
}

fn change_class(
    commands: &mut Commands,
    players: &mut Query<(Entity, &Player, &mut PlayerClass)>,
    classes: &mut ResMut<Classes>,
    player_id: u64,
    class: ClassType,
) {
    for (entity, player, mut player_class) in players
    {
        if player.0 != player_id
        {
            continue;
        }

        info!("Changing client '{player_id}'s class to {class:?}");
        if let Some(class) = classes.classes.get_mut(&player_class.class)
        {
            if let Some(teardown_fn_mutex) = &mut class.teardown_fn
            {
                if let Ok(mut teardown_fn) = teardown_fn_mutex.lock()
                {
                    (*teardown_fn)(commands, entity);
                }
            }
        }

        player_class.class = class;

        if let Some(class) = classes.classes.get_mut(&player_class.class)
        {
            if let Some(setup_fn_mutex) = &mut class.setup_fn
            {
                if let Ok(mut setup_fn) = setup_fn_mutex.lock()
                {
                    (*setup_fn)(commands, entity);
                }
            }
        }
        break;
    }
}

fn swap_class(
    players: &mut Query<(Entity, &Player, &mut PlayerClass)>,
    player_id: u64
) {
    for (_, player, mut player_class) in players
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
