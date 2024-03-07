use super::effect::{ChildType, SerializedActorEffect, SerializedEffectTrigger};


impl ChildType
{
    pub fn name(&self) -> &str
    {
        match self
        {
            ChildType::Grenade => "Grenade",
            ChildType::Melee => "Melee",
            ChildType::Missile => "Missile",
            ChildType::Projectile => "Projectile",
            ChildType::ChildActor => todo!(),
        }
    }
}

impl SerializedEffectTrigger
{
    pub fn describe(&self) -> String
    {
        match self
        {
            Self::OnDamage(e) => format!("When dealing damage: {}", e.instantiate().describe()),
            Self::OnReceiveDamage(e) => format!("On damage being dealt to you: {}", e.instantiate().describe()),
            Self::Periodically { remaining_period, period, effect } => 
                format!("Every {period} seconds: {}", effect.instantiate().describe()),
            Self::OnDeath(e) => format!("Upon dying: {}", e.instantiate().describe()),
            Self::OnAbilityCast { ability_type, effect } => 
                format!("On casting a {} ability: {}", ability_type.name(), effect.instantiate().describe()),
            Self::OnAbilityHit { ability_type, effect } =>
                format!("When a {} ability hits: {}", ability_type.name(), effect.instantiate().describe()),
            Self::OnAbilityEnd { ability_type, effect } =>
                format!("When a {} ability ends (destroyed/finishes): {}", ability_type.name(), effect.instantiate().describe()),
            Self::OnKill(e) => format!("Upon killing an enemy: {}", e.instantiate().describe()),
        }
    }
}
