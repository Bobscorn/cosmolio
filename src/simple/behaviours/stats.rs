use bevy::prelude::*;
use serde::{Deserialize, Serialize};


// TODO: Confirm this design of stat
// some alternatives could be: hashmap<str, f32> (stat name indexes a float values of the stats)
// Vector<struct Stat> -> struct Stat { name: str, value: f32 }
#[derive(Clone, Copy, Debug, Serialize, Deserialize, Hash, PartialEq, Eq, Reflect)]
pub enum Stat
{
    Health, // The health of the actor
    MaxHealth, // The max health of the actor
    Armor, // A damage reduction stat, not implemented TODO: this stat
    Damage, // A damage stat that scales (almost) all damage
    MovementSpeed, // How many units an actor moves whilst walking per second
    CooldownRate, // How fast a cooldown finishes, total duration will be: normal_duration / CooldownRate
}

/// A modification to one of an actor's Stats.
/// This can be temporary, or permanent.
#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub struct StatusEffect
{
    pub timeout: Option<f32>,
    pub stat: Stat,
    pub modification: StatModification,
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq, Reflect)]
pub enum StatModification
{
    Multiply{ factor: f32 },
    Add{ amount: f32 },
    Exponent{ power: f32 }
}

#[derive(Clone, Copy, Debug, Serialize, Deserialize, PartialEq)]
pub struct SerializedStat
{
    pub stat: Stat,
    pub value: f32,
}

impl Stat
{
    pub fn name(&self) -> &str
    {
        match self
        {
            Stat::Armor => "Armor",
            Stat::Health => "Health",
            Stat::CooldownRate => "Cooldown Speed",
            Stat::MaxHealth => "Maximum Health",
            Stat::MovementSpeed => "Movement speed",
            Stat::Damage => "Base Damage",
        }
    }
}

impl StatusEffect
{
    // Produces a description of what this status effect does
    // It will fit into these formats:
    // - 'This status effect [description]'
    // - 'Inflicts a status effect that [description]'
    pub fn get_description(&self) -> String
    {
        let effect_str;
        if match self.modification
        {
            StatModification::Add { amount } => amount == 0.0,
            StatModification::Exponent { power } => power == 1.0,
            StatModification::Multiply { factor } => factor == 1.0,
        } {
            effect_str = String::from("Does nothing");
        }
        else
        {
            let stat_str = self.stat.name();

            let (pretty_str, verbose_str) = match self.modification
            {
                StatModification::Add { amount } => 
                {
                    let verbose_form = Some(format!("Adds {amount} to {stat_str}"));
                    if amount < 0.0 { (format!("Decreases {stat_str} by {}", -amount), verbose_form) 
                    } else { (format!("Increases {stat_str} by {}", amount), verbose_form) }
                },
                StatModification::Exponent { power } => 
                {
                    (format!("Raises {stat_str} to the power {power}"), None)
                }
                StatModification::Multiply { factor } => 
                {
                    let verbose_form = Some(format!("Multiplies {stat_str} by {factor}"));
                    if factor == -1.0 { (format!("Inverts {stat_str}"), verbose_form) }
                    else if factor == 0.0 { (format!("Sets {stat_str} to zero"), verbose_form) }
                    else if factor < 1.0 && factor > 0.0 { (format!("Decreases {stat_str} by {}%", (1.0 - factor) * 100.0), verbose_form) }
                    else if factor > 1.0 { (format!("Increases {stat_str} by {}%", (factor - 1.0) * 100.0), verbose_form) }
                    else { (format!("Multiplies {stat_str} by {factor}"), None) }
                }
            };

            let verbose_str = verbose_str.map_or(String::new(), |x| { String::from(" (") + &x + ")" });
            effect_str = pretty_str + &verbose_str;
        }

        let duration_str = match self.timeout
        {
            Some(time) => format!("for {time} seconds"),
            None => "indefinitely".into(),
        };

        String::from(effect_str) + " " + &duration_str
    }
}


#[cfg(test)]
mod tests
{
    use crate::simple::behaviours::stats::StatModification;

    use super::{Stat, StatusEffect};


    #[test]
    fn test_status_effect_description()
    {
        let mut nothing_effect = StatusEffect {
            timeout: None,
            stat: Stat::MovementSpeed,
            modification: StatModification::Add { amount: 0.0 }
        };

        assert_eq!(nothing_effect.get_description(), "Does nothing indefinitely");

        nothing_effect.modification = StatModification::Multiply { factor: 1.0 };
        assert_eq!(nothing_effect.get_description(), "Does nothing indefinitely");

        nothing_effect.timeout = Some(5.0);
        nothing_effect.modification = StatModification::Exponent { power: 1.0 };
        assert_eq!(nothing_effect.get_description(), "Does nothing for 5 seconds");

        let test_effect = StatusEffect {
            timeout: None,
            stat: Stat::Health,
            modification: super::StatModification::Multiply { factor: 1.5 },
        };

        assert_eq!(test_effect.get_description(), format!("Increases {0} by 50% (Multiplies {0} by 1.5) indefinitely", Stat::Health.name()));

        let test_effect = StatusEffect {
            timeout: Some(-500.0),
            stat: Stat::CooldownRate,
            modification: StatModification::Multiply { factor: 0.35 },
        };

        assert_eq!(test_effect.get_description(), format!("Decreases {0} by 65% (Multiplies {0} by 0.35) for -500 seconds", Stat::CooldownRate.name()));

        let mut test_effect = StatusEffect {
            timeout: None,
            stat: Stat::Armor,
            modification: StatModification::Multiply { factor: 0.0 },
        };

        assert_eq!(test_effect.get_description(), format!("Sets {0} to zero (Multiplies {0} by 0) indefinitely", Stat::Armor.name()));

        test_effect.modification = StatModification::Multiply { factor: -1.0 };
        assert_eq!(test_effect.get_description(), format!("Inverts {0} (Multiplies {0} by -1) indefinitely", Stat::Armor.name()));

        test_effect.modification = StatModification::Multiply { factor: -0.5 };
        assert_eq!(test_effect.get_description(), format!("Multiplies {0} by -0.5 indefinitely", Stat::Armor.name()));

        test_effect.modification = StatModification::Multiply { factor: -15.0 };
        assert_eq!(test_effect.get_description(), format!("Multiplies {0} by -15 indefinitely", Stat::Armor.name()));

        let mut test_effect = StatusEffect {
            timeout: Some(3.5),
            stat: Stat::MaxHealth,
            modification: StatModification::Add { amount: -2.5 },
        };

        assert_eq!(test_effect.get_description(), format!("Decreases {0} by 2.5 (Adds -2.5 to {0}) for 3.5 seconds", Stat::MaxHealth.name()));

        test_effect.modification = StatModification::Add { amount: 10.0 };
        assert_eq!(test_effect.get_description(), format!("Increases {0} by 10 (Adds 10 to {0}) for 3.5 seconds", Stat::MaxHealth.name()));


    }
}
