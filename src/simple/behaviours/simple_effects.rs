use std::sync::Arc;

use bevy::prelude::*;
use bevy_rapier2d::geometry::CollisionGroups;

use crate::simple::consts::{PLAYER_PROJECTILE_FILTER, PLAYER_PROJECTILE_GROUP, PLAYER_PROJECTILE_GROUPS, RANGED_MAX_MISSILE_ANGULAR_ACCELERATION};

use super::{
    damage::DamageKnockback, 
    effect::{
        ActorDamageEffectContext, 
        ActorEffectContext, 
        ActorOnHitEffectContext, 
        DamageActor, 
        DamageEffect, 
        DamageEvent,
        Effect, 
        EffectTrigger, 
        OnHitEffect, 
        OnKillEffect, 
        OnDeathEffect,
        SerializeInto, 
        SerializedActorEffect, 
        SerializedDamageEffect, 
        SerializedEffectTrigger, 
        SerializedKillEffect, 
        SerializedDeathEffect,
        SerializedOnHitEffect, 
        SpawnLocation, 
        SpawnType, 
        WrappedEffect
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
    fn process_damage(&self, context: &mut ActorDamageEffectContext, effect_owner: DamageActor) -> f32 {
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
    fn process_damage(&self, context: &mut ActorDamageEffectContext, effect_owner: DamageActor) -> f32 {
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
        context.actor.context.status_effects.push(self.status_effect);
    }
    fn describe(&self) -> String {
        format!("Inflicts a status effect that {}", self.status_effect.get_description())
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

pub struct AffectHealthEffect
{
    pub amount: f32,
}

impl SerializeInto<SerializedActorEffect> for AffectHealthEffect
{
    fn serialize_into(&self) -> SerializedActorEffect {
        SerializedActorEffect::AffectHealth(self.amount)
    }
}

impl Effect for AffectHealthEffect
{
    fn apply_effect(&self, context: &mut ActorEffectContext) {
        context.world_access.damage_instances.push(DamageEvent { damage: -self.amount, instigator: context.actor.entity, victim: context.actor.entity });
    }
    fn describe(&self) -> String {
        match self.amount
        {
            0.0 => "Does nothing".into(),
            dmg if dmg < 0.0 => format!("Inflict {dmg} damage"),
            heal => format!("Inflict {} damage (heal for {})", heal, -heal),
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
                CollisionGroups { memberships: PLAYER_PROJECTILE_GROUP, filters: PLAYER_PROJECTILE_FILTER }, // TODO: more flexible version of this!
                Some(DamageKnockback::Repulsion { 
                    center: location,
                    strength: knockback_strength 
                })
            ));
        },
        SpawnType::Missile { damage, speed, acceleration, knockback_strength } => 
        {
            commands.spawn(MissileReplicationBundle::new(
                Missile { max_acceleration: acceleration, max_speed: speed, max_angular_acceleration: RANGED_MAX_MISSILE_ANGULAR_ACCELERATION, owner },
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
        do_spawn_object(context.world_access.commands, self.spawn_type, context.actor.location.0, context.actor.entity);
    }
    fn describe(&self) -> String {
        match self.spawn_type
        {
            SpawnType::Explosion { radius, damage, knockback_strength } =>
            {
                format!("Spawns an Explosion with {radius} radius, {damage} damage, and {knockback_strength} knockback")
            },
            SpawnType::Lightning {  } => todo!("implement lightning spawning"),
            SpawnType::Missile { damage, speed, acceleration, knockback_strength } => 
                format!("Spawns a Missile with speed: {speed} u/s, acceleration: {acceleration} u/s, damage: {damage}, and knockback strength: {knockback_strength}"),
        }
    }
}

///
/// Spawns an object (similar to `SpawnEffect`) at the location an ability hits something
pub struct SpawnAtHitEffect
{
    pub which_actor: DamageActor, // Only required when used as a DamageEffect
    pub spawn_type: SpawnType,
}

impl SerializeInto<SerializedOnHitEffect> for SpawnAtHitEffect
{
    fn serialize_into(&self) -> SerializedOnHitEffect {
        SerializedOnHitEffect::SpawnEffectAtHitLocation{ spawn_type: self.spawn_type }
    }
}

impl SerializeInto<SerializedDamageEffect> for SpawnAtHitEffect
{
    fn serialize_into(&self) -> SerializedDamageEffect {
        SerializedDamageEffect::SpawnObjectAt { which_actor: self.which_actor, spawn_type: self.spawn_type }
    }
}

impl OnHitEffect for SpawnAtHitEffect
{
    fn apply_effect(&self, context: &mut ActorOnHitEffectContext)
    {
        do_spawn_object(context.world_access.commands, self.spawn_type, context.hit_location, context.instigator.entity);
    }
    fn describe(&self) -> String {
        match self.spawn_type
        {
            SpawnType::Explosion { radius, damage, knockback_strength } =>
                format!("Spawns an Explosion (at the hit point) with {radius} radius, {damage} damage, and {knockback_strength} knockback"),
            SpawnType::Lightning {  } => 
                todo!("implement lightning spawning"),
            SpawnType::Missile { speed, damage, acceleration, knockback_strength } => 
                format!("Spawns a Missile (at the hit point) with speed: {speed} u/s, acceleration: {acceleration} u/s, damage: {damage}, and knockback strength: {knockback_strength}"),
        }
    }
}

impl DamageEffect for SpawnAtHitEffect
{
    fn process_damage(&self, context: &mut ActorDamageEffectContext, _effect_owner: DamageActor) -> f32
    {
        let dmg = context.damage;
        let (coms, act) = context.actor_values(self.which_actor);
        do_spawn_object(
            coms.commands, 
            self.spawn_type,
            act.location.0,
            act.entity
        );
        dmg
    }
    fn describe(&self) -> String {
        let spawn_place: String = match self.which_actor
        {
            DamageActor::Instigator => "at the Instigator".into(),
            DamageActor::Victim => "at the Victim".into(),
        };
        match self.spawn_type
        {
            SpawnType::Explosion { radius, damage, knockback_strength } =>
                format!("Spawns an Explosion ({spawn_place}) with {radius} radius, {damage} damage, and {knockback_strength} knockback"),
            SpawnType::Lightning {  } => 
                todo!("implement lightning spawning"),
            SpawnType::Missile { speed, damage, acceleration, knockback_strength } => 
                format!("Spawns a Missile ({spawn_place}) with speed: {speed} u/s, acceleration: {acceleration} u/s, damage: {damage}, and knockback strength: {knockback_strength}"),
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
            },
            SerializedActorEffect::AffectHealth(amount) =>
            {
                Arc::new(AffectHealthEffect{ amount: *amount })
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
            SerializedDamageEffect::SpawnObjectAt { which_actor, spawn_type } =>
            {
                Arc::new(SpawnAtHitEffect{ which_actor: *which_actor, spawn_type: *spawn_type })
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
                Arc::new(SpawnAtHitEffect{ which_actor: DamageActor::Victim, spawn_type: *spawn_type })
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

impl SerializedDeathEffect
{
    pub fn instantiate(&self) -> Arc<dyn OnDeathEffect>
    {
        match self
        {
            &SerializedDeathEffect::RegularEffect { effect } =>
            {
                Arc::new(WrappedEffect { effect: effect.instantiate() })
            }
        }
    }
}
