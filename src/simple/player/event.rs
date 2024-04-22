use bevy::prelude::*;
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simple::{
    gameplay::{actor::ActorContext, classes::{ActorClass, Classes, ClassType}},
    data::ClassBaseData,
};
use super::{LocalPlayer, Player};


#[derive(Event, PartialEq, Eq, Serialize, Deserialize)]
pub enum GeneralClientEvents
{
    ChangeClass(ClassType),
    SwapClass,
}

pub fn c_class_change(
    mut general_client_events: EventWriter<GeneralClientEvents>,
    player_q: Query<&ActorClass, With<LocalPlayer>>,
    input: Res<ButtonInput<KeyCode>>,
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

