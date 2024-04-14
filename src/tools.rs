use std::{fs::File, io::Write};

use clap::Parser;

use crate::simple::{behaviours::{effect::*, stats::StatusEffect}, upgrade::{static_upgrades::UpgradeCollection, Upgrade, UpgradeBehaviour}};

pub mod simple;

#[derive(Parser)]
pub struct MyArgs
{
    /// Path of file to write serialized data to
    #[arg(short, long, default_value_t = String::from("joe.ron"))]
    pub file: String,
}

fn upgrade_for_behaviour(behaviour: UpgradeBehaviour) -> Upgrade
{
    Upgrade
    {
        behaviour: behaviour.clone(),
        name: "Upgrade Name".into(),
        description: behaviour.describe()
    }
}

fn main()
{
    let args = MyArgs::parse();

    println!("Tools here, whats the prob?");

    let upgrade_collection = UpgradeCollection { upgrades: vec![
        upgrade_for_behaviour(UpgradeBehaviour::AddEffects(vec![
            SerializedEffectTrigger::OnAbilityHit { 
                ability_type: ChildType::Projectile, 
                effect: SerializedOnHitEffect::SpawnEffectAtHitLocation { 
                    spawn_type: SpawnType::Explosion { 
                        radius: 250.0, 
                        damage: 25.0, 
                        knockback_strength: -100.0,
                    } 
                } 
            },
            SerializedEffectTrigger::OnAbilityHit { 
                ability_type: ChildType::Projectile, 
                effect: SerializedOnHitEffect::RegularEffect { effect: SerializedActorEffect::AffectHealth(-5.0) },
            }
        ])),
        upgrade_for_behaviour(UpgradeBehaviour::AddEffects(vec![
            SerializedEffectTrigger::OnDamageDone(SerializedDamageViewEffect::EveryXDamageEffect { accumulated_damage: 0.0, damage_threshold: 50.0, which_actor: DamageActor::Instigator, effect: SerializedActorEffect::AffectHealth(10.0) })
        ]))
    ] };

    let mut f = File::create(&args.file).expect("Could not create output file");

    let pretty_config = 
        ron::ser::PrettyConfig::new()
        .enumerate_arrays(true)
        .separate_tuple_members(true);

    ron::ser::to_writer_pretty(&f, &upgrade_collection, pretty_config).expect("Failed to serialize data");

    f.flush().unwrap();

    println!("Wrote thing to file '{}'", args.file);
}
