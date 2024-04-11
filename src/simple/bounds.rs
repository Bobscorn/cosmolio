use std::sync::Arc;

use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::{assets::RonSerializedAsset, common::Position, player::Player};


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

pub fn cs_restrict_players_to_bounds(
    mut players: Query<&mut Position, With<Player>>,
    bounds: Res<WorldBounds>,
    bound_assets: Res<Assets<Bounds>>,
) {
    let Some(bounds) = bound_assets.get(&bounds.bounds) else { error!("World Bounds were not loaded!"); return; };
    for mut player_pos in &mut players
    {
        player_pos.0.x = player_pos.0.x.max(bounds.min.x);
        player_pos.0.y = player_pos.0.y.max(bounds.min.y);
        player_pos.0.x = player_pos.0.x.min(bounds.max.x);
        player_pos.0.y = player_pos.0.y.min(bounds.max.y);
    }
}
