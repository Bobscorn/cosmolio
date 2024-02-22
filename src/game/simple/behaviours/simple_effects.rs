use std::{sync::Arc, thread::spawn};

use bevy::prelude::*;

use crate::game::simple::consts::PLAYER_GROUPS;

use super::{effect::{ActorDamageEffectContext, ActorEffectContext, DamageEffect, Effect, SerializeInto, SerializedActorEffect, SerializedDamageEffect, SpawnLocation, SpawnType, StatusEffect, WrappedEffect}, explosion::ExplosionReplicationBundle};






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
        match self.spawn_type
        {
            SpawnType::Explosion { radius, damage, knockback_strength } => 
            {
                context.commands.spawn(ExplosionReplicationBundle::new(
                    radius, 
                    knockback_strength, 
                    context.location.0, 
                    damage, 
                    PLAYER_GROUPS, 
                    Some(crate::game::simple::behaviours::projectile::ProjectileKnockbackType::Repulsion { 
                        center: context.location.0, 
                        strength: knockback_strength 
                    })
                ));
            },
            SpawnType::Missile {  } => todo!(),
            SpawnType::Lightning {  } => todo!(),
        }
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
