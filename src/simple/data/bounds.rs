use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::ron_asset::RonSerializedAsset;

#[derive(Asset, Default, Deserialize, Serialize, Reflect)]
pub struct Bounds
{
    pub min: Vec2,
    pub max: Vec2,
}

impl RonSerializedAsset for Bounds
{
    fn extensions() -> &'static [&'static str] {
        &["bounds"]
    }
}


#[derive(Resource)]
pub struct WorldBounds
{
    pub bounds: Handle<Bounds>,
}

pub fn setup_world_bounds(
    world: &mut World
) -> Vec<UntypedHandle> {
    let asset_server = world.resource::<AssetServer>();

    let bounds_handle = asset_server.load("world_bounds.bounds");

    world.insert_resource(WorldBounds { bounds: bounds_handle.clone() });

    vec![bounds_handle.untyped()]
}
