use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::simple::common::Position;

use super::effect::{apply_on_damage_effects, apply_receive_damage_effects, ActorContext, ActorDamageEffectContext, DamageEvent};
use super::stats::{Stat, StatValue};



#[derive(Serialize, Deserialize)]
pub enum DamageKnockback
{
    Impulse(Vec2), // Push an object in a direction
    Repulsion{ center: Vec2, strength: f32 }, // Repel an object away from a point
    Attraction{ center: Vec2, strength: f32 }, // Attract an object towards a point
}

#[derive(Component, Serialize, Deserialize)]
pub struct Damage
{
    pub damage: f32,
    pub destroy_on_damage: bool,
    pub deal_damage_once: bool,
    pub knockback: Option<DamageKnockback>,
    pub did_damage: bool,
}

impl Damage
{
    pub fn new(damage: f32, destroy_on_damage: bool, deal_damage_once: bool, knockback: Option<DamageKnockback>) -> Self
    {
        Self
        {
            damage,
            destroy_on_damage,
            deal_damage_once,
            knockback,
            did_damage: false
        }
    }

    pub fn should_destroy(&self) -> bool
    {
        self.destroy_on_damage
    }

    pub fn new_typical_bullet(damage: f32) -> Self
    {
        Self
        {
            damage,
            destroy_on_damage: true,
            deal_damage_once: true,
            knockback: None,
            did_damage: false,
        }
    }

    pub fn new_typical_explosion(damage: f32) -> Self
    {
        Self
        {
            damage,
            destroy_on_damage: false,
            deal_damage_once: false,
            knockback: None,
            did_damage: false,
        }
    }

    pub fn new_typical_laser(damage: f32) -> Self
    {
        Self
        {
            damage, 
            destroy_on_damage: false,
            deal_damage_once: false,
            knockback: None,
            did_damage: false,
        }
    }

    pub fn new_typical_hitscan(damage: f32) -> Self
    {
        Self
        {
            damage,
            destroy_on_damage: false,
            deal_damage_once: true,
            knockback: None,
            did_damage: false,
        }
    }

    pub fn with_destroy_on_damage(&mut self, destroy_on_damage: bool) -> &mut Self
    {
        self.destroy_on_damage = destroy_on_damage;
        self
    }

    pub fn with_knockback(&mut self, knockback: Option<DamageKnockback>) -> &mut Self
    {
        self.knockback = knockback;
        self
    }
}



pub fn s_do_damage_events(
    mut commands: Commands,
    mut actor_lookup: Query<(&mut ActorContext, &mut Position)>,
    mut damage_events: EventReader<DamageEvent>,
) {
    for DamageEvent { instigator, victim, damage } in damage_events.read()
    {
        if instigator == victim
        {
            warn!("Entity {:?} trying to damage itself!?", instigator);
            continue;
        }
        let Ok(
            [(mut instigator_context, mut instigator_position), 
             (mut victim_context, mut victim_position)]
            ) = actor_lookup.get_many_mut([*instigator, *victim]) else { continue; };

        let mut context = ActorDamageEffectContext {
            commands: &mut commands,
            instigator_context: &mut instigator_context,
            instigator_location: &mut instigator_position,
            victim_context: &mut victim_context,
            victim_location: &mut victim_position,
            damage: *damage
        };
        let damage = apply_on_damage_effects(&mut context);
        context.damage = damage; // TODO: record damage stats?
        let damage_to_do = apply_receive_damage_effects(&mut context);
        let existing_health = *victim_context.stats.get(&Stat::Health).unwrap_or(&StatValue::new(0.0_f32));
        victim_context.stats.insert(Stat::Health, existing_health - damage_to_do);
    }
}
