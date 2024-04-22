mod damage;
mod dead;
mod effect_descriptions;
mod simple_effects;
mod stats;

pub mod effect;
pub mod effect_application;
pub use effect::ChildType;

pub use damage::{Damage, DamageKnockback, DamageSource, DamageEvent};
pub use stats::{Stat, StatusEffect, StatModification, SerializedStat};

// Struct that contains all the data useful to an 'affectable' entity
#[derive(Component, Default, Serialize, Deserialize, Reflect)]
pub struct ActorContext
{
    pub effects: Vec<effect::SerializedEffectTrigger>,
    pub status_effects: Vec<StatusEffect>,
    pub stats: HashMap<Stat, f32>,
    pub last_damage_source: Option<DamageSource>,
}

impl ActorContext
{
    pub fn get_stat(&self, stat: &Stat) -> Option<f32>
    {
        self.stats.get(stat).map(|x| { *x })
    }

    pub fn get_or_create_stat(&mut self, stat: Stat) -> f32
    {
        if let Some(val) = self.stats.get(&stat)
        {
            return *val;
        }

        self.stats.insert(stat, 0.0);
        0.0
    }

    pub fn set_stat(&mut self, stat: Stat, value: f32)
    {
        self.stats.insert(stat, value);
    }
}

// Struct used for entites created by/for an actor, that should apply effects on behalf of that actor
#[derive(Component, Reflect)]
pub struct ActorChild // TODO: rename to ActorAbility? Is there a use case for anything besides abilities?
{
    pub parent_actor: Entity,
    pub ability_type: ChildType
}

/// This component stores the sensor entities (using bevy_rapier sensors) for an actor.
/// Expected to be attached to the same entity as a ActorContext.
#[derive(Component, Reflect)]
pub struct ActorSensors
{
    pub sensors: Vec<Entity>,
}

use bevy::{prelude::*, utils::HashMap};
use bevy_replicon::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simple::state;

pub struct ActorPlugin;

impl Plugin for ActorPlugin
{
    fn build(&self, app: &mut App) {
        app
            .add_event::<DamageEvent>()
            .replicate::<ActorContext>()
            .replicate::<Damage>()
            .add_systems(FixedUpdate, (
                dead::s_destroy_dead_things,
            ).in_set(state::AuthoritySystems).in_set(state::FightingSystems))
            .add_systems(FixedUpdate, (
                damage::s_do_damage_events,
            ).in_set(state::AuthoritySystems))
            ;
    }
}

