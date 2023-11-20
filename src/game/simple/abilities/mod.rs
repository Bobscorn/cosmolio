use bevy::{prelude::*, ecs::system::SystemId, utils::HashMap};

use serde::{Serialize, Deserialize};

pub mod melee;
pub mod default_class;
pub mod melee_class;
pub mod bullet;
pub mod tags;

use self::melee_class::{c_normal_attack, c_big_swing, c_slicing_projectile, c_spin_attack};

use super::player::LocalPlayer;
use default_class::*;


pub struct Class
{
    pub abilities: HashMap<KeyCode, SystemId>,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize)]
pub enum ClassType
{
    DefaultClass,
    MeleeClass,
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

pub struct PlayerBulletColor1;
impl GetColor for PlayerBulletColor1
{
    fn get_color() -> Color {
        Color::rgb(0.8, 0.2, 0.2)
    }
}

pub struct PlayerBulletColor2;
impl GetColor for PlayerBulletColor2
{
    fn get_color() -> Color {
        Color::rgb(0.2, 0.8, 0.2)
    }
}

/// Registers and stores client ability systems to be run as one-shot systems
/// Run this as a Startup system
pub fn c_setup_abilities(
    world: &mut World,
) {
    let shoot_system_id = world.register_system(c_shoot_ability::<PlayerBulletColor1>);
    let melee_system_id = world.register_system(c_melee_ability);

    let mut abilities = HashMap::with_capacity(1);
    abilities.insert(KeyCode::Return, shoot_system_id);
    abilities.insert(KeyCode::Space, melee_system_id);

    let default_class = Class {
        abilities,
    };

    let normal_attack_id = world.register_system(c_normal_attack);
    let big_swing_id = world.register_system(c_big_swing);
    let slicing_projectile_id = world.register_system(c_slicing_projectile);
    let spin_attack_id = world.register_system(c_spin_attack);

    let mut abilities = HashMap::with_capacity(4);
    abilities.insert(KeyCode::Space, normal_attack_id);
    abilities.insert(KeyCode::Return, big_swing_id);
    abilities.insert(KeyCode::Q, slicing_projectile_id);
    abilities.insert(KeyCode::E, spin_attack_id);

    let melee_class = Class {
        abilities,
    };

    let mut classes = HashMap::with_capacity(2);
    classes.insert(ClassType::DefaultClass, default_class);
    classes.insert(ClassType::MeleeClass, melee_class);

    world.insert_resource(Classes
    {
        classes,
    });
}

/// Client side system responsible for reading input, and running the appropriate 'ability system'
pub fn c_class_input_system(
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
            info!("Running system for input {keycode:?}");
            commands.run_system(*system);
        }
    }
}


