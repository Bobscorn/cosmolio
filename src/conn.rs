use bevy::prelude::*;
use bevy_replicon::prelude::*;


pub fn setup_multiplayer_for_app(app: &mut App)
{
    app.add_plugins((ReplicationPlugins));
}

