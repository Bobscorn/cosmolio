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

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Effect
{
    #[default]
    Nothing,
    SpawnEntity(SpawnType),
    ApplyStatus{ target: Target, status: StatusEffect },
    CombinationEffect{ effects: Vec<Effect> },
}

impl Effect
{
    pub fn is_nothing(&self) -> bool
    {
        match self 
        {
            Self::Nothing => true,
            Self::ApplyStatus { target, status } => false,
            Self::SpawnEntity(_) => false,
            Self::CombinationEffect { effects } => effects.is_empty(),
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
    for effect_application in effect_events.read()
    {
        apply_effect(effect_application, &mut commands)
    }
}

fn apply_effect(effect_application: &EffectApplication, commands: &mut Commands)
{
    let EffectApplication { target, source, position, effect } = effect_application;
    
    info!("Applying effect {effect:?}");
    match effect
    {
        Effect::Nothing => todo!(),
        Effect::SpawnEntity(spawn_type) => 
        {
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
        Effect::CombinationEffect { effects } =>
        {
            for child_effect in effects
            {
                let child_application = EffectApplication { target: *target, source: *source, position: *position, effect: effect.clone() };
                apply_effect(&effect_application, commands);
            }
        }
    }
}



