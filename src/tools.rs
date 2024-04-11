use std::{fs::File, io::Write};

use clap::Parser;

use crate::simple::{behaviours::{effect::*, stats::StatusEffect}, upgrade::{static_upgrades::UpgradeCollection, Upgrade, UpgradeBehaviour}};

pub mod simple;

#[derive(Parser)]
pub struct MyArgs
{
    /// Path of file to write serialized data to
    #[arg(short, long)]
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

    println!("Tools here, whats the prob Joe?");

    let upgrade_collection = UpgradeCollection { upgrades: vec![
        upgrade_for_behaviour(UpgradeBehaviour::AddEffects(vec![SerializedEffectTrigger::OnKill(
            SerializedKillEffect::RegularEffect { effect: SerializedActorEffect::AffectHealth(1.0) }
        )])),
        // upgrade_for_behaviour(UpgradeBehaviour::AddEffects(vec![
        //     SerializedEffectTrigger::OnReceiveDamage(Seri)
        // ]))
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
