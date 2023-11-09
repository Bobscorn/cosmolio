use bevy::prelude::*;

use super::{
    Enemy,
    super::{common::Position, player::Player}
};


pub fn move_enemies(
    mut enemies: Query<(&Enemy, &mut Position), Without<Player>>,
    players: Query<&Position, (With<Player>, Without<Enemy>)>
) {
    for (enemy, mut position) in &mut enemies
    {
        let mut nearest_player_pos: Option<Vec2> = None;
        let mut nearest_player_distance_squared = f32::MAX;

        for player_position in &players
        {
            let distance_sq = (position.0 - player_position.0).length_squared();
            if distance_sq < nearest_player_distance_squared
            {
                nearest_player_pos = Some(player_position.0);
                nearest_player_distance_squared = distance_sq;
            }
        }

        let Some(nearest_player_pos) = nearest_player_pos else { return };

        let direction = (nearest_player_pos - position.0).normalize_or_zero();

        position.0 += direction * enemy.speed;
    }
}

