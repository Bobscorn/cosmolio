use bevy::prelude::*;
use serde::{Deserialize, Serialize};

use super::stats::StatusEffect;

//   Temporary solution to allowing specific abilities to have certain effects
//   v

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq, Reflect)]
pub enum ChildType
{
    Melee,
    Projectile,
    Missile,
    Grenade,
    Explosion,
    ChildActor,
}

//   ^
//   Temporary brainstormed solution allowing specific abilities have certain effects

// ^
// Useful structs


// ^
// Actor stuff
// Effect Serialization
// v

// These enums contain the serializable form of every effect
// It is used in combination with the Trigger enum (the Trigger enum stores instances of this enum)

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SpawnType
{
    Explosion{ radius: f32, damage: f32, knockback_strength: f32 },
    // Future ideas v
    Missile{ damage: f32, speed: f32, acceleration: f32, knockback_strength: f32 },
    Lightning{  },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SpawnLocation // TODO: does this struct make sense?
{
    AtCaster,
    AtHitEnemy,

}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum DamageActor
{
    Victim, // The actor that was dealt the damage
    Instigator, // The actor responsible for dealing the damage
}

impl DamageActor
{
    pub fn as_str(&self) -> &'static str
    {
        match self
        {
            Self::Victim => "victim",
            Self::Instigator => "instigator",
        }
    }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedActorEffect
{
    InflictStatusEffect(StatusEffect),
    SpawnEffect(SpawnType, SpawnLocation),
    AffectHealth(f32),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedDamageChangeEffect
{
    MultiplyDamageEffect{ factor: f32 },
    AddDamageEffect{ amount: f32 },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedDamageViewEffect
{
    SpawnObjectAt{ which_actor: DamageActor, spawn_type: SpawnType },
    EveryXDamageEffect{ accumulated_damage: f32, damage_threshold: f32, which_actor: DamageActor, effect: SerializedActorEffect },
    EveryXHealedEffect{ accumulated_healing: f32, healing_threshold: f32, which_actor: DamageActor, effect: SerializedActorEffect },
    RegularEffect{ effect: SerializedActorEffect },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedKillEffect
{
    RegularEffect{ effect: SerializedActorEffect },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedDeathEffect
{
    RegularEffect{ effect: SerializedActorEffect },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedOnHitEffect
{
    SpawnEffectAtHitLocation{ spawn_type: SpawnType },
    RegularEffect{ effect: SerializedActorEffect },
}

impl Into<SerializedDamageViewEffect> for SerializedActorEffect
{
    fn into(self) -> SerializedDamageViewEffect {
        SerializedDamageViewEffect::RegularEffect { effect: self }
    }
}

impl Into<SerializedKillEffect> for SerializedActorEffect
{
    fn into(self) -> SerializedKillEffect {
        SerializedKillEffect::RegularEffect { effect: self }
    }
}

impl Into<SerializedDeathEffect> for SerializedActorEffect
{
    fn into(self) -> SerializedDeathEffect {
        SerializedDeathEffect::RegularEffect { effect: self }
    }
}

impl Into<SerializedOnHitEffect> for SerializedActorEffect
{
    fn into(self) -> SerializedOnHitEffect {
        SerializedOnHitEffect::RegularEffect { effect: self }
    }
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedEffectTrigger
{
    OnKill(SerializedKillEffect),
    OnDeath(SerializedDeathEffect),
    Periodically{ remaining_period: f32, period: f32, effect: SerializedActorEffect },
    OnDoDamage(SerializedDamageChangeEffect),       // <- DOES CHANGE DAMAGE
    OnDamageDone(SerializedDamageViewEffect),       // <- DOESN'T CHANGE DAMAGE
    OnReceiveDamage(SerializedDamageChangeEffect),  // <- DOES CHANGE DAMAGE
    OnDamageReceived(SerializedDamageViewEffect),   // <- DOESN'T CHANGE DAMAGE
    OnAbilityCast{ ability_type: ChildType, effect: SerializedActorEffect },
    OnAbilityHit{ ability_type: ChildType, effect: SerializedOnHitEffect },
    OnAbilityEnd{ ability_type: ChildType, effect: SerializedActorEffect },
}

// If ever needed, OR, NOT, and AND 'operators' (and other binary operators) could be made as convenience structs for ActorCondition

// TODO: Provide serialized versions of ActorCondition, and a serialized IfEffect
// pub trait ActorCondition
// {
//     fn check_actor(&self, actor_context: &ActorContext) -> bool;
// }

// pub struct IfEffect<TAct: ActorCondition, TEff: Effect>
// {
//     pub condition: TAct,
//     pub effect: TEff
// }

// impl<TAct: ActorCondition, TEff: Effect> Effect for IfEffect<TAct, TEff>
// {
//     fn apply_effect(&self, actor: &mut ActorContext) {
//         if self.condition.check_actor(&actor)
//         {
//             self.effect.apply_effect(actor);
//         }
//     }
// }
