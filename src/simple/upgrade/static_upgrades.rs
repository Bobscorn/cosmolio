use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simple::assets::RonSerializedAsset;

use super::Upgrade;

#[derive(Asset, Clone, Default, Deserialize, Serialize, Reflect)]
pub struct UpgradeCollection
{
    pub upgrades: Vec<Upgrade>,
}

impl RonSerializedAsset for UpgradeCollection
{
    fn extensions() -> &'static [&'static str] {
        &["upgrades"]
    }
}

#[derive(Resource)]
pub struct StaticUpgrades
{
    pub upgrades: Handle<UpgradeCollection>,
}

pub fn setup_static_upgrades(
    world: &mut World
) -> Vec<UntypedHandle> {

    let asset_server = world.resource::<AssetServer>();

    let upgrades = asset_server.load("predefined_upgrades.upgrades");

    world.insert_resource(StaticUpgrades{ upgrades: upgrades.clone() });

    vec![upgrades.untyped()]
}
