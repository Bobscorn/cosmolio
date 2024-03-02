use std::sync::{Arc, Mutex};

use bevy::{ecs::system::SystemId, prelude::*, utils::hashbrown::HashMap};

use serde::{Serialize, Deserialize};

pub mod melee;
pub mod default_class;
pub mod melee_class;
pub mod ranged_class;
pub mod bullet;
pub mod tags;
pub mod class;

use self::{
    class::{AbilityTrigger, Class, ClassBaseData, ClassType, Classes, PlayerClass}, melee_class::{c_big_swing, c_dash, c_normal_attack, c_slicing_projectile, c_spin_attack}, ranged_class::{c_basic_grenade_ability, c_basic_gun_ability, c_equipmachine_gun_ability, c_machine_gun_shoot_ability, c_missile_ability, c_shotgun_ability, s_ranged_class_setup, s_ranged_class_teardown}
};

use super::{behaviours::{effect::ActorContext, stats::Stat}, player::LocalPlayer};
use default_class::*;


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

pub fn add_ability<S, M>(world: &mut World, abilities: &mut HashMap<AbilityTrigger, SystemId>, trigger: AbilityTrigger, system: S)
    where S: IntoSystem<(), (), M> + 'static
{
    let system_id = world.register_system(system);
    abilities.insert(trigger, system_id);
}

pub fn setup_classes(
    world: &mut World,
) -> Vec<Handle<ClassBaseData>> {
    info!("Setting up player classes");
    let asset_server = world.resource::<AssetServer>();
    let ranged_data = asset_server.load("ranged_class_data.cbd");
    let melee_data = asset_server.load("melee_class_data.cbd");
    let default_data = asset_server.load("default_class_data.cbd");

    let shoot_system_id = world.register_system(c_shoot_ability::<PlayerBulletColor1>);
    let melee_system_id = world.register_system(c_melee_ability);

    let mut abilities = HashMap::with_capacity(1);
    abilities.insert(AbilityTrigger::JustPressed(KeyCode::Return), shoot_system_id);
    abilities.insert(AbilityTrigger::JustPressed(KeyCode::Space), melee_system_id);

    let default_class = Class {
        setup_fn: None,
        teardown_fn: None,
        abilities,
        base_data: default_data.clone()
    };

    let normal_attack_id = world.register_system(c_normal_attack);
    let big_swing_id = world.register_system(c_big_swing);
    let slicing_projectile_id = world.register_system(c_slicing_projectile);
    let spin_attack_id = world.register_system(c_spin_attack);
    let dash_ability_id = world.register_system(c_dash);

    let mut abilities = HashMap::with_capacity(5);
    abilities.insert(AbilityTrigger::JustPressed(KeyCode::Space), normal_attack_id);
    abilities.insert(AbilityTrigger::JustPressed(KeyCode::Return), big_swing_id);
    abilities.insert(AbilityTrigger::JustPressed(KeyCode::Q), slicing_projectile_id);
    abilities.insert(AbilityTrigger::JustPressed(KeyCode::E), spin_attack_id);
    abilities.insert(AbilityTrigger::JustPressed(KeyCode::T), dash_ability_id);

    let melee_class = Class {
        setup_fn: None,
        teardown_fn: None,
        abilities,
        base_data: melee_data.clone(),
    };

    let mut abilities = HashMap::with_capacity(7);

    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::Space), c_basic_gun_ability);
    add_ability(world, &mut abilities, AbilityTrigger::HeldDown(KeyCode::Space), c_machine_gun_shoot_ability);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::Return), c_basic_grenade_ability);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::R), c_shotgun_ability);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::G), c_equipmachine_gun_ability);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::F), c_missile_ability);

    let ranged_class = Class {
        setup_fn: Some(Arc::new(Mutex::new(s_ranged_class_setup))),
        teardown_fn: Some(Arc::new(Mutex::new(s_ranged_class_teardown))),
        abilities,
        base_data: ranged_data.clone(),
    };

    let mut classes = HashMap::with_capacity(2);
    classes.insert(ClassType::DefaultClass, default_class);
    classes.insert(ClassType::MeleeClass, melee_class);
    classes.insert(ClassType::RangedClass, ranged_class);

    world.insert_resource(Classes
    {
        classes,
    });

    info!("Successfully set up player classes");
    vec![default_data, melee_data, ranged_data]
}

/// Client side system responsible for reading input, and running the appropriate 'ability system'
pub fn c_class_input_system(
    mut commands: Commands,
    player: Query<&PlayerClass, With<LocalPlayer>>,
    classes: Res<Classes>,
    input: Res<Input<KeyCode>>,
) {
    let Ok(class) = player.get_single() else { return; };

    for (trigger, system) in &classes.classes[&class.get_class()].abilities
    {
        let (run_condition, keycode) = match trigger {
            AbilityTrigger::JustPressed(keycode) => (input.just_pressed(*keycode), *keycode),
            AbilityTrigger::HeldDown(keycode) => (input.pressed(*keycode), *keycode),
            AbilityTrigger::JustPressedOrReleased(keycode) => (input.just_pressed(*keycode) || input.just_released(*keycode), *keycode),
            AbilityTrigger::JustReleased(keycode) => (input.just_released(*keycode), *keycode),
        };
        if run_condition
        {
            commands.run_system(*system);
        }
    }
}


