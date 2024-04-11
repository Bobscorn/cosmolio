use std::{
    error::Error, fmt::Display, sync::{Arc, Mutex}
};

use bevy::{asset::{AssetLoader, AsyncReadExt}, ecs::system::SystemId, prelude::*, utils::HashMap};
use serde::{Deserialize, Serialize};

use crate::simple::behaviours::{effect::{ActorContext, SerializedEffectTrigger}, stats::SerializedStat};


#[derive(PartialEq, Eq, Hash)]
pub enum AbilityTrigger
{
    JustPressed(KeyCode),
    HeldDown(KeyCode),
    JustPressedOrReleased(KeyCode),
    JustReleased(KeyCode),
}

pub struct LabelledSystemId
{
    pub name: String,
    pub system_id: SystemId
}

pub struct Class
{
    pub setup_fn: Option<Arc<Mutex<dyn FnMut(&mut Commands, Entity) + Sync + Send>>>,
    pub teardown_fn: Option<Arc<Mutex<dyn FnMut(&mut Commands, Entity) + Sync + Send>>>,
    pub abilities: HashMap<AbilityTrigger, LabelledSystemId>,
    pub base_data: Handle<ClassBaseData>,
}

#[derive(PartialEq, Eq, Hash, Serialize, Deserialize, Debug, Clone, Copy)]
pub enum ClassType
{
    DefaultClass,
    MeleeClass,
    RangedClass,
}

#[derive(Component, Serialize, Deserialize)]
pub struct ActorClass
{
    class: ClassType,
}

#[derive(Resource)]
pub struct Classes
{
    pub classes: HashMap<ClassType, Class>,
}

#[derive(Asset, Clone, Debug, TypePath, Deserialize, Serialize, PartialEq)]
pub struct ClassBaseData
{
    pub effects: Vec<SerializedEffectTrigger>,
    pub stats: Vec<SerializedStat>,
    pub description: String,
    pub name: String,
}

impl Display for ActorClass { 
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        f.write_str(format!("{}", self.class).as_str())
    } 
}
impl Display for ClassType
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            ClassType::DefaultClass => f.write_str("Default"),
            ClassType::MeleeClass => f.write_str("Melee"),
            ClassType::RangedClass => f.write_str("Ranged"),
        }
    }
}

impl ClassType
{
    pub fn all_class_types() -> [Self; 3]
    {
        [ClassType::DefaultClass, ClassType::MeleeClass, ClassType::RangedClass]
    }
}

impl ActorClass
{
    pub fn new(class: ClassType) -> Self
    {
        Self {
            class
        }
    }

    pub fn get_class(&self) -> ClassType
    {
        self.class
    }

    pub fn set_class(&mut self, commands: &mut Commands, class_data: &Assets<ClassBaseData>, classes: &mut Classes, actor: &mut ActorContext, entity: Entity, new_class: ClassType)
    {
        self.set_class_internal(commands, class_data, classes, actor, entity, new_class, true);
    }

    fn set_class_internal(&mut self, commands: &mut Commands, class_data: &Assets<ClassBaseData>, classes: &mut Classes, actor: &mut ActorContext, entity: Entity, new_class: ClassType, teardown: bool)
    {
        if teardown
        {
            if let Some(class_data) = classes.classes.get_mut(&self.class)
            {
                if let Some(tr_dwn) = &mut class_data.teardown_fn
                {
                    if let Ok(mut real_tr_dwn) = tr_dwn.lock()
                    {
                        real_tr_dwn(commands, entity);
                    }
                }
            }
        }
        let Some(new_class_data) = classes.classes.get_mut(&new_class) else { error!("Can not change to class {:?}!", new_class); return; };

        if let Some(setup_fn_mute) = &mut new_class_data.setup_fn
        {
            if let Ok(mut setup_fn) = setup_fn_mute.lock()
            {
                setup_fn(commands, entity);
            }
        }

        actor.stats.clear();
        actor.effects.clear();
        let dat = class_data.get(&new_class_data.base_data).unwrap();
        for base_stat in &dat.stats
        {
            actor.stats.insert(base_stat.stat, base_stat.value);
        }
        for base_effect in &dat.effects
        {
            actor.effects.push(*base_effect);
        }
        self.class = new_class;
    }
}

