use std::sync::Arc;

use bevy::{prelude::*, utils::HashMap};

use serde::{Deserialize, Serialize};

use super::{damage::Damage, stats::*};
use crate::simple::{common::Position, consts::SERVER_STR};


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


// Struct that contains all the data useful to an 'affectable' entity
#[derive(Component, Default, Serialize, Deserialize, Reflect)]
pub struct ActorContext
{
    pub effects: Vec<SerializedEffectTrigger>,
    pub status_effects: Vec<StatusEffect>,
    pub stats: HashMap<Stat, f32>,
}

impl ActorContext
{
    pub fn get_stat(&self, stat: &Stat) -> Option<f32>
    {
        self.stats.get(stat).map(|x| { *x })
    }

    pub fn get_or_create_stat(&mut self, stat: Stat) -> f32
    {
        if let Some(val) = self.stats.get(&stat)
        {
            return *val;
        }

        self.stats.insert(stat, 0.0);
        0.0
    }

    pub fn set_stat(&mut self, stat: Stat, value: f32)
    {
        self.stats.insert(stat, value);
    }
}

// Struct used for entites created by/for an actor, that should apply effects on behalf of that actor
#[derive(Component, Reflect)]
pub struct ActorChild // TODO: rename to ActorAbility? Is there a use case for anything besides abilities?
{
    pub parent_actor: Entity,
    pub ability_type: ChildType
}

