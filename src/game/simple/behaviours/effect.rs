use bevy::{prelude::*, utils::HashMap};

use serde::{Deserialize, Serialize};

use crate::game::simple::consts::PLAYER_GROUPS;

use super::explosion::ExplosionReplicationBundle;




// TODO: Confirm this design of stat
// some alternatives could be: hashmap<str, f32> (stat name indexes a float values of the stats)
// Vector<struct Stat> -> struct Stat { name: str, value: f32 }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash)]
pub enum Stat
{
    Health,
    Armor,
    Damage,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum StatModification
{
    Multiply{ factor: f32 },
    Add{ amount: f32 },
    Exponent{ power: f32 }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub struct StatusEffect
{
    pub timeout: Option<f32>,
    pub stat: Stat,
    pub modification: StatModification,
}


// Struct that contains all the data useful to an 'affectable' entity
#[derive(Component)]
pub struct ActorContext
{
    pub powerups: Vec<EffectTrigger>,
    pub status_effects: Vec<StatusEffect>,
    pub stats: HashMap<Stat, f32>,
}

pub trait Effect
{
    fn apply_effect(&self, actor: &mut ActorContext);
}

pub trait DamageEffect
{
    fn process_damage(&self, instigator: &mut ActorContext, victim: &mut ActorContext, damage_in: f32) -> f32;
}

pub trait OnKillEffect
{
    fn apply_effect(&self, killer: &mut ActorContext, killed: &mut ActorContext);
}

// ^
// Effects
// Triggers
// v

pub enum EffectTrigger
{
    OnDamage(Box<dyn DamageEffect>),
    Periodically{ remaining_period: f32, period: f32, effect: Box<dyn Effect> },
    OnKill(Box<dyn OnKillEffect>),
    OnReceiveDamage(Box<dyn DamageEffect>),
    OnCastSpell(Box<dyn Effect>)
}

// ^
// Triggers
// Convenience Implementations
// v

impl<T: Effect> DamageEffect for T
{
    fn process_damage(&self, instigator: &mut ActorContext, victim: &mut ActorContext, damage_in: f32) -> f32 {
        self.apply_effect(instigator);
        damage_in
    }
}

impl<T: Effect> OnKillEffect for T
{
    fn apply_effect(&self, killer: &mut ActorContext, killed: &mut ActorContext) {
        self.apply_effect(killer);
    }
}

// ^
// Convenience
// Generic Effects
// v

pub trait ActorCondition
{
    fn check_actor(&self, actor_context: &ActorContext) -> bool;
}

pub struct IfEffect<TAct: ActorCondition, TEff: Effect>
{
    pub condition: TAct,
    pub effect: TEff
}

impl<TAct: ActorCondition, TEff: Effect> Effect for IfEffect<TAct, TEff>
{
    fn apply_effect(&self, ) {
        
    }
}



// ^
// Generic Effects
// Older implementation
// v

#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Owner
{
    Player{ id: u64 },
    Enemy{ ent: Entity },
}

// TODO: rework the 'target' system, replace with 'effect' systems
#[derive(PartialEq, Eq, Clone, Copy, Debug, Serialize, Deserialize)]
pub enum Target
{
    Caster,
    NearestAlly,
    NearestAllyExcludingCaster,
    NearestEnemy,

}

// Save for convenience spawning system?
// v
// 
#[derive(Clone, Copy, Debug, Serialize, Deserialize)]
pub enum SpawnType
{
    Explosion{ radius: f32, damage: f32, knockback_strength: f32, owner: Owner },
    // Future ideas v
    Missile{  },
    Lightning{  },
}

#[derive(Default, Debug, Clone, Serialize, Deserialize)]
pub enum Effect
{
    #[default]
    Nothing,
    SpawnEntity(SpawnType),
    ApplyStatus{ target: Target, status: StatusEffect },
    CombinationEffect{ effects: Vec<Effect> },
}

impl Effect
{
    pub fn is_nothing(&self) -> bool
    {
        match self 
        {
            Self::Nothing => true,
            Self::ApplyStatus { target, status } => false,
            Self::SpawnEntity(_) => false,
            Self::CombinationEffect { effects } => effects.is_empty(),
        }
    }
}

#[derive(Component)]
pub struct OnDestroy
{
    pub effect: Effect,
}

#[derive(Event)]
pub struct EffectApplication
{
    pub target: Option<Entity>,
    pub source: Option<Entity>,
    pub position: Vec2,
    pub effect: Effect,
}


pub fn s_apply_effect(
    mut commands: Commands,
    mut effect_events: EventReader<EffectApplication>,
) {
    for effect_application in effect_events.read()
    {
        apply_effect(effect_application, &mut commands)
    }
}

fn apply_effect(effect_application: &EffectApplication, commands: &mut Commands)
{
    let EffectApplication { target, source, position, effect } = effect_application;
    
    info!("Applying effect {effect:?}");
    match effect
    {
        Effect::Nothing => todo!(),
        Effect::SpawnEntity(spawn_type) => 
        {
            match spawn_type
            {
                SpawnType::Explosion { radius, damage, knockback_strength, owner } => 
                {
                    commands.spawn(ExplosionReplicationBundle::new(*radius, *knockback_strength, *position, *damage, PLAYER_GROUPS, Some(crate::game::simple::behaviours::projectile::ProjectileKnockbackType::Repulsion { center: *position, strength: *knockback_strength })));
                },
                SpawnType::Missile {  } => todo!(),
                SpawnType::Lightning {  } => todo!(),
            }
        },
        Effect::ApplyStatus { target, status } => todo!(),
        Effect::CombinationEffect { effects } =>
        {
            for child_effect in effects
            {
                let child_application = EffectApplication { target: *target, source: *source, position: *position, effect: effect.clone() };
                apply_effect(&effect_application, commands);
            }
        }
    }
}



