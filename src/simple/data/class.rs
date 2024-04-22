use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use crate::simple::gameplay::actor::{SerializedStat, effect::SerializedEffectTrigger};

use super::ron_asset::RonSerializedAsset;

#[derive(Asset, Clone, Debug, Default, TypePath, Deserialize, Serialize, PartialEq)]
pub struct ClassBaseData
{
    pub effects: Vec<SerializedEffectTrigger>,
    pub stats: Vec<SerializedStat>,
    pub description: String,
    pub name: String,
}

impl RonSerializedAsset for ClassBaseData
{
    fn extensions() -> &'static [&'static str] {
        &[".cbd"]
    }
}
