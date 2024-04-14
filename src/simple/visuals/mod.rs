pub mod healthbar;
pub mod ui;

use bevy::prelude::*;

#[derive(Resource)]
pub struct Images
{
    pub enemy_img: Handle<Image>,
    pub player_img: Handle<Image>,
}

pub fn setup_images(
    world: &mut World,
) {
    let asset_server = world.resource::<AssetServer>();

    let imgs = Images 
    {
        enemy_img: asset_server.load("enemy_icon.png"),
        player_img: asset_server.load("player_icon.png"),
    };

    world.insert_resource(imgs);
}
