use bevy::prelude::*;


#[derive(Resource)]
pub struct Images
{
    pub enemy_img: Handle<Image>,
    pub player_img: Handle<Image>,
}

pub fn setup_images(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let imgs = Images 
    {
        enemy_img: asset_server.load("enemy_icon.png"),
        player_img: asset_server.load("player_icon.png"),
    };

    commands.insert_resource(imgs);
}
