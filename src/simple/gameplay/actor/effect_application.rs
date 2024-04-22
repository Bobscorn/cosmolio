use bevy::prelude::*;

use crate::simple::{consts::SERVER_STR, gameplay::Position};

use super::effect::*;
use super::{ActorContext, DamageEvent};

pub struct ActorReference<'a>
{
    pub entity: Entity,
    pub context: &'a mut ActorContext,
    pub location: &'a mut Position,
}

pub struct EffectContextWorldAccess<'a, 'b, 'c>
{
    pub commands: &'a mut Commands<'b, 'c>,
    pub damage_instances: &'a mut Vec<DamageEvent>,
}

// All values needed for applying an effect
pub struct ActorEffectContext<'a, 'b, 'c, 'd>
{
    pub world_access: &'a mut EffectContextWorldAccess<'b, 'c, 'd>,
    pub actor: &'a mut ActorReference<'b>,
}

// Possibly move to another file
pub trait SerializeInto<T>
{
    fn serialize_into(&self) -> T;
}

// All values needed for applying a damage effect
pub struct ActorDamageEffectContext<'a, 'b, 'c, 'd>
{
    pub world_access: &'a mut EffectContextWorldAccess<'b, 'c, 'd>,
    pub instigator: &'a mut ActorReference<'b>,
    pub victim: Option<&'a mut ActorReference<'b>>, // Victim will be None when an entity damages/heals itself
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

impl<'a, 'b, 'c, 'd> ActorDamageEffectContext<'a, 'b, 'c, 'd>
{
    // pub fn actor_values<'d>(&'d mut self, which_actor: DamageActor) -> (&'d mut &'a mut Commands<'b, 'c>, Entity, &'d mut &'a mut ActorContext, &'d mut &'a mut Position)
    pub fn actor_values<'e>(&'e mut self, which_actor: DamageActor) -> (&'e mut EffectContextWorldAccess<'b, 'c, 'd>, &'e mut ActorReference<'b>)
    {
        match which_actor
        {
            DamageActor::Instigator => (self.world_access, self.instigator),
            DamageActor::Victim => (self.world_access, match &mut self.victim { Some(e) => e, None => self.instigator }),
        }
    }

    /// Gets the victim actor reference.
    /// This will grab 
    pub fn get_victim<'e>(&'e mut self) -> &'e mut ActorReference<'b>
    {
        match &mut self.victim
        {
            Some(vic) => vic,
            None => &mut self.instigator,
        }
    }
}


// All values needed for applying an on kill effect
pub struct ActorKillEffectContext<'a, 'b, 'c, 'd>
{
    pub world_access: &'a mut EffectContextWorldAccess<'b, 'c, 'd>,
    pub instigator: &'a mut ActorReference<'b>,
    pub victim: &'a mut ActorReference<'b>,
}


// All values needed for applying an on kill effect
pub struct ActorDeathEffectContext<'a, 'b, 'c, 'd>
{
    pub world_access: &'a mut EffectContextWorldAccess<'b, 'c, 'd>,
    pub instigator: Option<&'a mut ActorReference<'b>>,
    pub victim: &'a mut ActorReference<'b>,
}

pub struct ActorOnHitEffectContext<'a, 'b, 'c, 'd>
{
    pub world_access: &'a mut EffectContextWorldAccess<'b, 'c, 'd>,
    pub instigator: &'a mut ActorReference<'b>,
    pub victim: Option<&'a mut ActorReference<'b>>,
    pub hit_location: Vec2,
}

// ^
// Effects
// Public Facing Effect Interface
// v

pub fn apply_on_ability_cast_effects<'a, 'b, 'c, 'd>(ability_type: ChildType, context: &mut ActorEffectContext<'a, 'b, 'c, 'd>)
{
    let mut effects = Vec::new();
    for effect_trigger in &mut context.actor.context.effects
    {
        let SerializedEffectTrigger::OnAbilityCast{ ability_type: ability_trigger_type, effect } = effect_trigger else { continue; };
        if *ability_trigger_type == ability_type
        {
            effects.push(effect.clone());
        }
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on ({:?}) ability cast effects", effects.len(), ability_type);
    }
    for effect in &mut effects
    {
        effect.apply_effect(context);
    }
}

