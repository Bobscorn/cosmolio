use bevy::{prelude::*, ecs::system::{SystemId, RunSystemOnce}, utils::HashMap};

use bevy_replicon::{renet::ClientId, prelude::FromClient, server::ClientEntityMap};
use serde::{Serialize, Deserialize};

mod shoot;
pub mod default_class;
pub mod bullet;

use shoot::*;

use super::player::LocalPlayer;


pub struct Class
{
    pub abilities: HashMap<KeyCode, SystemId>,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassType
{
    DefaultClass,
}

#[derive(Component, Serialize, Deserialize)]
pub struct PlayerClass
{
    pub class: ClassType,
}

#[derive(Resource)]
pub struct Classes
{
    pub classes: HashMap<ClassType, Class>,
}

/// Registers and stores client ability systems to be run as one-shot systems
pub fn setup_client_abilities(
    world: &mut World,

) {
    let shoot_system_id = world.register_system(client_shoot_ability_systems);

    let mut abilities = HashMap::with_capacity(1);
    abilities.insert(KeyCode::Space, shoot_system_id);

    let default_class = Class {
        abilities,
    };

    let mut classes = HashMap::with_capacity(1);
    classes.insert(ClassType::DefaultClass, default_class);

    world.insert_resource(Classes
    {
        classes,
    });
}

/// Client side system responsible for reading input, and running the appropriate 'ability system'
pub fn client_ability_system(
    mut commands: Commands,
    player: Query<&PlayerClass, With<LocalPlayer>>,
    classes: Res<Classes>,
    input: Res<Input<KeyCode>>,
) {
    let Ok(class) = player.get_single() else { return; };

    for (keycode, system) in &classes.classes[&class.class].abilities
    {
        if input.just_pressed(*keycode)
        {
            commands.run_system(*system);
        }
    }
}


