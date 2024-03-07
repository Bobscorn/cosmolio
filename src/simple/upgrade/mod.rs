pub mod ui;
pub mod description;

use serde::{Deserialize, Serialize};

use super::behaviours::effect::{SerializedActorEffect, SerializedEffectTrigger};

// #[derive(Deserialize, Serialize)]
// pub enum TriggerType
// {
//     OnDamage,
//     Periodically,
//     OnKill,
//     OnDeath,
//     OnReceiveDamage,
//     OnAbilityCast,
//     OnAbilityHit,
//     OnAbilityEnd,
// }

// #[derive(Deserialize, Serialize)]
// pub enum ReplaceCriteria
// {
//     ByEffectTrigger(TriggerType),
//     ByRegularEffectType(SerializedActorEffect),
// }

#[derive(Clone, Copy, Deserialize, Serialize)]
pub enum UpgradeBehaviour
{
    AddEffect(SerializedEffectTrigger),
    // ReplaceEffects{ criteria: ReplaceCriteria,  },
    // RemoveEffects // Maybe?
}

#[derive(Deserialize, Serialize)]
pub struct Upgrade
{
    pub behaviour: UpgradeBehaviour,
    pub description: String,
}
