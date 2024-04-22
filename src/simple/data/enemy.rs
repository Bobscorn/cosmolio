use bevy::prelude::*;

use super::ClassBaseData;

#[derive(Resource)]
pub struct EnemyData
{
    pub regular_enemy_data: Handle<ClassBaseData>,
}

pub fn setup_enemies(
    world: &mut World
) -> Vec<UntypedHandle> {

    let asset_server = world.resource::<AssetServer>();

    let regular_enemy_handle = asset_server.load("regular_enemy_data.cbd");

    world.insert_resource(EnemyData{ regular_enemy_data: regular_enemy_handle.clone() });

    vec![regular_enemy_handle.untyped()]
}