pub fn apply_on_ability_hit_effects<'a, 'b, 'c, 'd>(ability_type: ChildType, context: &mut ActorOnHitEffectContext<'a, 'b, 'c, 'd>)
{
    let mut effects = Vec::new();
    for effect_trigger in &mut context.instigator.context.effects
    {
        let SerializedEffectTrigger::OnAbilityHit{ ability_type: ability_trigger_type, effect } = effect_trigger else { continue; };
        if *ability_trigger_type == ability_type
        {
            effects.push(effect.clone());
        }
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on ({:?}) ability hit effects", effects.len(), ability_type);
    }
    for effect in &mut effects
    {
        effect.apply_effect(context);
    }
}

pub fn apply_on_ability_end_effects<'a, 'b, 'c, 'd>(ability_type: ChildType, context: &mut ActorEffectContext<'a, 'b, 'c, 'd>)
{
    let mut effects = Vec::new();
    for effect_trigger in &mut context.actor.context.effects
    {
        let SerializedEffectTrigger::OnAbilityEnd{ ability_type: ability_trigger_type, effect } = effect_trigger else { continue; };
        if *ability_trigger_type == ability_type
        {
            effects.push(effect.clone());
        }
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on ({:?}) ability end effects", effects.len(), ability_type);
    }
    for effect in &mut effects
    {
        effect.apply_effect(context);
    }
}

