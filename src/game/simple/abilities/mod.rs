use bevy::prelude::*;

use serde::{Serialize, Deserialize};

pub mod shoot;

pub fn add_component<T: Bundle + Default>(mut commands: Commands, ent: Entity)
{
    let Some(mut ent_coms) = commands.get_entity(ent) else { return; };

    ent_coms.insert(T::default());
}


#[derive(Debug, Default, Deserialize, Event, Serialize)]
pub enum AbilityActivation
{
    #[default]
    None,
    ShootBullet(Vec2, Entity)
}
