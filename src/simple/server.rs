use bevy::prelude::*;
use bevy_replicon::prelude::*;

use super::{
    behaviours::effect::ActorContext, classes::class::{ClassBaseData, ClassType, Classes, ActorClass}, client::GeneralClientEvents, common::*, player::*, plugin::*
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
            if client_id == &player.0 {
                let movement = event.0 * time.delta_seconds() * MOVE_SPEED;
                **position += movement;
            }
        }
    }
}

pub fn s_general_client_events(
    mut commands: Commands,
    mut players: Query<(Entity, &Player, &mut ActorClass, &mut ActorContext)>,
    mut classes: ResMut<Classes>,
    class_data: Res<Assets<ClassBaseData>>,
    mut client_events: EventReader<FromClient<GeneralClientEvents>>,
) {
    for FromClient { client_id, event } in client_events.read()
    {
        match event
        {
            GeneralClientEvents::ChangeClass(new_class) => change_class(&mut commands, &mut players, &class_data, &mut classes, *client_id, *new_class),
            GeneralClientEvents::SwapClass => swap_class(&mut commands, &class_data, &mut classes, &mut players, *client_id),
        }
    }
}

fn change_class(
    commands: &mut Commands,
    players: &mut Query<(Entity, &Player, &mut ActorClass, &mut ActorContext)>,
    class_data: &Res<Assets<ClassBaseData>>,
    classes: &mut ResMut<Classes>,
    player_id: ClientId,
    class: ClassType,
) {
    for (entity, player, mut player_class, mut actor_context) in players
    {
        if player.0 != player_id
        {
            continue;
        }
        
        player_class.set_class(commands, class_data, classes, &mut actor_context, entity, class);
        break;
    }
}

fn swap_class(
    commands: &mut Commands,
    class_data: &Res<Assets<ClassBaseData>>,
    classes: &mut ResMut<Classes>,
    players: &mut Query<(Entity, &Player, &mut ActorClass, &mut ActorContext)>,
    player_id: ClientId,
) {
    for (entity, player, mut player_class, mut actor) in players
    {
        if player.0 != player_id
        {
            continue;
        }

        info!("Swapping client '{}'s class", player_id.get());
        if player_class.get_class() == ClassType::DefaultClass
        {
            player_class.set_class(commands, &class_data, classes, &mut actor, entity, ClassType::MeleeClass);
        }
        else
        {
            player_class.set_class(commands, &class_data, classes, &mut actor, entity, ClassType::DefaultClass);
        }
        break;
    }
}
