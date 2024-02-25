use bevy::prelude::*;
use bevy_replicon::ReplicationPlugins;

mod simple;

fn main() {
    App::new()
        .init_resource::<simple::plugin::Cli>()
        .add_plugins((DefaultPlugins, ReplicationPlugins, simple::plugin::SimpleGame))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}
