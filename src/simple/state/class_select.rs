use bevy::prelude::*;

use crate::simple::{classes::class::ActorClass, visuals::ui::Fonts};

/// Tag component for the root UI node of the Class select UI
#[derive(Component)]
pub struct ClassSelectUIRoot
{
}

/// Data component for each UI Node that will catch UI events
#[derive(Component)]
pub struct ClassSelectUI
{
    pub class: ActorClass,
}

/// UI Setup function
pub fn setup_class_select_ui(
    mut commands: Commands,
    fonts: Res<Fonts>,
) {
    commands.spawn((NodeBundle {
        // TODO
        ..default()
    }, ClassSelectUIRoot)).with_children(|builder| {
        builder.spawn(); // TODO 
    });
}

pub fn handle_class_select_ui(

) {

}
