use bevy::{log::LogPlugin, prelude::*};
use bevy_replicon::RepliconPlugins;
use bevy_replicon_renet::RepliconRenetPlugins;

mod simple;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(LogPlugin{ filter: "info,wgpu_core=warn,wgpu_hal=warn,cosmolio=debug".into(), level: bevy::log::Level::DEBUG, ..default() })
        )
        .add_plugins(simple::inspector_plugin::SimpleGameInspector)
        .add_plugins((RepliconPlugins, RepliconRenetPlugins, simple::plugin::SimpleGame))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}
