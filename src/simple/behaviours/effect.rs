use std::sync::Arc;

use bevy::{prelude::*, utils::HashMap};

use serde::{Deserialize, Serialize};

use super::stats::*;
use crate::simple::common::Position;


#[derive(Clone, Copy, Debug, Event)]
pub struct DamageEvent
{
    pub instigator: Entity,
    pub victim: Entity,
    pub damage: f32
}

// ^
// Useful structs
// Actor stuff
// v
//   Temporary solution to allowing specific abilities to have certain effects
//   v

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Eq)]
pub enum ChildType
{
    Melee,
    Projectile,
    Missile,
    Grenade,
    ChildActor,
}

//   ^
//   Temporary brainstormed solution allowing specific abilities have certain effects


// Struct that contains all the data useful to an 'affectable' entity
#[derive(Component, Default)]
pub struct ActorContext
{
    pub effects: Vec<EffectTrigger>,
    pub status_effects: Vec<StatusEffect>,
    pub base_stats: Handle<BaseStats>,
    pub cur_stats: HashMap<Stat, f32>,
}

impl ActorContext
{
    pub fn base_stats_are_loaded(&self, base_stat_r: &Assets<BaseStats>) -> bool
    {
        base_stat_r.get(self.base_stats).is_some()
    }

    pub fn get_stat(&self, stat: Stat, base_stat_r: &Assets<BaseStats>) -> Option<f32>
    {
        if let Some(val) = self.cur_stats.get(&stat)
        {
            return Some(*val);
        }

        if let Some(base_stats) = base_stat_r.get(self.base_stats)
        {
            return base_stats.stats.get(&stat).map(|x| { *x })
        }

        None
    }

    pub fn get_or_create_stat(&self, stat: Stat, base_stat_r: &Assets<BaseStats>) -> f32
    {
        if let Some(val) = self.cur_stats.get(&stat)
        {
            return *val;
        }

        let base_stats = base_stat_r.get(self.base_stats).expect("expected base stats to have been loaded");
        if let Some(val) = base_stats.stats.get(&stat)
        {
            return *val;
        }
        self.base_stats.

        self.cur_stats.insert(stat, 0.0);
        0.0
    }

    pub fn set_stat(&self, stat: Stat, value: f32)
    {

    }
}

// Struct used for entites created by/for an actor, that should apply effects on behalf of that actor
#[derive(Component)]
pub struct ActorChild // TODO: rename to ActorAbility? Is there a use case for anything besides abilities?
{
    pub parent_actor: Entity,
    pub ability_type: ChildType
}

// ^
// Actor stuff
// Effect Serialization
// v

