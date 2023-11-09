use std::time::Duration;

use bevy::prelude::*;
use bevy_replicon::ReplicationPlugins;

mod game;
mod systems;

fn main() {
    App::new()
        .init_resource::<game::simple::plugin::Cli>()
        .add_plugins((DefaultPlugins, ReplicationPlugins, game::simple::plugin::SimpleGame))
        .insert_resource(FixedTime::new(Duration::from_millis(16)))
        .run();
}
