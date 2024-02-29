use std::sync::{Arc, Mutex};

use bevy::{ecs::system::SystemId, prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::simple::behaviours::{effect::{ActorContext, SerializedEffectTrigger}, stats::{SerializedBaseStat, StatValue}};


#[derive(PartialEq, Eq, Hash)]
pub enum AbilityTrigger
{
    JustPressed(KeyCode),
    HeldDown(KeyCode),
    JustPressedOrReleased(KeyCode),
    JustReleased(KeyCode),
}

pub struct Class
{
    pub setup_fn: Option<Arc<Mutex<dyn FnMut(&mut Commands, Entity, &mut ActorContext) + Sync + Send>>>,
    pub teardown_fn: Option<Arc<Mutex<dyn FnMut(&mut Commands, Entity) + Sync + Send>>>,
    pub abilities: HashMap<AbilityTrigger, SystemId>,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ClassType
{
    DefaultClass,
    MeleeClass,
    RangedClass,
}

#[derive(Component, Serialize, Deserialize)]
pub struct PlayerClass
{
    pub class: ClassType,
}

#[derive(Resource)]
pub struct Classes
{
    pub classes: HashMap<ClassType, Class>,
}

#[derive(Deserialize, Serialize)]
pub struct ClassBaseData
{
    pub effects: Vec<SerializedEffectTrigger>,
    pub stats: Vec<SerializedBaseStat>
}

impl Clone for ClassBaseData
{
    fn clone(&self) -> Self {
        Self
        {
            effects: self.effects.clone(),
            stats: self.stats.clone()
        }
    }
}

impl Into<ActorContext> for ClassBaseData
{
    fn into(self) -> ActorContext {
        ActorContext { 
            status_effects: Vec::new(),
            effects: Vec::from_iter(self.effects.iter().map(|x| { x.instantiate() })),
            stats: HashMap::from_iter(self.stats.iter().map(|x| { (x.stat, StatValue::new(x.value)) }))
        }
    }
}

#[cfg(test)]
mod tests
{
    use crate::simple::behaviours::effect::SerializedEffectTrigger;

    use super::ClassBaseData;

    #[test]
    fn test_into_actor()
    {
        // let test_base_data = ClassBaseData
        // {
        //     effects: vec![SerializedEffectTrigger::]
        // }
    }
}
