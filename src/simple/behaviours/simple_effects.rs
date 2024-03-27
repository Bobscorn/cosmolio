use std::sync::Arc;

use bevy::prelude::*;
use bevy_rapier2d::geometry::CollisionGroups;

use crate::simple::consts::{PLAYER_PROJECTILE_FILTER, PLAYER_PROJECTILE_GROUP, PLAYER_PROJECTILE_GROUPS};

use super::{
    damage::DamageKnockback, effect::{
        ActorDamageEffectContext, ActorEffectContext, ActorOnHitEffectContext, DamageEffect, Effect, EffectTrigger, OnHitEffect, OnKillEffect, SerializeInto, SerializedActorEffect, SerializedDamageEffect, SerializedEffectTrigger, SerializedKillEffect, SerializedOnHitEffect, SpawnLocation, SpawnType, WrappedEffect
    }, explosion::ExplosionReplicationBundle, missile::{Missile, MissileReplicationBundle}, stats::StatusEffect
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
    fn describe(&self) -> String {
        if self.factor == -1.0 { String::from("invert damage dealt (multiply by -1)") }
        else if self.factor == 1.0 { String::from("do nothing") }
        else if self.factor >= 0.0 && self.factor < 1.0 { format!("decrease damage dealt by {0}%", (1.0 - self.factor) * 100.0) }
        else if self.factor > 1.0 { format!("increase damage dealt by {0}%", (self.factor - 1.0) * 100.0) }
        else { format!("multiply damage dealt by {0}", self.factor) }
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
    fn describe(&self) -> String {
        if self.amount == 0.0 { "do nothing".into() }
        else if self.amount < 0.0 { format!("decrease damage dealt by {0}", -self.amount) }
        else { format!("increase damage dealt by {0}", self.amount) }
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
    fn describe(&self) -> String {
        todo!();
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

pub fn do_spawn_object(commands: &mut Commands, spawn_type: SpawnType, location: Vec2, owner: Entity)
{
    match spawn_type
    {
        SpawnType::Explosion { radius, damage, knockback_strength } => 
        {
            commands.spawn(ExplosionReplicationBundle::new(
                owner,
                radius, 
                knockback_strength, 
                location, 
                damage, 
                CollisionGroups { memberships: PLAYER_PROJECTILE_GROUP, filters: PLAYER_PROJECTILE_FILTER }, 
                Some(DamageKnockback::Repulsion { 
                    center: location,
                    strength: knockback_strength 
                })
            ));
        },
        SpawnType::Missile { damage, speed, knockback_strength } => 
        {
            commands.spawn(MissileReplicationBundle::new(
                Missile::from_owner(owner),
                location,
                Vec2::ZERO,
                damage,
                PLAYER_PROJECTILE_GROUPS, // TODO: more flexible version of this!
                Some(DamageKnockback::RepulsionFromSelf { strength: knockback_strength }),
            ));
        },
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
        do_spawn_object(context.commands, self.spawn_type, context.location.0, context.actor_entity);
    }
    fn describe(&self) -> String {
        match self.spawn_type
        {
            SpawnType::Explosion { radius, damage, knockback_strength } =>
            {
                format!("Spawns an Explosion with {radius} radius, {damage} damage, and {knockback_strength} knockback")
            },
            SpawnType::Lightning {  } => todo!("implement lightning spawning"),
            SpawnType::Missile { damage, speed, knockback_strength } => 
                format!("Spawns a Missile of speed {speed}, doing damage {damage}, and knocking back with strength {knockback_strength}"),
        }
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
        do_spawn_object(context.commands, self.spawn_type, context.hit_location, context.instigator_entity);
    }
    fn describe(&self) -> String {
        match self.spawn_type
        {
            SpawnType::Explosion { radius, damage, knockback_strength } =>
            {
                format!("Spawns an Explosion (at the hit point) with {radius} radius, {damage} damage, and {knockback_strength} knockback")
            },
            SpawnType::Lightning {  } => todo!("implement lightning spawning"),
            SpawnType::Missile { speed, damage, knockback_strength } => 
                format!("Spawns a Missile (at hit point) with speed {speed} u/s doing {damage} damage and knocking back with strength {knockback_strength}"),
        }
    }
}


impl SerializedEffectTrigger
{
    pub fn instantiate(&self) -> EffectTrigger
    {
        match self
        {
            &SerializedEffectTrigger::OnDamage(effect) =>
            {
                EffectTrigger::OnDamage(effect.instantiate())
            },
            &SerializedEffectTrigger::Periodically { remaining_period, period, effect } =>
            {
                EffectTrigger::Periodically { remaining_period, period, effect: effect.instantiate() }
            },
            &SerializedEffectTrigger::OnKill(effect) =>
            {
                EffectTrigger::OnKill(effect.instantiate())
            },
            &SerializedEffectTrigger::OnDeath(effect) =>
            {
                EffectTrigger::OnDeath(effect.instantiate())
            },
            &SerializedEffectTrigger::OnReceiveDamage(effect) =>
            {
                EffectTrigger::OnReceiveDamage(effect.instantiate())
            },
            &SerializedEffectTrigger::OnAbilityCast { ability_type, effect } =>
            {
                EffectTrigger::OnAbilityCast { ability_type: ability_type, effect: effect.instantiate() }
            },
            &SerializedEffectTrigger::OnAbilityHit { ability_type, effect } =>
            {
                EffectTrigger::OnAbilityHit { ability_type: ability_type, effect: effect.instantiate() }
            },
            &SerializedEffectTrigger::OnAbilityEnd { ability_type, effect } =>
            {
                EffectTrigger::OnAbilityEnd { ability_type: ability_type, effect: effect.instantiate() }
            }
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

impl SerializedKillEffect
{
    pub fn instantiate(&self) -> Arc<dyn OnKillEffect>
    {
        match self
        {
            &SerializedKillEffect::RegularEffect { effect } => 
            {
                Arc::new(WrappedEffect { effect: effect.instantiate() })
            }
        }
    }
}
