use bevy::prelude::*;

use crate::simple::common::{Dead, Position};

use super::effect::{
    apply_on_ability_end_effects, 
    apply_on_death_effects, 
    ActorChild, 
    ActorContext, 
    ActorEffectContext
};




pub fn s_destroy_dead_things(
    mut commands: Commands,
    mut parent_lookup: Query<(&mut ActorContext, &mut Position), Without<Dead>>,
    mut dead_parents: Query<(Entity, &mut ActorContext, &mut Position), With<Dead>>,
    dead_children: Query<(Entity, &ActorChild), With<Dead>>,
    dead_things: Query<Entity, (Without<ActorChild>, Without<ActorContext>, With<Dead>)>,
) {
    for (entity, child) in &dead_children
    {
        if let Ok((mut actor_context, mut position)) = parent_lookup.get_mut(child.parent_actor)
        {
            apply_on_ability_end_effects(child.ability_type, &mut ActorEffectContext{ actor: &mut actor_context, commands: &mut commands, location: &mut position });
        }
        else if let Ok((_, mut actor_context, mut position)) = dead_parents.get_mut(entity) 
        {
            apply_on_ability_end_effects(child.ability_type, &mut ActorEffectContext{ actor: &mut actor_context, commands: &mut commands, location: &mut position });
        }
        else
        {
            error!("Did not find parent for dead entity!");
        }
        commands.entity(entity).despawn_recursive();
    }
    for (entity, mut actor, mut position) in &mut dead_parents
    {
        apply_on_death_effects(&mut ActorEffectContext{ actor: &mut actor, commands: &mut commands, location: &mut position });
        commands.entity(entity).despawn_recursive();
    }
    for entity in &dead_things
    {
        commands.entity(entity).despawn_recursive();
    }
}




