
mod class;
mod default_class;
mod melee_class;
mod ranged_class;
mod setup;

pub use class::{ActorClass, Class, ClassType, Classes};
pub use setup::setup_classes;

use bevy::prelude::*;
use bevy_replicon::prelude::*;

use crate::simple::state;

pub struct ClassesPlugin;

impl Plugin for ClassesPlugin
{
    fn build(&self, app: &mut App) {
        app
            // Data v
            .replicate::<ranged_class::RangedClassData>()
            .replicate::<ActorClass>()

            // Events v
            .add_client_event::<default_class::DefaultClassAbility>(ChannelKind::Ordered)
            .add_client_event::<melee_class::MeleeClassEvent>(ChannelKind::Ordered)
            .add_client_event::<ranged_class::RangedClassEvent>(ChannelKind::Ordered)
            // Systems v
            //  Authority systems v
            .add_systems(FixedUpdate, (
                default_class::s_default_class_ability_response,
                melee_class::s_melee_class_ability_response,
                ranged_class::s_ranged_class_response,
                class::s_setup_initial_class,
            ).in_set(state::AuthoritySystems))
            .add_systems(FixedUpdate, (
                setup::c_class_input_system,
            ).in_set(state::HostAndClientSystems).in_set(state::FightingSystems))
            ;
    }
}
