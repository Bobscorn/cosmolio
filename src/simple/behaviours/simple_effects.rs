use std::sync::Arc;

use bevy::prelude::*;

use crate::simple::consts::PLAYER_GROUPS;

use super::{
    effect::{
        ActorDamageEffectContext, 
        ActorEffectContext, 
        ActorOnHitEffectContext,
        DamageEffect, 
        Effect, 
        OnHitEffect,
        SerializeInto, 
        SerializedActorEffect, 
        SerializedDamageEffect, 
        SerializedOnHitEffect,
        SpawnLocation, 
        SpawnType, 
        StatusEffect, 
        WrappedEffect
    }, 
    damage::DamageKnockback,
    explosion::ExplosionReplicationBundle
};






pub struct DamageFactor
{
    pub factor: f32
}

impl SerializeInto<SerializedDamageEffect> for DamageFactor
{
    fn serialize_into(&self) -> SerializedDamageEffect {
        SerializedDamageEffect::MultiplyDamageEffect { factor: self.factor }
    }
}

impl DamageEffect for DamageFactor
{
    fn process_damage(&self, context: &mut ActorDamageEffectContext) -> f32 {
        context.damage * self.factor
    }
}

pub struct DamageAddition
{
    pub amount: f32
}

impl SerializeInto<SerializedDamageEffect> for DamageAddition
{
    fn serialize_into(&self) -> SerializedDamageEffect {
        SerializedDamageEffect::AddDamageEffect { amount: self.amount }
    }
}

impl DamageEffect for DamageAddition
{
    fn process_damage(&self, context: &mut ActorDamageEffectContext) -> f32 {
        context.damage + self.amount
    }
}


pub struct InflictStatusEffect
{
    pub status_effect: StatusEffect,
}

impl SerializeInto<SerializedActorEffect> for InflictStatusEffect
{
    fn serialize_into(&self) -> SerializedActorEffect {
        SerializedActorEffect::InflictStatusEffect(self.status_effect)
    }
}

impl Effect for InflictStatusEffect
{
    fn apply_effect(&self, context: &mut ActorEffectContext) {
        context.actor.status_effects.push(self.status_effect);
    }
}

impl InflictStatusEffect
{
    pub fn new(status_effect: StatusEffect) -> Self
    {
        Self 
        {
            status_effect
        }
    }
}

pub fn do_spawn_object(commands: &mut Commands, spawn_type: SpawnType, location: Vec2)
{
    match spawn_type
    {
        SpawnType::Explosion { radius, damage, knockback_strength } => 
        {
            commands.spawn(ExplosionReplicationBundle::new(
                radius, 
                knockback_strength, 
                location, 
                damage, 
                PLAYER_GROUPS, 
                Some(DamageKnockback::Repulsion { 
                    center: location,
                    strength: knockback_strength 
                })
            ));
        },
        SpawnType::Missile {  } => todo!(),
        SpawnType::Lightning {  } => todo!(),
    }
}

pub struct SpawnEffect
{
    pub spawn_type: SpawnType,
    pub spawn_location: SpawnLocation,
}

impl SerializeInto<SerializedActorEffect> for SpawnEffect
{
    fn serialize_into(&self) -> SerializedActorEffect {
        SerializedActorEffect::SpawnEffect(self.spawn_type, self.spawn_location)
    }
}

impl Effect for SpawnEffect
{
    fn apply_effect(&self, context: &mut ActorEffectContext) {
        do_spawn_object(context.commands, self.spawn_type, context.location.0);
    }
}

///
/// Spawns an object (similar to `SpawnEffect`) at the location an ability hits something
pub struct SpawnAtHitEffect
{
    pub spawn_type: SpawnType,
}

impl SerializeInto<SerializedOnHitEffect> for SpawnAtHitEffect
{
    fn serialize_into(&self) -> SerializedOnHitEffect {
        SerializedOnHitEffect::SpawnEffectAtHitLocation{ spawn_type: self.spawn_type }
    }
}

impl OnHitEffect for SpawnAtHitEffect
{
    fn apply_effect(&self, context: &mut ActorOnHitEffectContext)
    {
        do_spawn_object(context.commands, self.spawn_type, context.hit_location);
    }
}


impl SerializedActorEffect
{
    pub fn instantiate(&self) -> Arc<dyn Effect>
    {
        match self
        {
            SerializedActorEffect::InflictStatusEffect(status_effect) =>
            {
                Arc::new(InflictStatusEffect::new(*status_effect))
            },
            SerializedActorEffect::SpawnEffect(spawn_type, spawn_location) =>
            {
                Arc::new(SpawnEffect{ spawn_type: *spawn_type, spawn_location: *spawn_location })
            }
        }
    }
}


impl SerializedDamageEffect
{
    pub fn instantiate(&self) -> Arc<dyn DamageEffect>
    {
        match self
        {
            SerializedDamageEffect::MultiplyDamageEffect { factor } =>
            {
                Arc::new(DamageFactor{ factor: *factor })
            },
            SerializedDamageEffect::AddDamageEffect { amount } =>
            {
                Arc::new(DamageAddition{ amount: *amount })
            },
            SerializedDamageEffect::RegularEffect { effect } =>
            {
                Arc::new(WrappedEffect { effect: effect.instantiate() })
            }
        }
    }
}

impl SerializedOnHitEffect
{
    pub fn instantiate(&self) -> Arc<dyn OnHitEffect>
    {
        match self
        {
            SerializedOnHitEffect::SpawnEffectAtHitLocation{ spawn_type } =>
            {
                Arc::new(SpawnAtHitEffect{ spawn_type: *spawn_type })
            },
            SerializedOnHitEffect::RegularEffect{ effect } =>
            {
                Arc::new(WrappedEffect { effect: effect.instantiate() })
            }
        }
    }
}
