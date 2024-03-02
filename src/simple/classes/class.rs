use std::{
    sync::{Arc, Mutex},
    error::Error,
    fmt::Display,
};

use bevy::{asset::{AssetLoader, AsyncReadExt}, ecs::system::SystemId, prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::simple::behaviours::{effect::{ActorContext, SerializedEffectTrigger}, stats::{SerializedStat, StatValue}};


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

#[derive(Asset, Debug, TypePath, Deserialize, Serialize, PartialEq)]
pub struct ClassBaseData
{
    pub effects: Vec<SerializedEffectTrigger>,
    pub stats: Vec<SerializedStat>
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
    use std::{fs::File, io::{Read, Write}};

    use crate::simple::behaviours::{
        effect::{SerializedEffectTrigger, SerializedDamageEffect},
        stats::{SerializedStat, Stat},
    };

    use super::ClassBaseData;

    const TEST_FILE_PATH: &str = "cargo_test_file.cbd";

    #[test]
    fn test_into_actor()
    {
        let base_data = ClassBaseData
        {
            effects: vec![SerializedEffectTrigger::OnDamage(SerializedDamageEffect::MultiplyDamageEffect { factor: 2.5 })],
            stats: vec![SerializedStat{ stat: Stat::Health, value: 100.0 }]
        };

        let mut f = File::create(TEST_FILE_PATH).unwrap();

        ron::ser::to_writer(&f, &base_data).expect("could not serialize class base data");

        f.flush().unwrap();

        let mut f = File::open(TEST_FILE_PATH).unwrap();

        let mut bytes = Vec::new();
        f.read_to_end(&mut bytes).expect("could not read bytes of serialized file");

        let new_data = ron::de::from_bytes::<ClassBaseData>(&bytes).expect("could not deserialize class base data");
        
        assert_eq!(new_data, base_data);
        
        let _  = std::fs::remove_file(TEST_FILE_PATH);
    }
}
