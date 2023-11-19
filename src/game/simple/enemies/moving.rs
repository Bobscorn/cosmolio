use bevy::prelude::*;

use crate::game::simple::common::Velocity;

use super::{
    Enemy,
    super::{common::Position, player::Player}
};


pub fn cs_move_enemies(
    mut enemies: Query<(&Enemy, &Position, &mut Velocity), Without<Player>>,
    players: Query<&Position, (With<Player>, Without<Enemy>)>,
    time: Res<Time>,
) {
    const MAX_ACCELERATION: f32 = 125.0;
    let max_delta_v = MAX_ACCELERATION * time.delta_seconds();

    for (enemy, position, mut velocity) in &mut enemies
    {
        // Cap velocity:
        const MAX_VELOCITY_SQ: f32 = 200.0 * 200.0;
        if velocity.length_squared() > MAX_VELOCITY_SQ
        {
            velocity.0 = velocity.normalize() * 200.0;
        }

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

        let target_velocity = direction * enemy.speed;
        let diff = target_velocity - velocity.0;
        let diff_mag = diff.length();
        if diff_mag <= 0.0
        {
            continue;
        }

        let diff_norm = diff / diff_mag;

        let diff = diff_norm * diff_mag.min(max_delta_v);

        velocity.0 += diff;
    }
}