pub fn apply_on_kill_effects<'a, 'b, 'c, 'd>(context: &mut ActorKillEffectContext<'a, 'b, 'c, 'd>)
{
    let mut effects = Vec::new();
    for effect_trigger in &mut context.instigator.context.effects
    {
        let SerializedEffectTrigger::OnKill(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on kill effects", effects.len());
    }
    for effect in &mut effects
    {
        effect.apply_effect(context);
    }
}

/// Applies 'on damage' effects on an actor and 'on receive damage' effects to that actor
/// Returns the new modified damage/healing to do
// pub fn apply_damage_to_self_effects

/// Applies the 'on damage' effects of an actor (via &mut ActorContext) to damage from the actor
pub fn apply_on_damage_effects<'a, 'b, 'c, 'd>(context: &mut ActorDamageEffectContext<'a, 'b, 'c, 'd>, base_damage: f32) -> f32
{
    trace_span!("on_damage_effects");
    let mut effects = Vec::new();
    for effect_trigger in &mut context.instigator.context.effects
    {
        let SerializedEffectTrigger::OnDoDamage(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on damage effects", effects.len());
    }
    let mut damage_to_do = base_damage;
    for effect in &mut effects
    {
        damage_to_do = effect.process_damage(context, DamageActor::Instigator, damage_to_do);
    }
    damage_to_do
}

pub fn apply_damage_done_effects<'a, 'b, 'c, 'd>(context: &mut ActorDamageEffectContext<'a, 'b, 'c, 'd>, base_damage: f32)
{
    trace_span!("damage_done_effects");
    let mut effects = Vec::new();
    for effect_trigger in &mut context.instigator.context.effects
    {
        let SerializedEffectTrigger::OnDamageDone(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    if effects.len() > 0
    {
        trace!("{SERVER_STR} Applying '{}' damage done effects", effects.len());
    }
    for effect in &mut effects
    {
        effect.apply_effect(context, DamageActor::Instigator, base_damage);
    }
}

/// Applies the 'on receive damage' effects of an actor (via &mut ActorContext) to received damage of the entity
pub fn apply_receive_damage_effects<'a, 'b, 'c, 'd>(context: &mut ActorDamageEffectContext<'a, 'b, 'c, 'd>, damage_in: f32) -> f32
{
    trace_span!("on_receive_damage_effects");
    let mut effects = Vec::new();
    for effect_trigger in &mut context.get_victim().context.effects
    {
        let SerializedEffectTrigger::OnReceiveDamage(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    if effects.len() > 0
    {
        trace!("{SERVER_STR} Applying '{}' on receive damage effects", effects.len());
    }
    let mut damage_out = damage_in;
    for effect in &mut effects
    {
        damage_out = effect.process_damage(context, DamageActor::Victim, damage_out);
    }
    damage_out
}

pub fn apply_damage_received_effects<'a, 'b, 'c, 'd>(context: &mut ActorDamageEffectContext<'a, 'b, 'c, 'd>, damage_in: f32)
{
    trace_span!("on_damage_received_effects");
    let mut effects = Vec::new();
    for effect_trigger in &mut context.get_victim().context.effects
    {
        let SerializedEffectTrigger::OnDamageReceived(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    if effects.len() > 0
    {
        trace!("{SERVER_STR} Applying '{}' on damage received effects", effects.len());
    }
    for effect in &mut effects
    {
        effect.apply_effect(context, DamageActor::Victim, damage_in);
    }
}

/// Applies the 'on death' effects of an actor (via &mut ActorContext)
pub fn apply_on_death_effects<'a, 'b, 'c, 'd>(context: &mut ActorDeathEffectContext<'a, 'b, 'c, 'd>)
{
    let mut effects: Vec<SerializedDeathEffect> = Vec::new();
    for effect_trigger in &mut context.victim.context.effects
    {
        let SerializedEffectTrigger::OnDeath(effect) = effect_trigger else { continue; };
        effects.push(effect.clone());
    }
    if effects.len() > 0
    {
        info!("{SERVER_STR} Applying '{}' on death effects", effects.len());
    }
    for effect in &mut effects
    {
        effect.apply_effect(context);
    }
}


#[cfg(test)]
mod tests
{
    use bevy::{ecs::system::CommandQueue, prelude::*};

    use super::*;
    use super::super::stats::{Stat, StatusEffect, StatModification};

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
        let test_dmg_effect = SerializedDamageChangeEffect::AddDamageEffect { amount: 5.0 };
        let mut my_actor = ActorContext::default();
        let mut my_other_actor = ActorContext::default();

        my_actor.stats.insert(Stat::Health, 50.0_f32);

        my_actor.effects.push(SerializedEffectTrigger::OnDeath(test_effect.into()));
        my_actor.effects.push(SerializedEffectTrigger::OnKill(test_effect.into()));
        my_actor.effects.push(SerializedEffectTrigger::OnAbilityCast{ ability_type: ChildType::Grenade, effect: test_effect.into() });
        my_actor.effects.push(SerializedEffectTrigger::OnAbilityHit{ ability_type: ChildType::Grenade, effect: test_effect.into() });
        my_actor.effects.push(SerializedEffectTrigger::OnAbilityEnd{ ability_type: ChildType::Grenade, effect: test_effect.into() });
        my_actor.effects.push(SerializedEffectTrigger::Periodically { remaining_period: 0.0_f32, period: 2.0_f32, effect: test_effect.into() });
        my_actor.effects.push(SerializedEffectTrigger::OnDoDamage(test_dmg_effect));
        my_actor.effects.push(SerializedEffectTrigger::OnDamageDone(test_effect.into()));
        my_actor.effects.push(SerializedEffectTrigger::OnReceiveDamage(test_dmg_effect));
        my_actor.effects.push(SerializedEffectTrigger::OnDamageReceived(test_effect.into()));
        
        let mut fake_position = Position(Vec2::ZERO);
        let mut fake_other_position = Position(Vec2::ZERO);
        let fake_world = World::new();
        let mut fake_cmd_queue = CommandQueue::default();
        let mut fake_commands = Commands::new(&mut fake_cmd_queue, &fake_world);

        let mut fake_damage_events = Vec::new();


        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);
        {
            let mut world_access = EffectContextWorldAccess { commands: &mut fake_commands, damage_instances: &mut fake_damage_events };
            let mut test_actor_ref = ActorReference { context: &mut my_actor, entity: Entity::PLACEHOLDER, location: &mut fake_position };
            let mut fake_context = ActorEffectContext {
                world_access: &mut world_access,
                actor: &mut test_actor_ref
            };

            apply_on_ability_cast_effects(ChildType::Grenade, &mut fake_context);
        }

        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);
        
        {
            let mut world_access = EffectContextWorldAccess { commands: &mut fake_commands, damage_instances: &mut fake_damage_events };
            let mut test_actor_ref = ActorReference { context: &mut my_actor, entity: Entity::PLACEHOLDER, location: &mut fake_position };
            let mut fake_context = ActorEffectContext {
                world_access: &mut world_access,
                actor: &mut test_actor_ref
            };

            apply_on_ability_end_effects(ChildType::Grenade, &mut fake_context);
        }

        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);
        {
            let mut world_access = EffectContextWorldAccess { commands: &mut fake_commands, damage_instances: &mut fake_damage_events };
            let mut test_actor_ref = ActorReference { context: &mut my_actor, entity: Entity::PLACEHOLDER, location: &mut fake_position };
            let mut fake_other_actor = ActorReference { context: &mut my_other_actor, entity: Entity::PLACEHOLDER, location: &mut fake_other_position };
            let mut fake_context = ActorKillEffectContext {
                world_access: &mut world_access,
                instigator: &mut test_actor_ref,
                victim: &mut fake_other_actor
            };

            apply_on_kill_effects(&mut fake_context);
        }

        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);

        {
            let mut world_access = EffectContextWorldAccess { commands: &mut fake_commands, damage_instances: &mut fake_damage_events };
            let mut test_actor_ref = ActorReference { context: &mut my_actor, entity: Entity::PLACEHOLDER, location: &mut fake_position };
            let mut fake_context = ActorDeathEffectContext {
                world_access: &mut world_access,
                instigator: None,
                victim: &mut test_actor_ref,
            };

            apply_on_death_effects(&mut fake_context);
        }

        assert_eq!(my_actor.status_effects.len(), 1);
        
        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);

        {
            let mut world_access = EffectContextWorldAccess { commands: &mut fake_commands, damage_instances: &mut fake_damage_events };
            let mut test_actor_ref = ActorReference { context: &mut my_actor, entity: Entity::PLACEHOLDER, location: &mut fake_position };
            let mut fake_other_actor = ActorReference { context: &mut my_other_actor, entity: Entity::PLACEHOLDER, location: &mut fake_other_position };
            let mut fake_context = ActorDamageEffectContext {
                world_access: &mut world_access,
                instigator: &mut test_actor_ref,
                victim: Some(&mut fake_other_actor),
            };
            let new_dmg = apply_on_damage_effects(&mut fake_context, 25.0);

            assert_eq!(new_dmg, 30.0_f32);

            apply_damage_done_effects(&mut fake_context, new_dmg);
        }

        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);

        {
            let mut world_access = EffectContextWorldAccess { commands: &mut fake_commands, damage_instances: &mut fake_damage_events };
            let mut test_actor_ref = ActorReference { context: &mut my_actor, entity: Entity::PLACEHOLDER, location: &mut fake_position };
            let mut fake_other_actor = ActorReference { context: &mut my_other_actor, entity: Entity::PLACEHOLDER, location: &mut fake_other_position };
            let mut fake_context = ActorDamageEffectContext {
                world_access: &mut world_access,
                instigator: &mut test_actor_ref,
                victim: Some(&mut fake_other_actor),
            };

            let new_dmg = apply_receive_damage_effects(&mut fake_context, 25.0);

            assert_eq!(new_dmg, 30.0_f32);

            apply_damage_received_effects(&mut fake_context, new_dmg);
        }
        assert_eq!(my_actor.status_effects.len(), 1);

        my_actor.status_effects.clear();
        assert_eq!(my_actor.status_effects.len(), 0);

        {
            let mut world_access = EffectContextWorldAccess { commands: &mut fake_commands, damage_instances: &mut fake_damage_events };
            let mut test_actor_ref = ActorReference { context: &mut my_actor, entity: Entity::PLACEHOLDER, location: &mut fake_position };
            let mut fake_context = ActorOnHitEffectContext {
                world_access: &mut world_access,
                instigator: &mut test_actor_ref,
                victim: None,
                hit_location: Vec2::ZERO,
            };

            apply_on_ability_hit_effects(ChildType::Grenade, &mut fake_context);
        }

        assert_eq!(my_actor.status_effects.len(), 1);
    }
}
