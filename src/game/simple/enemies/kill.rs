use bevy::prelude::*;

use crate::game::simple::common::Dead;

use super::Enemy;

pub fn server_kill_dead_enemies(
    mut commands: Commands,
    dead_enemies: Query<Entity, (With<Dead>, With<Enemy>)>
) {
    for entity in &dead_enemies
    {
        let Some(ent_coms) = commands.get_entity(entity) else { continue };
        ent_coms.despawn_recursive();
    }
}

