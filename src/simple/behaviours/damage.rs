use bevy::prelude::*;
use serde::{Serialize, Deserialize};

use crate::simple::common::Position;

use super::effect::{apply_on_damage_effects, apply_receive_damage_effects, ActorContext, ActorReference, ActorDamageEffectContext, DamageEvent, EffectContextWorldAccess};
use super::stats::Stat;



#[derive(Serialize, Deserialize)]
pub enum DamageKnockback
{
    Impulse(Vec2), // Push an object in a direction
    Repulsion{ center: Vec2, strength: f32 }, // Repel an object away from a point (point in world space)
    Attraction{ center: Vec2, strength: f32 }, // Attract an object towards a point (point in world space)
    RepulsionFromSelf { strength: f32 }, // Repel an object away from this object (if it has a position)
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

fn calc_dmg_effects(
    damage_context: &mut ActorDamageEffectContext,
) -> f32 {

    let damage = apply_on_damage_effects(damage_context);
    damage_context.damage = damage; // TODO: record damage stats?
    apply_receive_damage_effects(damage_context)
}

fn do_dmg(
    damage_to_do: f32,
    actor_context: &mut ActorContext,
    instigator: Entity,
) {
    let existing_health = *actor_context.stats.get(&Stat::Health).unwrap_or(&0.0_f32);
    let new_health = existing_health - damage_to_do;
    actor_context.stats.insert(Stat::Health, new_health);
    actor_context.last_damage_source = Some(super::effect::DamageSource::Actor(instigator));
}

fn dmg_events(
    coms: &mut Commands,
    actor_lookup: &mut Query<(&mut ActorContext, &mut Position)>,
    read_events: &Vec<DamageEvent>,
    write_events: &mut Vec<DamageEvent>,
) {
    for DamageEvent { instigator, victim, damage } in read_events
    {
        if instigator == victim
        {
            let Ok((mut actor_context, mut actor_pos)) = actor_lookup.get_mut(*instigator) else { 
                error!("Could not find actor comps for entity {:?} in damage event!", instigator); 
                continue;
            };

            let mut damage_context = ActorDamageEffectContext
            {
                world_access: &mut EffectContextWorldAccess { commands: coms, damage_instances: write_events },
                instigator: &mut ActorReference { entity: *instigator, context: &mut actor_context, location: &mut actor_pos },
                victim: None,
                damage: *damage,
            };

            let dmg = calc_dmg_effects(&mut damage_context);
            do_dmg(dmg, &mut actor_context, *instigator);

            continue;
        }
        let Ok(
            [(mut instigator_context, mut instigator_position), 
            (mut victim_context, mut victim_position)]
            ) = actor_lookup.get_many_mut([*instigator, *victim]) 
            else 
            { 
                error!("Did not find actor comps in query from damage event!"); 
                trace!("Instigator: {:?}, Victim: {:?}", instigator, victim);
                continue; 
            };

        
        let mut victim = ActorReference { entity: *victim, context: &mut victim_context, location: &mut victim_position };

        let mut damage_context = ActorDamageEffectContext 
        {
            world_access: &mut EffectContextWorldAccess { commands: coms, damage_instances: write_events },
            instigator: &mut ActorReference { entity: *instigator, context: &mut instigator_context, location: &mut instigator_position },
            victim: Some(&mut victim),
            damage: *damage
        };

        let dmg = calc_dmg_effects(&mut damage_context);
        do_dmg(dmg, &mut victim_context, *instigator);
    }
}

pub fn s_do_damage_events(
    mut commands: Commands,
    mut actor_lookup: Query<(&mut ActorContext, &mut Position)>,
    mut damage_events: EventReader<DamageEvent>,
) {
    let mut damage_es_1: Vec<DamageEvent> = Vec::new();
    let mut damage_es_2: Vec<DamageEvent> = Vec::new();

    let mut count = 0;

    damage_es_1.extend(damage_events.read());
    while !damage_es_1.is_empty() || !damage_es_2.is_empty()
    {
        dmg_events(&mut commands, &mut actor_lookup, &damage_es_1, &mut damage_es_2);
        damage_es_1.clear();
        dmg_events(&mut commands, &mut actor_lookup, &damage_es_2, &mut damage_es_1);
        count += 1;
        if count >= 10
        {
            error!("Damage event recursion reached 10 levels! WTF?");
            break;
        }
    }
}
