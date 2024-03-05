use bevy::prelude::*;
use bevy_inspector_egui::quick::WorldInspectorPlugin;

use super::{behaviours::effect::ActorContext, enemies::EnemySpawning};



pub struct SimpleGameInspector;

impl Plugin for SimpleGameInspector
{
    fn build(&self, app: &mut App) {
        app
            .register_type::<ActorContext>()
            .register_type::<EnemySpawning>()
            .add_plugins(WorldInspectorPlugin::new());
    }
}
