use bevy::prelude::*;


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

#[derive(Clone, Copy, Deserialize, Serialize)]
pub struct ClassData
{
    pub effects: Vec<SerializedEffectTrigger>,
    pub stats: Vec<SerializedBaseStat>
}