pub fn s_setup_initial_class(
    mut commands: Commands,
    class_datas: Res<Assets<ClassBaseData>>,
    mut classes: ResMut<Classes>,
    mut new_ents: Query<(Entity, &mut ActorContext, &mut ActorClass), Added<ActorClass>>,
) {
    for (entity, mut actor, mut class) in &mut new_ents
    {
        let c = class.class;
        class.set_class_internal(&mut commands, &class_datas, &mut classes, &mut actor, entity, c, false);
    }
}

impl Into<ActorContext> for ClassBaseData
{
    fn into(self) -> ActorContext {
        ActorContext { 
            status_effects: Vec::new(),
            effects: self.effects,
            stats: HashMap::from_iter(self.stats.iter().map(|x| { (x.stat, x.value) })),
            last_damage_source: None,
        }
    }
}

#[derive(Default)]
pub struct ClassDataLoader;

#[derive(Debug)]
pub enum ClassDataLoadError
{
    Io(std::io::Error),
    Ron(ron::error::SpannedError),
}

impl From<std::io::Error> for ClassDataLoadError
{
    fn from(value: std::io::Error) -> Self {
        Self::Io(value)
    }
}

impl From<ron::error::SpannedError> for ClassDataLoadError
{
    fn from(value: ron::error::SpannedError) -> Self {
        Self::Ron(value)
    }
}

impl Display for ClassDataLoadError
{
    fn fmt(&self, f: &mut std::fmt::Formatter<'_>) -> std::fmt::Result {
        match self
        {
            ClassDataLoadError::Io(e) => f.write_fmt(format_args!("Io error: {}", e)),
            ClassDataLoadError::Ron(e) => f.write_fmt(format_args!("Ron error: {}", e)),
        }
    }
}

impl Error for ClassDataLoadError {}

impl AssetLoader for ClassDataLoader
{
    type Asset = ClassBaseData;
    type Settings = ();
    type Error = ClassDataLoadError;

    fn load<'a>(
            &'a self,
            reader: &'a mut bevy::asset::io::Reader,
            _settings: &'a Self::Settings,
            _load_context: &'a mut bevy::asset::LoadContext,
        ) -> bevy::utils::BoxedFuture<'a, Result<Self::Asset, Self::Error>> {
        Box::pin(async move {
            let mut bytes = Vec::new();
            reader.read_to_end(&mut bytes).await?;
            let custom_asset = ron::de::from_bytes::<ClassBaseData>(&bytes)?.into();
            Ok(custom_asset)
        })
    }

    fn extensions(&self) -> &[&str] {
        &["cbd"]
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
            stats: vec![SerializedStat{ stat: Stat::Health, value: 100.0 }],
            description: "Test Stuff".into(),
            name: "Test Class".into(),
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

    #[test]
    fn test_serialize_class_data()
    {
        let mut base_data = ClassBaseData
        {
            effects: Vec::new(),
            stats: vec![SerializedStat{ stat: Stat::Health, value: 100.0 }, SerializedStat{ stat: Stat::MaxHealth, value: 100.0 }],
            description: "Test Stuff".into(),
            name: "Test Class".into(),
        };

        let f = File::create("melee_data_out.cbd").unwrap();

        ron::ser::to_writer(&f, &base_data).expect("failed to serialize melee data");
        let f = File::create("default_data_out.cbd").unwrap();

        ron::ser::to_writer(&f, &base_data).expect("failed to serialize default data");

        base_data.effects.push(SerializedEffectTrigger::OnAbilityHit { 
            ability_type: crate::simple::behaviours::effect::ChildType::Missile, 
            effect: crate::simple::behaviours::effect::SerializedOnHitEffect::SpawnEffectAtHitLocation { 
                spawn_type: crate::simple::behaviours::effect::SpawnType::Explosion { 
                    radius: crate::simple::consts::RANGED_MISSILE_EXPLOSION_RADIUS, 
                    damage: crate::simple::consts::RANGED_MISSILE_EXPLOSION_DAMAGE, 
                    knockback_strength: crate::simple::consts::RANGED_MISSILE_EXPLOSION_KNOCKBACK_STRENGTH 
                }
            } 
        });
        let f = File::create("ranged_data_out.cbd").unwrap();

        ron::ser::to_writer(&f, &base_data).expect("failed to serialize melee data");
    }
}