// These enums contain the serializable form of every effect
// It is used in combination with the Trigger enum (the Trigger enum stores instances of this enum)

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SpawnType
{
    Explosion{ radius: f32, damage: f32, knockback_strength: f32 },
    // Future ideas v
    Missile{  },
    Lightning{  },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SpawnLocation
{
    AtCaster,
    AtHitEnemy,

}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SerializedActorEffect
{
    InflictStatusEffect(StatusEffect),
    SpawnEffect(SpawnType, SpawnLocation),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SerializedDamageEffect
{
    MultiplyDamageEffect{ factor: f32 },
    AddDamageEffect{ amount: f32 },
    RegularEffect{ effect: SerializedActorEffect }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SerializedKillEffect
{
    RegularEffect{ effect: SerializedActorEffect },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub enum SerializedOnHitEffect
{
    SpawnEffectAtHitLocation{ spawn_type: SpawnType },
    RegularEffect{ effect: SerializedActorEffect },
}

// ^
// Effect Serialization
// Effect Traits
// v

// All values needed for applying an effect
pub struct ActorEffectContext<'a, 'b, 'c>
{
    pub commands: &'a mut Commands<'b, 'c>,
    pub actor: &'a mut ActorContext,
    pub location: &'a mut Position,
}

// Possibly move to another file
pub trait SerializeInto<T>
{
    fn serialize_into(&self) -> T;
}

pub trait Effect: SerializeInto<SerializedActorEffect> + Send + Sync
{
    fn apply_effect(&self, context: &mut ActorEffectContext);
}

// All values needed for applying a damage effect
pub struct ActorDamageEffectContext<'a, 'b, 'c>
{
    pub commands: &'a mut Commands<'b, 'c>,
    pub instigator_context: &'a mut ActorContext,
    pub instigator_location: &'a mut Position,
    pub victim_context: &'a mut ActorContext,
    pub victim_location: &'a mut Position,
    pub damage: f32
}

pub trait DamageEffect: SerializeInto<SerializedDamageEffect> + Send + Sync
{
    fn process_damage(&self, context: &mut ActorDamageEffectContext) -> f32;
}


// All values needed for applying an on kill effect
pub struct ActorOnKillEffectContext<'a, 'b, 'c>
{
    pub commands: &'a mut Commands<'b, 'c>,
    pub instigator_context: &'a mut ActorContext,
    pub instigator_location: &'a mut Position,
    pub victim_context: &'a mut ActorContext,
    pub victim_location: &'a mut Position,
}

pub trait OnKillEffect: SerializeInto<SerializedKillEffect> + Send + Sync
{
    fn apply_effect(&self, context: &mut ActorOnKillEffectContext);
}

pub struct ActorOnHitEffectContext<'a, 'b, 'c>
{
    pub commands: &'a mut Commands<'b, 'c>,
    pub instigator_context: &'a mut ActorContext,
    pub instigator_location: &'a mut Position,
    pub victim_context: Option<&'a mut ActorContext>,
    pub victim_location: Option<&'a mut Position>,
    pub hit_location: Vec2,
}

pub trait OnHitEffect: SerializeInto<SerializedOnHitEffect> + Send + Sync
{
    fn apply_effect(&self, context: &mut ActorOnHitEffectContext);
}

// ^
// Effects
// Triggers
// v

pub enum EffectTrigger
{
    OnDamage(Arc<dyn DamageEffect>),
    Periodically{ remaining_period: f32, period: f32, effect: Arc<dyn Effect> },
    OnKill(Arc<dyn OnKillEffect>),
    OnDeath(Arc<dyn Effect>),
    OnReceiveDamage(Arc<dyn DamageEffect>),
    OnAbilityCast{ ability_type: ChildType, effect: Arc<dyn Effect> },
    OnAbilityHit{ ability_type: ChildType, effect: Arc<dyn OnHitEffect> },
    OnAbilityEnd{ ability_type: ChildType, effect: Arc<dyn Effect> }, // TODO: better name/design for effect trigger when abilities 'end' (e.g. missiles/bullets hit, or melee hit finishes)
}

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq)]
pub enum SerializedEffectTrigger
{
    OnDamage(SerializedDamageEffect),
    Periodically{ remaining_period: f32, period: f32, effect: SerializedActorEffect },
    OnKill(SerializedKillEffect),
    OnDeath(SerializedActorEffect),
    OnReceiveDamage(SerializedDamageEffect),
    OnAbilityCast{ ability_type: ChildType, effect: SerializedActorEffect },
    OnAbilityHit{ ability_type: ChildType, effect: SerializedOnHitEffect },
    OnAbilityEnd{ ability_type: ChildType, effect: SerializedActorEffect },
}

// ^
// Triggers
// Convenience Implementations of Effect
// v

pub struct WrappedEffect
{
    pub effect: Arc<dyn Effect>
}

impl SerializeInto<SerializedDamageEffect> for WrappedEffect
{
    fn serialize_into(&self) -> SerializedDamageEffect {
        SerializedDamageEffect::RegularEffect { effect: self.effect.serialize_into() }
    }
}

impl SerializeInto<SerializedKillEffect> for WrappedEffect
{
    fn serialize_into(&self) -> SerializedKillEffect {
        SerializedKillEffect::RegularEffect { effect: self.effect.serialize_into() }
    }
}

impl SerializeInto<SerializedOnHitEffect> for WrappedEffect
{
    fn serialize_into(&self) -> SerializedOnHitEffect {
        SerializedOnHitEffect::RegularEffect { effect: self.effect.serialize_into() }
    }
}

impl DamageEffect for WrappedEffect
{
    fn process_damage(&self, context: &mut ActorDamageEffectContext) -> f32 {
        self.effect.apply_effect(&mut ActorEffectContext 
        { 
            commands: context.commands, 
            actor: context.instigator_context, 
            location: context.instigator_location 
        });
        context.damage
    }
}

impl OnKillEffect for WrappedEffect
{
    fn apply_effect(&self, context: &mut ActorOnKillEffectContext) {
        self.effect.apply_effect(&mut ActorEffectContext
        {
            commands: context.commands,
            actor: context.instigator_context,
            location: context.instigator_location
        });
    }
}

impl OnHitEffect for WrappedEffect
{
    fn apply_effect(&self, context: &mut ActorOnHitEffectContext) {
        self.effect.apply_effect(&mut ActorEffectContext
        {
            commands: context.commands,
            actor: context.instigator_context,
            location: context.instigator_location
        });
    }
}


// ^
// Convenience Implementations of Effect
// Generic Effects
// v

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



// ^
// Generic Effects
// Public Facing Effect Interface
// v

pub fn apply_on_ability_cast_effects<'a, 'b, 'c>(ability_type: ChildType, context: &mut ActorEffectContext<'a, 'b, 'c>)
{
    let mut effects: Vec<Arc<dyn Effect>> = Vec::new();
    for effect_trigger in &mut context.actor.effects
    {
        let EffectTrigger::OnAbilityCast{ ability_type: ability_trigger_type, effect } = effect_trigger else { continue; };
        if *ability_trigger_type == ability_type
        {
            effects.push(effect.clone());
        }
    }
    for effect in effects
    {
        effect.apply_effect(context);
    }
}

pub fn apply_on_ability_hit_effects<'a, 'b, 'c>(ability_type: ChildType, context: &mut ActorOnHitEffectContext<'a, 'b, 'c>)
{
    let mut effects: Vec<Arc<dyn OnHitEffect>> = Vec::new();
    for effect_trigger in &mut context.instigator_context.effects
    {
        let EffectTrigger::OnAbilityHit{ ability_type: ability_trigger_type, effect } = effect_trigger else { continue; };
        if *ability_trigger_type == ability_type
        {
            effects.push(effect.clone());
        }
    }
    for effect in effects
    {
        effect.apply_effect(context);
    }
}