/// This component stores the sensor entities (using bevy_rapier sensors) for an actor.
/// Expected to be attached to the same entity as a ActorContext.
#[derive(Component, Reflect)]
pub struct ActorSensors
{
    pub sensors: Vec<Entity>,
}

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

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedActorEffect
{
    InflictStatusEffect(StatusEffect),
    SpawnEffect(SpawnType, SpawnLocation),
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedDamageEffect
{
    MultiplyDamageEffect{ factor: f32 },
    AddDamageEffect{ amount: f32 },
    SpawnObjectAt{ which_actor: DamageActor, spawn_type: SpawnType },
    RegularEffect{ effect: SerializedActorEffect }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedKillEffect
{
    RegularEffect{ effect: SerializedActorEffect },
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum SerializedOnHitEffect
{
    SpawnEffectAtHitLocation{ spawn_type: SpawnType },
    RegularEffect{ effect: SerializedActorEffect },
}

impl Into<SerializedDamageEffect> for SerializedActorEffect
{
    fn into(self) -> SerializedDamageEffect {
        SerializedDamageEffect::RegularEffect { effect: self }
    }
}

impl Into<SerializedKillEffect> for SerializedActorEffect
{
    fn into(self) -> SerializedKillEffect {
        SerializedKillEffect::RegularEffect { effect: self }
    }
}

impl Into<SerializedOnHitEffect> for SerializedActorEffect
{
    fn into(self) -> SerializedOnHitEffect {
        SerializedOnHitEffect::RegularEffect { effect: self }
    }
}

// ^
// Effect Serialization
// Effect Traits
// v

// All values needed for applying an effect
pub struct ActorEffectContext<'a, 'b, 'c>
{
    pub commands: &'a mut Commands<'b, 'c>,
    pub actor_entity: Entity,
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
    // Return a description of the effect that fits into the sentences:
    // "When X occurs [description]"
    // "Applies an effect that will [description]"
    fn describe(&self) -> String; 
}

// All values needed for applying a damage effect
pub struct ActorDamageEffectContext<'a, 'b, 'c>
{
    pub commands: &'a mut Commands<'b, 'c>,
    pub instigator_entity: Entity,
    pub instigator_context: &'a mut ActorContext,
    pub instigator_location: &'a mut Position,
    pub victim_entity: Entity,
    pub victim_context: &'a mut ActorContext,
    pub victim_location: &'a mut Position,
    pub damage: f32
}

// impl<'a, 'b, 'c, 'd> ActorDamageEffectContext<'b, 'c, 'd>
// {
//     pub fn actor_values(&'a mut self, which_actor: DamageActor) -> (&'b mut Commands<'c, 'd>, Entity, &'b mut ActorContext, &'b mut Position)
//     {
//         match which_actor
//         {
//             DamageActor::Instigator => (self.commands, self.instigator_entity, &mut self.instigator_context, &mut self.instigator_location),
//             DamageActor::Victim => (self.commands, self.victim_entity, &mut self.victim_context, &mut self.victim_location),
//         }
//     }
// }

impl<'a, 'b, 'c> ActorDamageEffectContext<'a, 'b, 'c>
{
    pub fn actor_values<'d>(&'d mut self, which_actor: DamageActor) -> (&'d mut &'a mut Commands<'b, 'c>, Entity, &'d &'a mut ActorContext, &'d &'a mut Position)
    {
        match which_actor
        {
            DamageActor::Instigator => (&mut self.commands, self.instigator_entity, &self.instigator_context, &self.instigator_location),
            DamageActor::Victim => (&mut self.commands, self.victim_entity, &self.victim_context, &self.victim_location),
        }
    }
}

pub trait DamageEffect: SerializeInto<SerializedDamageEffect> + Send + Sync
{
    fn process_damage(&self, context: &mut ActorDamageEffectContext, effect_owner: DamageActor) -> f32;
    // Returns a description of what this effect does, that will fit in the sentences:
    // "Upon damaging a target [description]"
    // "Gain an OnDamage effect that will [description]"
    fn describe(&self) -> String;
}


// All values needed for applying an on kill effect
pub struct ActorOnKillEffectContext<'a, 'b, 'c>
{
    pub commands: &'a mut Commands<'b, 'c>,
    pub instigator_entity: Entity,
    pub instigator_context: &'a mut ActorContext,
    pub instigator_location: &'a mut Position,
    pub victim_entity: Entity,
    pub victim_context: &'a mut ActorContext,
    pub victim_location: &'a mut Position,
}

pub trait OnKillEffect: SerializeInto<SerializedKillEffect> + Send + Sync
{
    fn apply_effect(&self, context: &mut ActorOnKillEffectContext);
    // Describes the effect, implementors will return a description that fits into the sentences:
    // "Upon killing a target [description]"
    // "On dealing a killing blow [description]"
    fn describe(&self) -> String; 
}

pub struct ActorOnHitEffectContext<'a, 'b, 'c>
{
    pub commands: &'a mut Commands<'b, 'c>,
    pub instigator_entity: Entity,
    pub instigator_context: &'a mut ActorContext,
    pub instigator_location: &'a mut Position,
    pub victim_entity: Entity,
    pub victim_context: Option<&'a mut ActorContext>,
    pub victim_location: Option<&'a mut Position>,
    pub hit_location: Vec2,
}

pub trait OnHitEffect: SerializeInto<SerializedOnHitEffect> + Send + Sync
{
    fn apply_effect(&self, context: &mut ActorOnHitEffectContext);
    // Returns a description of what the effect will do, that will fit in the sentences:
    // "When hitting a target [description]"
    // "Gain an OnHit effect that will [description]"
    fn describe(&self) -> String;
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

#[derive(Clone, Debug, Copy, Serialize, Deserialize, PartialEq, Reflect)]
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
    fn process_damage(&self, context: &mut ActorDamageEffectContext, effect_owner: DamageActor) -> f32 {
        let dmg = context.damage;
        let (commands, ent, actor, location) = match effect_owner
        {
            DamageActor::Instigator => (&mut context.commands, context.instigator_entity, &mut context.instigator_context, &mut context.instigator_location),
            DamageActor::Victim => (&mut context.commands, context.victim_entity, &mut context.victim_context, &mut context.victim_location)
        };
        self.effect.apply_effect( 
            &mut ActorEffectContext 
            { 
                commands, 
                actor_entity: ent,
                actor,
                location
            },
        );
        dmg
    }
    fn describe(&self) -> String {
        self.effect.describe()
    }
}

impl OnKillEffect for WrappedEffect
{
    fn apply_effect(&self, context: &mut ActorOnKillEffectContext) {
        self.effect.apply_effect(&mut ActorEffectContext
        {
            commands: context.commands,
            actor_entity: context.instigator_entity,
            actor: context.instigator_context,
            location: context.instigator_location
        });
    }
    fn describe(&self) -> String {
        self.effect.describe()
    }
}

impl OnHitEffect for WrappedEffect
{
    fn apply_effect(&self, context: &mut ActorOnHitEffectContext) {
        self.effect.apply_effect(&mut ActorEffectContext
        {
            commands: context.commands,
            actor_entity: context.instigator_entity,
            actor: context.instigator_context,
            location: context.instigator_location
        });
    }
    fn describe(&self) -> String {
        self.effect.describe()
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
        let SerializedEffectTrigger::OnAbilityCast{ ability_type: ability_trigger_type, effect } = effect_trigger else { continue; };
        if *ability_trigger_type == ability_type
        {
            effects.push(effect.instantiate());
        }
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on ({:?}) ability cast effects", effects.len(), ability_type);
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
        let SerializedEffectTrigger::OnAbilityHit{ ability_type: ability_trigger_type, effect } = effect_trigger else { continue; };
        if *ability_trigger_type == ability_type
        {
            effects.push(effect.instantiate());
        }
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on ({:?}) ability hit effects", effects.len(), ability_type);
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
        let SerializedEffectTrigger::OnAbilityEnd{ ability_type: ability_trigger_type, effect } = effect_trigger else { continue; };
        if *ability_trigger_type == ability_type
        {
            effects.push(effect.instantiate());
        }
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on ({:?}) ability end effects", effects.len(), ability_type);
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
        let SerializedEffectTrigger::OnKill(effect) = effect_trigger else { continue; };
        effects.push(effect.instantiate());
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on kill effects", effects.len());
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
        let SerializedEffectTrigger::OnDamage(effect) = effect_trigger else { continue; };
        effects.push(effect.instantiate());
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on damage effects", effects.len());
    }
    for effect in effects
    {
        context.damage = effect.process_damage(context, DamageActor::Instigator);
    }
    context.damage
}

/// Applies the 'on receive damage' effects of an actor (via &mut ActorContext) to received damage of the entity
pub fn apply_receive_damage_effects<'a, 'b, 'c>(context: &mut ActorDamageEffectContext<'a, 'b, 'c>) -> f32
{
    let mut effects: Vec<Arc<dyn DamageEffect>> = Vec::new();
    for effect_trigger in &mut context.instigator_context.effects
    {
        let SerializedEffectTrigger::OnReceiveDamage(effect) = effect_trigger else { continue; };
        effects.push(effect.instantiate());
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on receive damage effects", effects.len());
    }
    for effect in effects
    {
        context.damage = effect.process_damage(context, DamageActor::Victim);
    }
    context.damage
}

/// Applies the 'on death' effects of an actor (via &mut ActorContext)
pub fn apply_on_death_effects<'a, 'b, 'c>(context: &mut ActorEffectContext<'a, 'b, 'c>)
{
    let mut effects: Vec<Arc<dyn Effect>> = Vec::new();
    for effect_trigger in &mut context.actor.effects
    {
        let SerializedEffectTrigger::OnDeath(effect) = effect_trigger else { continue; };
        effects.push(effect.instantiate());
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on death effects", effects.len());
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
    use bevy::{ecs::system::CommandQueue, prelude::*};

    use crate::simple::common::Position;

    use super::*;

    #[test]
    fn test_effects()
    {
        let test_effect = SerializedActorEffect::InflictStatusEffect(
            StatusEffect { 
                timeout: None, 
                modification: StatModification::Add { amount: 1.0 }, 
                stat: Stat::Health 
            }
        );
        let mut my_actor = ActorContext::default();
        let mut my_other_actor = ActorContext::default();

        my_actor.stats.insert(Stat::Health, 50.0_f32);

        my_actor.effects.push(SerializedEffectTrigger::OnDeath(test_effect));
        my_actor.effects.push(SerializedEffectTrigger::OnKill(test_effect.into()));
        my_actor.effects.push(SerializedEffectTrigger::OnAbilityCast{ ability_type: ChildType::Grenade, effect: test_effect.into() });
        my_actor.effects.push(SerializedEffectTrigger::OnAbilityHit{ ability_type: ChildType::Grenade, effect: test_effect.into() });
        my_actor.effects.push(SerializedEffectTrigger::OnAbilityEnd{ ability_type: ChildType::Grenade, effect: test_effect.into() });
        my_actor.effects.push(SerializedEffectTrigger::Periodically { remaining_period: 0.0_f32, period: 2.0_f32, effect: test_effect.into() });
        my_actor.effects.push(SerializedEffectTrigger::OnDamage(test_effect.into()));
        my_actor.effects.push(SerializedEffectTrigger::OnReceiveDamage(test_effect.into()));
        
        let mut fake_position = Position(Vec2::ZERO);
        let mut fake_other_position = Position(Vec2::ZERO);
        let fake_world = World::new();
        let mut fake_cmd_queue = CommandQueue::default();
        let mut fake_commands = Commands::new(&mut fake_cmd_queue, &fake_world);

        assert_eq!(my_actor.status_effects.len(), 0);
        let mut fake_context = ActorEffectContext {
            actor: &mut my_actor,
            actor_entity: Entity::PLACEHOLDER,
            commands: &mut fake_commands,
            location: &mut fake_position
        };


        apply_on_death_effects(&mut fake_context);

        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);
        let mut fake_context = ActorEffectContext {
            actor: &mut my_actor,
            actor_entity: Entity::PLACEHOLDER,
            commands: &mut fake_commands,
            location: &mut fake_position
        };

        apply_on_ability_cast_effects(ChildType::Grenade, &mut fake_context);

        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);
        let mut fake_context = ActorEffectContext {
            actor: &mut my_actor,
            actor_entity: Entity::PLACEHOLDER,
            commands: &mut fake_commands,
            location: &mut fake_position
        };

        apply_on_ability_end_effects(ChildType::Grenade, &mut fake_context);

        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);
        let mut fake_context = ActorOnKillEffectContext {
            commands: &mut fake_commands,
            instigator_entity: Entity::PLACEHOLDER,
            instigator_context: &mut my_actor,
            instigator_location: &mut fake_position,
            victim_context: &mut my_other_actor,
            victim_location: &mut fake_other_position,
            victim_entity: Entity::PLACEHOLDER,
        };

        apply_on_kill_effects(&mut fake_context);

        assert_eq!(my_actor.status_effects.len(), 1);
        
        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);
        let mut fake_context = ActorDamageEffectContext {
            commands: &mut fake_commands,
            instigator_entity: Entity::PLACEHOLDER,
            instigator_context: &mut my_actor,
            instigator_location: &mut fake_position,
            victim_context: &mut my_other_actor,
            victim_location: &mut fake_other_position,
            victim_entity: Entity::PLACEHOLDER,
            damage: 25.0_f32
        };
        let new_dmg = apply_on_damage_effects(&mut fake_context);

        assert_eq!(new_dmg, 25.0_f32);

        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);
        let mut fake_context = ActorDamageEffectContext {
            commands: &mut fake_commands,
            instigator_entity: Entity::PLACEHOLDER,
            instigator_context: &mut my_actor,
            instigator_location: &mut fake_position,
            victim_context: &mut my_other_actor,
            victim_location: &mut fake_other_position,
            victim_entity: Entity::PLACEHOLDER,
            damage: 25.0_f32
        };

        let new_dmg = apply_receive_damage_effects(&mut fake_context);

        assert_eq!(new_dmg, 25.0_f32);
        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);
        let mut fake_context = ActorOnHitEffectContext {
            commands: &mut fake_commands,
            instigator_entity: Entity::PLACEHOLDER,
            instigator_context: &mut my_actor,
            instigator_location: &mut fake_position,
            victim_context: None,
            victim_location: None,
            victim_entity: Entity::PLACEHOLDER,
            hit_location: Vec2::ZERO
        };

        apply_on_ability_hit_effects(ChildType::Grenade, &mut fake_context);

        assert_eq!(my_actor.status_effects.len(), 1);
    }
}
