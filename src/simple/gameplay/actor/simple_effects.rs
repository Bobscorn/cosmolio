use bevy::prelude::*;
use bevy_rapier2d::geometry::CollisionGroups;

use crate::simple::consts::{PLAYER_PROJECTILE_FILTER, PLAYER_PROJECTILE_GROUP, PLAYER_PROJECTILE_GROUPS, RANGED_MAX_MISSILE_ANGULAR_ACCELERATION};

use super::{
    DamageKnockback,
    DamageEvent,
    effect_application::{
        ActorDamageEffectContext, 
        ActorEffectContext, 
        ActorOnHitEffectContext, 
        ActorDeathEffectContext,
        ActorKillEffectContext,
    },
    effect::{
        DamageActor,
        SerializedActorEffect, 
        SerializedDamageChangeEffect, 
        SerializedDamageViewEffect,
        SerializedKillEffect, 
        SerializedDeathEffect,
        SerializedOnHitEffect, 
        SpawnType,
    }, 
    stats::StatusEffect,
    super::objects::{
        ExplosionReplicationBundle, 
        Missile, 
        MissileReplicationBundle,
    }, 
};






impl SerializedActorEffect
{
    pub fn apply_effect(&mut self, context: &mut ActorEffectContext)
    {
        match self
        {
            Self::AffectHealth(h) => {
                context.world_access.damage_instances.push(DamageEvent { instigator: context.actor.entity, victim: context.actor.entity, damage: -(*h) });
            },
            Self::InflictStatusEffect(s) => {
                context.actor.context.status_effects.push(*s);
            },
            Self::SpawnEffect(spawn_type, _pos) => {
                do_spawn_object(context.world_access.commands, *spawn_type, context.actor.location.0, context.actor.entity);
            }
        }
    }
    pub fn describe(&self) -> String
    {
        match self
        {
            Self::AffectHealth(h) => describe_affect_health_effect(*h),
            Self::InflictStatusEffect(s) => describe_inflict_status_effect(s),
            Self::SpawnEffect(spawn_type, _location) => describe_spawn_object(spawn_type),
        }
    }
}

impl SerializedDamageChangeEffect
{
    pub fn process_damage(&mut self, context: &mut ActorDamageEffectContext, _which_actor: DamageActor, damage_in: f32) -> f32
    {
        match self
        {
            Self::AddDamageEffect { amount } => {
                return damage_in + *amount;
            },
            Self::MultiplyDamageEffect { factor } => {
                return damage_in * (*factor);
            },
        }
    }
    pub fn describe(&self) -> String
    {
        match self
        {
            Self::AddDamageEffect { amount } => describe_damage_add(*amount),
            Self::MultiplyDamageEffect { factor } => describe_damage_factor(*factor),
        }
    }
}

impl SerializedDamageViewEffect
{
    pub fn apply_effect(&mut self, context: &mut ActorDamageEffectContext, dmg_actor: DamageActor, damage_done: f32)
    {
        trace_span!("damage_view_effect");
        trace!(dmg_actor = ?dmg_actor, damage_done = %damage_done, effect = ?self, "doing damage view effect");
        match self
        {
            Self::EveryXDamageEffect { accumulated_damage, damage_threshold, which_actor, effect } => 
            {
                trace!(
                    accumulated_damage = ?accumulated_damage, 
                    damage_threshold = ?damage_threshold, 
                    which_actor = ?which_actor, 
                    inner_effect = ?effect,
                    "Doing Every X Damage effect"
                );
                *accumulated_damage += damage_done.max(0.0);
                if accumulated_damage >= damage_threshold
                {
                    trace!("Threshold high enough, doing effect");
                    *accumulated_damage -= *damage_threshold;
                    let (world_access, actor) = context.actor_values(*which_actor);
                    effect.apply_effect(&mut ActorEffectContext {
                        world_access,
                        actor
                    });
                }
            },
            Self::EveryXHealedEffect { accumulated_healing, healing_threshold, which_actor, effect } => 
            {
                trace!(
                    accumulated_healing = ?accumulated_healing, 
                    healing_threshold = ?healing_threshold, 
                    which_actor = ?which_actor, 
                    inner_effect = ?effect,
                    "Doing Every X Healed effect"
                );
                *accumulated_healing -= damage_done.min(0.0);
                if accumulated_healing >= healing_threshold
                {
                    trace!("Threshold high enough, doing effect");
                    *accumulated_healing -= *healing_threshold;
                    let (world_access, actor) = context.actor_values(*which_actor);
                    effect.apply_effect(&mut ActorEffectContext {
                        world_access,
                        actor
                    });
                }
            },
            Self::SpawnObjectAt { which_actor, spawn_type } =>
            {
                trace!(obj_actor = ?which_actor, spawn_type = ?spawn_type, "Doing SpawnObjectAt effect");
                let (world_access, actor) = context.actor_values(*which_actor);
                do_spawn_object(world_access.commands, *spawn_type, actor.location.0, actor.entity);
            },
            Self::RegularEffect { effect } =>
            {
                trace!(inner_effect = ?effect, "Doing regular effect");
            }
        }
    }

