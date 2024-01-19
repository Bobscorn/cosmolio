use bevy::prelude::*;

use serde::{Deserialize, Serialize};

use crate::game::simple::consts::PLAYER_GROUPS;

use super::explosion::ExplosionReplicationBundle;

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Owner
{
    Player{ id: u64 },
    Enemy{ ent: Entity },
}

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Target
{
    Caster,
    NearestAlly,
    NearestAllyExcludingCaster,
    NearestEnemy,

}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StatusEffect
{

}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SpawnType
{
    Explosion{ radius: f32, damage: f32, knockback_strength: f32, owner: Owner },
    // Future ideas v
    Missile{  },
    Lightning{  },
}

#[derive(Default, Debug, Clone, Copy, Serialize, Deserialize)]
pub enum Effect
{
    #[default]
    Nothing,
    SpawnEntity(SpawnType),
    ApplyStatus{ target: Target, status: StatusEffect },
}

impl Effect
{
    pub fn is_nothing(&self) -> bool
    {
        match self 
        {
            Self::Nothing => true,
            Self::ApplyStatus { target, status } => false,
            Self::SpawnEntity(_) => false
        }
    }
}

#[derive(Component)]
pub struct OnDestroy
{
    pub effect: Effect,
}

#[derive(Event)]
pub struct EffectApplication
{
    pub target: Option<Entity>,
    pub source: Option<Entity>,
    pub position: Vec2,
    pub effect: Effect,
}


pub fn s_apply_effect(
    mut commands: Commands,
    mut effect_events: EventReader<EffectApplication>,
) {
    for EffectApplication { target, source, position, effect } in effect_events.read()
    {
        info!("Applying effect {effect:?}");
        match effect
        {
            Effect::Nothing => todo!(),
            Effect::SpawnEntity(spawn_type) => {
                match spawn_type
                {
                    SpawnType::Explosion { radius, damage, knockback_strength, owner } => 
                    {
                        commands.spawn(ExplosionReplicationBundle::new(*radius, *knockback_strength, *position, *damage, PLAYER_GROUPS));
                    },
                    SpawnType::Missile {  } => todo!(),
                    SpawnType::Lightning {  } => todo!(),
                }
            },
            Effect::ApplyStatus { target, status } => todo!(),
        }
    }
}



