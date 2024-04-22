mod bounds;
mod class;
mod enemy;
mod fonts;
mod images;
mod ron_asset;
mod upgrade;
mod wave;

pub use bounds::{Bounds, WorldBounds};
pub use class::ClassBaseData;
pub use enemy::EnemyData;
pub use fonts::Fonts;
pub use images::Images;
pub use wave::{WaveData, WaveDataResus};
pub use upgrade::{StaticUpgrades, UpgradeCollection};

use bevy::prelude::*;

#[derive(Resource)]
pub struct WaitingHandles
{
    pub handles: Vec<UntypedHandle>,
}

pub struct DataServerPlugin;
pub struct DataPlugin;

impl Plugin for DataServerPlugin 
{
    fn build(&self, app: &mut App) {
        app
            .init_asset::<WaveData>()
            .init_asset_loader::<ron_asset::RonAssetLoader<WaveData>>()
            .init_asset::<Bounds>()
            .init_asset_loader::<ron_asset::RonAssetLoader<Bounds>>()
            .add_systems(Startup, setup_server);
    }
}

impl Plugin for DataPlugin 
{
    fn build(&self, app: &mut App) {
        app
            .init_asset::<UpgradeCollection>()
            .init_asset_loader::<ron_asset::RonAssetLoader<UpgradeCollection>>()
            .init_asset::<ClassBaseData>()
            .init_asset_loader::<ron_asset::RonAssetLoader<ClassBaseData>>()
            .add_systems(Startup, (fonts::cs_setup_fonts, images::setup_images));
    }
}

fn setup_server(
    world: &mut World,
) {
    let mut existing_handles = match world.get_resource::<WaitingHandles>()
    {
        Some(h) => h.handles.clone(),
        None => Vec::new(),
    };

    existing_handles.append(&mut bounds::setup_world_bounds(world));
    existing_handles.append(&mut enemy::setup_enemies(world));
    existing_handles.append(&mut upgrade::setup_static_upgrades(world));
    existing_handles.append(&mut wave::setup_wave_data(world));
    existing_handles.append(&mut crate::simple::gameplay::classes::setup_classes(world));

    world.insert_resource(WaitingHandles { handles: existing_handles });
}
