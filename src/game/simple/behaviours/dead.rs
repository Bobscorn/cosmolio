use bevy::prelude::*;

use crate::game::simple::common::{Dead, Position};

use super::effect::{OnDestroy, EffectApplication};





pub fn s_destroy_dead_things(
    mut commands: Commands,
    dead_with_effects: Query<Entity, (With<OnDestroy>, Without<Position>, With<Dead>)>,
    dead_with_pos_and_effect: Query<(Entity, &Position, &OnDestroy), With<Dead>>,
    dead_things: Query<Entity, (Without<OnDestroy>, With<Dead>)>,
    mut effect_writer: EventWriter<EffectApplication>,
) {
    for (entity, position, on_destroy) in &dead_with_pos_and_effect
    {
        if !on_destroy.effect.is_nothing()
        {
            effect_writer.send(EffectApplication { target: None, source: None, position: position.0, effect: on_destroy.effect.clone() });
        }
        commands.entity(entity).despawn_recursive();
    }
    for entity in &dead_with_effects
    {   
        commands.entity(entity).despawn_recursive();
    }
    for entity in &dead_things
    {
        commands.entity(entity).despawn_recursive();
    }
}




