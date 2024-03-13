use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};

use super::{behaviours::effect::ActorContext, enemies::WaveOverseer};



pub struct SimpleGameInspector;

impl Plugin for SimpleGameInspector
{
    fn build(&self, app: &mut App) {
        app
            .register_type::<ActorContext>()
            .register_type::<WaveOverseer>()
            .add_plugins((WorldInspectorPlugin::new(), ResourceInspectorPlugin::<WaveOverseer>::default()));
    }
}
