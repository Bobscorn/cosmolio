use bevy::prelude::*;

use crate::simple::{
    data::{Bounds, WorldBounds},
    gameplay::Position,
    player::Player,
};

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