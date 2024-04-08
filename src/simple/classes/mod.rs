use std::sync::{Arc, Mutex};

use bevy::{prelude::*, utils::hashbrown::HashMap};

pub mod melee;
pub mod default_class;
pub mod melee_class;
pub mod ranged_class;
pub mod bullet;
pub mod tags;
pub mod class;

use self::{
    class::{AbilityTrigger, ActorClass, Class, ClassType, Classes, LabelledSystemId}, 
    melee_class::{c_big_swing, c_dash, c_normal_attack, c_slicing_projectile, c_spin_attack}, 
    ranged_class::{
        c_basic_grenade_ability, 
        c_basic_gun_ability, 
        c_equipmachine_gun_ability, 
        c_machine_gun_shoot_ability, 
        c_missile_ability, 
        c_shotgun_ability, 
        s_ranged_class_setup, 
        s_ranged_class_teardown
    }
};

use super::player::LocalPlayer;
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

pub fn add_ability<S, M>(world: &mut World, abilities: &mut HashMap<AbilityTrigger, LabelledSystemId>, trigger: AbilityTrigger, name: &str, system: S)
    where S: IntoSystem<(), (), M> + 'static
{
    let system_id = world.register_system(system);
    abilities.insert(trigger, LabelledSystemId{ name: name.into(), system_id });
}

pub fn setup_classes(
    world: &mut World,
) -> Vec<UntypedHandle> {
    info!("Setting up player classes");
    let asset_server = world.resource::<AssetServer>();
    let ranged_data = asset_server.load("ranged_class_data.cbd");
    let melee_data = asset_server.load("melee_class_data.cbd");
    let default_data = asset_server.load("default_class_data.cbd");

    let mut abilities = HashMap::with_capacity(1);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::Space), "Melee attack", c_melee_ability);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::Enter), "Gun attack", c_shoot_ability::<PlayerBulletColor1>);

    let default_class = Class {
        setup_fn: None,
        teardown_fn: None,
        abilities,
        base_data: default_data.clone()
    };

    let mut abilities = HashMap::with_capacity(5);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::Space), "Base attack", c_normal_attack);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::Enter), "Big swing", c_big_swing);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::KeyQ), "Slicing projectile", c_slicing_projectile);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::KeyE), "Spin attack", c_spin_attack);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::KeyT), "Dash", c_dash);

    let melee_class = Class {
        setup_fn: None,
        teardown_fn: None,
        abilities,
        base_data: melee_data.clone(),
    };

    let mut abilities = HashMap::with_capacity(7);

    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::Space), "Basic gun attack", c_basic_gun_ability);
    add_ability(world, &mut abilities, AbilityTrigger::HeldDown(KeyCode::Space), "Machine gun fire", c_machine_gun_shoot_ability);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::Enter), "Grenade throw", c_basic_grenade_ability);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::KeyR), "Shotgun blast", c_shotgun_ability);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::KeyG), "Equip machinegun", c_equipmachine_gun_ability);
    add_ability(world, &mut abilities, AbilityTrigger::JustPressed(KeyCode::KeyF), "Fire missiiles", c_missile_ability);

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
    vec![default_data.untyped(), melee_data.untyped(), ranged_data.untyped()]
}

/// Client side system responsible for reading input, and running the appropriate 'ability system'
pub fn c_class_input_system(
    mut commands: Commands,
    player: Query<&ActorClass, With<LocalPlayer>>,
    classes: Res<Classes>,
    input: Res<ButtonInput<KeyCode>>,
) {
    let Ok(class) = player.get_single() else { return; };

    for (trigger, labelled_system) in &classes.classes[&class.get_class()].abilities
    {
        let (run_condition, _keycode) = match trigger {
            AbilityTrigger::JustPressed(keycode) => (input.just_pressed(*keycode), *keycode),
            AbilityTrigger::HeldDown(keycode) => (input.pressed(*keycode), *keycode),
            AbilityTrigger::JustPressedOrReleased(keycode) => (input.just_pressed(*keycode) || input.just_released(*keycode), *keycode),
            AbilityTrigger::JustReleased(keycode) => (input.just_released(*keycode), *keycode),
        };
        if run_condition
        {
            debug!("Running {} class {} system", class, labelled_system.name);
            commands.run_system(labelled_system.system_id);
        }
    }
}


