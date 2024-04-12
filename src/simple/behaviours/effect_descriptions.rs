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
            ChildType::Explosion => "Explosion",
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
            Self::OnDoDamage(e) => format!("Damage you do: {}", e.describe()),
            Self::OnReceiveDamage(e) => format!("Damage you receive: {}", e.describe()),
            Self::OnDamageDone(e) => format!("When doing damage: {}", e.describe()),
            Self::OnDamageReceived(e) => format!("When receiving damage: {}", e.describe()),
            Self::Periodically { remaining_period, period, effect } => 
                format!("Every {period} seconds: {}", effect.describe()),
            Self::OnDeath(e) => format!("Upon dying: {}", e.describe()),
            Self::OnAbilityCast { ability_type, effect } => 
                format!("On casting a {} ability: {}", ability_type.name(), effect.describe()),
            Self::OnAbilityHit { ability_type, effect } =>
                format!("When a {} ability hits: {}", ability_type.name(), effect.describe()),
            Self::OnAbilityEnd { ability_type, effect } =>
                format!("When a {} ability ends (destroyed/finishes): {}", ability_type.name(), effect.describe()),
            Self::OnKill(e) => format!("Upon killing an enemy: {}", e.describe()),
        }
    }
}
