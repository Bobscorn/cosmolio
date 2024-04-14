use bevy::prelude::*;
use bevy_inspector_egui::quick::{ResourceInspectorPlugin, WorldInspectorPlugin};

use super::{behaviours::effect::ActorContext, common::{Position, Velocity}, enemies::WaveOverseer};



pub struct SimpleGameInspector;

impl Plugin for SimpleGameInspector
{
    fn build(&self, app: &mut App) {
        if std::env::args().all(|x| !x.contains("server"))
        {
            return;
        }

        app
            .register_type::<ActorContext>()
            .register_type::<WaveOverseer>()
            .register_type::<Velocity>()
            .register_type::<Position>()
            .add_plugins((WorldInspectorPlugin::new(), ResourceInspectorPlugin::<WaveOverseer>::default(), ResourceInspectorPlugin::<Time<Virtual>>::default()));
    }
}
