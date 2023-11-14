use bevy::prelude::*;
use bevy_replicon::ReplicationPlugins;

mod game;
mod systems;

fn main() {
    App::new()
        .init_resource::<game::simple::plugin::Cli>()
        .add_plugins((DefaultPlugins, ReplicationPlugins, game::simple::plugin::SimpleGame))
        .insert_resource(Time::<Fixed>::from_hz(60.0))
        .run();
}