pub fn apply_on_ability_end_effects<'a, 'b, 'c>(ability_type: ChildType, context: &mut ActorEffectContext<'a, 'b, 'c>)
{
    let mut effects: Vec<Arc<dyn Effect>> = Vec::new();
    for effect_trigger in &mut context.actor.effects
    {
        let EffectTrigger::OnAbilityEnd{ ability_type: ability_trigger_type, effect } = effect_trigger else { continue; };
        if *ability_trigger_type == ability_type
        {
            effects.push(effect.clone());
        }
    }
    for effect in effects
    {
        effect.apply_effect(context);
    }
}

pub fn apply_on_kill_effects<'a, 'b, 'c>(context: &mut ActorOnKillEffectContext<'a, 'b, 'c>)
{
    let mut effects: Vec<Arc<dyn OnKillEffect>> = Vec::new();
    for effect_trigger in &mut context.instigator_context.effects
    {
        let EffectTrigger::OnKill(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    for effect in effects
    {
        effect.apply_effect(context);
    }
}

/// Applies the 'on damage' effects of an actor (via &mut ActorContext) to damage from the actor
pub fn apply_on_damage_effects<'a, 'b, 'c>(context: &mut ActorDamageEffectContext<'a, 'b, 'c>) -> f32
{
    let mut effects: Vec<Arc<dyn DamageEffect>> = Vec::new();
    for effect_trigger in &mut context.instigator_context.effects
    {
        let EffectTrigger::OnDamage(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    for effect in effects
    {
        context.damage = effect.process_damage(context);
    }
    context.damage
}

/// Applies the 'on receive damage' effects of an actor (via &mut ActorContext) to received damage of the entity
pub fn apply_receive_damage_effects<'a, 'b, 'c>(context: &mut ActorDamageEffectContext<'a, 'b, 'c>) -> f32
{
    let mut effects: Vec<Arc<dyn DamageEffect>> = Vec::new();
    for effect_trigger in &mut context.instigator_context.effects
    {
        let EffectTrigger::OnReceiveDamage(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    for effect in effects
    {
        context.damage = effect.process_damage(context);
    }
    context.damage
}

/// Applies the 'on death' effects of an actor (via &mut ActorContext)
pub fn apply_on_death_effects<'a, 'b, 'c>(context: &mut ActorEffectContext<'a, 'b, 'c>)
{
    let mut effects: Vec<Arc<dyn Effect>> = Vec::new();
    for effect_trigger in &mut context.actor.effects
    {
        let EffectTrigger::OnDeath(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    for effect in effects
    {
        effect.apply_effect(context);
    }
}

// ^
// Public Facing Effect Interface
// Older implementation
// v

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Owner
{
    Player{ id: u64 },
    Enemy{ ent: Entity },
}

// TODO: rework the 'target' system, replace with 'effect' systems
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Target
{
    Caster,
    NearestAlly,
    NearestAllyExcludingCaster,
    NearestEnemy,

}


#[cfg(test)]
mod tests
{
    use std::sync::Arc;
    use bevy::{ecs::system::CommandQueue, prelude::*};

    use crate::simple::common::Position;

    use super::*;

    #[derive(Clone)]
    struct TestEffect
    {
        pub value: f32
    }

    impl TestEffect { 
        pub fn with_value(self, new_v: f32) -> Self { Self { value: new_v } } 
        pub fn change_actor_val(&self, actor: &mut ActorContext)
        {
            let existing_health = actor.stats[&Stat::Health];
            actor.stats.insert(Stat::Health, existing_health * self.value);
        }
    }

    impl SerializeInto<SerializedActorEffect> for TestEffect { 
        fn serialize_into(&self) -> SerializedActorEffect {
            panic!("Test effect can't actually serialize into effects");
        }
    }
    impl SerializeInto<SerializedOnHitEffect> for TestEffect {
        fn serialize_into(&self) -> SerializedOnHitEffect {
            panic!("Test effect can't actually serialize into effects");
        }
    }
    impl SerializeInto<SerializedDamageEffect> for TestEffect {
        fn serialize_into(&self) -> SerializedDamageEffect {
            panic!("Test effect can't actually serialize into effects");
        }
    }
    impl SerializeInto<SerializedKillEffect> for TestEffect {
        fn serialize_into(&self) -> SerializedKillEffect {
            panic!("Test effect can't actually serialize into effects");
        }
    }
    impl Effect for TestEffect {
        fn apply_effect<'a, 'b, 'c>(&self, context: &mut ActorEffectContext<'a, 'b, 'c>) {
            self.change_actor_val(context.actor);
        }
    }
    impl DamageEffect for TestEffect {
        fn process_damage(&self, context: &mut super::ActorDamageEffectContext) -> f32 {
            self.change_actor_val(context.instigator_context);
            0.0_f32
        }
    }
    impl OnHitEffect for TestEffect {
        fn apply_effect(&self, context: &mut super::ActorOnHitEffectContext) {
            self.change_actor_val(context.instigator_context);
        }
    }
    impl OnKillEffect for TestEffect {
        fn apply_effect(&self, context: &mut super::ActorOnKillEffectContext) {
            self.change_actor_val(context.instigator_context);
        }
    }

    #[test]
    fn test_effects()
    {
        let test_effect = TestEffect { value: 2.0_f32 };
        let mut my_actor = ActorContext::default();
        let mut my_other_actor = ActorContext::default();

        my_actor.stats.insert(Stat::Health, StatValue::new(50.0_f32));

        my_actor.effects.push(EffectTrigger::OnDeath(Arc::new(test_effect.clone())));
        my_actor.effects.push(EffectTrigger::OnKill(Arc::new(test_effect.clone())));
        my_actor.effects.push(EffectTrigger::OnAbilityCast{ ability_type: ChildType::Grenade, effect: Arc::new(test_effect.clone()) });
        my_actor.effects.push(EffectTrigger::OnAbilityHit{ ability_type: ChildType::Grenade, effect: Arc::new(test_effect.clone()) });
        my_actor.effects.push(EffectTrigger::OnAbilityEnd{ ability_type: ChildType::Grenade, effect: Arc::new(test_effect.clone()) });
        my_actor.effects.push(EffectTrigger::Periodically { remaining_period: 0.0_f32, period: 2.0_f32, effect: Arc::new(test_effect.clone()) });
        my_actor.effects.push(EffectTrigger::OnDamage(Arc::new(test_effect.clone())));
        my_actor.effects.push(EffectTrigger::OnReceiveDamage(Arc::new(test_effect.clone())));
        
        let mut fake_position = Position(Vec2::ZERO);
        let mut fake_other_position = Position(Vec2::ZERO);
        let fake_world = World::new();
        let mut fake_cmd_queue = CommandQueue::default();
        let mut fake_commands = Commands::new(&mut fake_cmd_queue, &fake_world);

        let mut fake_context = ActorEffectContext {
            actor: &mut my_actor,
            commands: &mut fake_commands,
            location: &mut fake_position
        };

        apply_on_death_effects(&mut fake_context);

        assert_eq!(my_actor.stats.get(&Stat::Health).unwrap().value, 50.0_f32 * 2.0_f32);

        let mut fake_context = ActorEffectContext {
            actor: &mut my_actor,
            commands: &mut fake_commands,
            location: &mut fake_position
        };

        apply_on_ability_cast_effects(ChildType::Grenade, &mut fake_context);

        assert_eq!(my_actor.stats.get(&Stat::Health).unwrap().value, 50.0_f32 * 2.0_f32 * 2.0_f32);

        let mut fake_context = ActorEffectContext {
            actor: &mut my_actor,
            commands: &mut fake_commands,
            location: &mut fake_position
        };

        apply_on_ability_end_effects(ChildType::Grenade, &mut fake_context);

        assert_eq!(my_actor.stats.get(&Stat::Health).unwrap().value, 50.0_f32 * 2.0_f32 * 2.0_f32 * 2.0_f32);

        let mut fake_context = ActorOnKillEffectContext {
            commands: &mut fake_commands,
            instigator_context: &mut my_actor,
            instigator_location: &mut fake_position,
            victim_context: &mut my_other_actor,
            victim_location: &mut fake_other_position
        };

        apply_on_kill_effects(&mut fake_context);

        assert_eq!(my_actor.stats.get(&Stat::Health).unwrap().value, 50.0_f32 * 2.0_f32 * 2.0_f32 * 2.0_f32 * 2.0_f32);
        
        my_actor.stats.insert(Stat::Health, StatValue::new(50.0_f32));
        let mut fake_context = ActorDamageEffectContext {
            commands: &mut fake_commands,
            instigator_context: &mut my_actor,
            instigator_location: &mut fake_position,
            victim_context: &mut my_other_actor,
            victim_location: &mut fake_other_position,
            damage: 25.0_f32
        };
        let new_dmg = apply_on_damage_effects(&mut fake_context);

        assert_eq!(new_dmg, 0.0_f32);
        assert_eq!(my_actor.stats.get(&Stat::Health).unwrap().value, 50.0_f32 * 2.0_f32);

        let mut fake_context = ActorDamageEffectContext {
            commands: &mut fake_commands,
            instigator_context: &mut my_actor,
            instigator_location: &mut fake_position,
            victim_context: &mut my_other_actor,
            victim_location: &mut fake_other_position,
            damage: 25.0_f32
        };

        let new_dmg = apply_receive_damage_effects(&mut fake_context);

        assert_eq!(new_dmg, 0.0_f32);
        assert_eq!(my_actor.stats.get(&Stat::Health).unwrap().value, 50.0_f32 * 2.0_f32 * 2.0_f32);

        my_actor.stats.insert(Stat::Health, StatValue::new(50.0_f32));
        let mut fake_context = ActorOnHitEffectContext {
            commands: &mut fake_commands,
            instigator_context: &mut my_actor,
            instigator_location: &mut fake_position,
            victim_context: None,
            victim_location: None,
            hit_location: Vec2::ZERO
        };

        apply_on_ability_hit_effects(ChildType::Grenade, &mut fake_context);

        assert_eq!(my_actor.stats.get(&Stat::Health).unwrap().value, 50.0_f32 * 2.0_f32);
    }
}
