use bevy::{prelude::App, DefaultPlugins};
use bevy_replicon::ReplicationPlugins;

mod conn;
mod game;
mod systems;

fn main() {
    App::new()
        .init_resource::<game::simple::plugin::Cli>()
        .add_plugins((DefaultPlugins, ReplicationPlugins, game::simple::plugin::SimpleGame))
        .run();
}
