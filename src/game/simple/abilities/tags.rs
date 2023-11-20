use bevy::prelude::*;

use serde::{Deserialize, Serialize};

#[derive(Component, Serialize, Deserialize)]
pub struct CanUseAbilities;

#[derive(Component)]
pub struct CanMove;