    pub fn describe(&self) -> String
    {
        match self
        {
            Self::EveryXDamageEffect { 
                accumulated_damage, damage_threshold, which_actor, effect 
            } => describe_every_x_effect("damage", *damage_threshold, which_actor, effect),
            Self::EveryXHealedEffect { 
                accumulated_healing, healing_threshold, which_actor, effect 
            } => describe_every_x_effect("healing", *healing_threshold, which_actor, effect),
            Self::SpawnObjectAt { which_actor, spawn_type } => format!("At the {}: {}", which_actor.as_str(), describe_spawn_object(spawn_type)),
            Self::RegularEffect { effect } => effect.describe(),
        }
    }
}

impl SerializedDeathEffect
{
    pub fn apply_effect(&mut self, context: &mut ActorDeathEffectContext) -> bool
    {
        trace_span!("death_effect");
        trace!(effect = ?self, "doing death effect");
        match self
        {
            Self::RegularEffect { effect } =>
            {
                trace!(inner_effect = ?effect, "doing regular effect");
                effect.apply_effect(&mut ActorEffectContext {
                    world_access: context.world_access,
                    actor: context.victim
                });
            }
        };

        false
    }
    pub fn describe(&self) -> String
    {
        match self
        {
            Self::RegularEffect { effect } => effect.describe(),
        }
    }
}

impl SerializedKillEffect
{
    pub fn apply_effect(&mut self, context: &mut ActorKillEffectContext)
    {
        trace_span!("on_kill_effect");
        trace!(effect = ?self, "doing kill effect");
        match self
        {
            Self::RegularEffect { effect } =>
            {
                trace!(inner_effect = ?effect, "doing regular effect");
                effect.apply_effect(&mut ActorEffectContext {
                    world_access: context.world_access,
                    actor: context.instigator
                });
            }
        }
    }
    pub fn describe(&self) -> String
    {
        match self
        {
            Self::RegularEffect { effect } => effect.describe(),
        }
    }
}

impl SerializedOnHitEffect
{
    pub fn apply_effect(&mut self, context: &mut ActorOnHitEffectContext)
    {
        trace_span!("on_hit_effect");
        trace!(effect = ?self, "doing on hit effect");
        match self
        {
            Self::SpawnEffectAtHitLocation { spawn_type } =>
            {
                trace!(spawn_type = ?spawn_type, "doing spawn effect at hit location");
                do_spawn_object(context.world_access.commands, *spawn_type, context.hit_location, context.instigator.entity);
            },
            Self::RegularEffect { effect } =>
            {
                trace!(inner_effect = ?effect, "doing regular effect");
                effect.apply_effect(&mut ActorEffectContext { world_access: context.world_access, actor: context.instigator });
            }
        }
    }
    pub fn describe(&self) -> String
    {
        match self
        {
            Self::SpawnEffectAtHitLocation { spawn_type } => describe_spawn_at_hit(spawn_type),
            Self::RegularEffect { effect } => effect.describe(),
        }
    }
}



fn describe_damage_factor(factor: f32) -> String {
    if factor == -1.0 { String::from("gets inverted (multiply by -1)") }
    else if factor == 1.0 { String::from("is not changed") }
    else if factor >= 0.0 && factor < 1.0 { format!("is decreased by {0}%", (1.0 - factor) * 100.0) }
    else if factor > 1.0 { format!("is increased by {0}%", (factor - 1.0) * 100.0) }
    else { format!("is multiplied by {0}", factor) }
}

fn describe_damage_add(amount: f32) -> String {
    if amount == 0.0 { "is not changed".into() }
    else if amount < 0.0 { format!("is decreased by {0}", -amount) }
    else { format!("is increased by {0}", amount) }
}

fn describe_inflict_status_effect(status_effect: &StatusEffect) -> String {
    format!("Inflicts a status effect that {}", status_effect.get_description())
}

fn describe_affect_health_effect(amount: f32) -> String {
    match amount
    {
        0.0 => "Does nothing".into(),
        dmg if dmg < 0.0 => format!("Inflict {} damage", -dmg),
        heal => format!("Heal {} damage (damage for {})", heal, -heal),
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

fn describe_spawn_object(spawn_type: &SpawnType) -> String {
    match spawn_type
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

fn describe_spawn_at_hit(spawn_type: &SpawnType) -> String {
    match spawn_type
    {
        SpawnType::Explosion { radius, damage, knockback_strength } =>
            format!("Spawns an Explosion (at the hit point) with {radius} radius, {damage} damage, and {knockback_strength} knockback"),
        SpawnType::Lightning {  } => 
            todo!("implement lightning spawning"),
        SpawnType::Missile { speed, damage, acceleration, knockback_strength } => 
            format!("Spawns a Missile (at the hit point) with speed: {speed} u/s, acceleration: {acceleration} u/s, damage: {damage}, and knockback strength: {knockback_strength}"),
    }
}

fn describe_every_x_effect(heal_dmg_str: &str, threshold: f32, which_actor: &DamageActor, effect: &SerializedActorEffect) -> String
{
    format!("Every {threshold} points of {heal_dmg_str} done, apply effect on the {} that: {}", which_actor.as_str(), effect.describe())
}
