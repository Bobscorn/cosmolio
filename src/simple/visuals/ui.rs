use bevy::prelude::*;


#[derive(Resource)]
pub struct Fonts
{
    pub upgrade_font: Handle<Font>,
}


pub fn cs_setup_fonts(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    commands.insert_resource(Fonts{ upgrade_font: asset_server.load("upgrade_font.ttf") });
}
