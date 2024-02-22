use bevy::prelude::*;

use crate::game::simple::common::Position;

use super::effect::{apply_on_damage_effects, apply_receive_damage_effects, ActorContext, ActorDamageEffectContext, DamageEvent, Stat};



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
        let existing_health = *victim_context.stats.get(&Stat::Health).unwrap_or(&0.0_f32);
        victim_context.stats.insert(Stat::Health, existing_health - damage_to_do);
    }
}
