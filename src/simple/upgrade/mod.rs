pub mod ui;
pub mod description;
pub mod static_upgrades;

use bevy::prelude::*;
use bevy_replicon::{core::ClientId, network_event::{client_event::FromClient, server_event::{SendMode, ToClients}}};
use rand::{seq::IteratorRandom, thread_rng};
use serde::{Deserialize, Serialize};

use crate::simple::{behaviours::effect::{SerializedDamageChangeEffect, SpawnLocation, SpawnType}, consts::SERVER_STR};

use self::static_upgrades::{StaticUpgrades, UpgradeCollection};

use super::{behaviours::{effect::{ActorContext, SerializedActorEffect, SerializedEffectTrigger}, stats::StatusEffect}, player::Player};

// #[derive(Deserialize, Serialize)]
// pub enum TriggerType
// {
//     OnDamage,
//     Periodically,
//     OnKill,
//     OnDeath,
//     OnReceiveDamage,
//     OnAbilityCast,
//     OnAbilityHit,
//     OnAbilityEnd,
// }

// #[derive(Deserialize, Serialize)]
// pub enum ReplaceCriteria
// {
//     ByEffectTrigger(TriggerType),
//     ByRegularEffectType(SerializedActorEffect),
// }

#[derive(Clone, PartialEq, Deserialize, Serialize, Reflect)]
pub enum UpgradeBehaviour
{
    AddEffects(Vec<SerializedEffectTrigger>),
    AddStatusEffects(Vec<StatusEffect>),
    // ReplaceEffects{ criteria: ReplaceCriteria,  },
    // RemoveEffects // Maybe?
}

#[derive(Clone, PartialEq, Deserialize, Serialize, Reflect)]
pub struct Upgrade
{
    pub name: String,
    pub behaviour: UpgradeBehaviour,
    pub description: String,
}


/// An event emitted by the server to each client containing the upgrades available to them
#[derive(Event, PartialEq, Serialize, Deserialize)]
pub struct GeneratedAvailableUpgrades
{
    pub upgrades: Vec<Upgrade>,
}

/// An event emitted from clients to the server to indicate which upgrade a player has chosen
#[derive(Event, PartialEq, Serialize, Deserialize)]
pub struct ChosenUpgrade
{
    pub upgrade: Upgrade,
}

/// Stores upgrades that are available to the player.
/// 
/// Server side (and host) only component, used to track what upgrades are available to a player
/// This prevents bugs/fraudulent adding of upgrades
#[derive(Component)]
pub struct AvailablePlayerUpgrades
{
    pub chosen: bool,
    pub upgrades: Vec<Upgrade>,
}


fn generate_upgrades_for_client(_player_context: &ActorContext, upgrades: &UpgradeCollection) -> Vec<Upgrade>
{
    // let behaviour = UpgradeBehaviour::AddEffects(vec![SerializedEffectTrigger::OnDamage(SerializedDamageEffect::AddDamageEffect { amount: -1.5 })]);
    // let behaviour_2 = UpgradeBehaviour::AddEffects(vec![SerializedEffectTrigger::OnDamage(SerializedDamageEffect::RegularEffect{ effect: SerializedActorEffect::SpawnEffect(SpawnType::Explosion { radius: 50.0, damage: 1.0, knockback_strength: 50.0 }, SpawnLocation::AtCaster)})]);
    // TODO: Randomly sample from predefined upgrades
    //upgrades.upgrades.iter().map(|x| x.clone()).choose_multiple(&mut thread_rng(), 3)
    upgrades.upgrades.iter().map(|x| x.clone()).collect::<Vec<Upgrade>>()
}

fn add_upgrade_to_actor(actor: &mut ActorContext, upgrade: Upgrade)
{
    match upgrade.behaviour
    {
        UpgradeBehaviour::AddEffects(effects) => 
        {
            for effect in effects
            {
                actor.effects.push(effect);
            }
        },
        UpgradeBehaviour::AddStatusEffects(statuses) => 
        {
            for stat_effect in statuses
            {
                actor.status_effects.push(stat_effect);
            }
        }
    }
}

pub fn s_generate_and_emit_available_upgrades(
    mut commands: Commands,
    mut available_upgrades_events: EventWriter<ToClients<GeneratedAvailableUpgrades>>,
    available_upgrades: Res<StaticUpgrades>,
    upgrade_asset: Res<Assets<UpgradeCollection>>,
    players: Query<(Entity, &Player, &ActorContext)>,
) {
    info!("{SERVER_STR} Generate availabe upgrades go!");
    for (player_ent, player_id, context) in &players
    {
        let Some(static_upgrades) = upgrade_asset.get(&available_upgrades.upgrades) else { error!("Static Upgrades were not loaded!"); return; };
        let upgrades = generate_upgrades_for_client(context, static_upgrades);
        commands.entity(player_ent).insert(AvailablePlayerUpgrades { chosen: false, upgrades: upgrades.clone() });
        available_upgrades_events.send(ToClients { mode: SendMode::Direct(player_id.0), event: GeneratedAvailableUpgrades { upgrades } });
        debug!("{SERVER_STR} Sending upgrades to client {}", player_id.0.get());
    }
}

pub fn s_receive_chosen_upgrades(
    mut commands: Commands,
    mut chosen_upgrades: EventReader<FromClient<ChosenUpgrade>>,
    mut players: Query<(Entity, &Player, &mut ActorContext, &mut AvailablePlayerUpgrades)>,
) {
    for FromClient { client_id, event } in chosen_upgrades.read()
    {
        for (player_ent, player_id, mut actor_context, mut available_upgrades) in &mut players
        {
            if &player_id.0 != client_id
            {
                continue;
            }

            if available_upgrades.chosen
            {
                warn!("{SERVER_STR} Client {} sent multiple ChosenUpgrade events at once! It should only ever get one upgrade at a time", client_id.get());
                continue;
            }
            if !available_upgrades.upgrades.iter().any(|x| x == &event.upgrade)
            {
                warn!("{SERVER_STR} Received invalid chosen upgrade from client: {}", player_id.0.get());
                debug!("{SERVER_STR} Chosen upgrade has description: {}", event.upgrade.description);
                continue;
            }

            info!("{SERVER_STR} Added upgrade to player's actor context. Description is: {}", event.upgrade.description);
            add_upgrade_to_actor(&mut actor_context, event.upgrade.clone());
            available_upgrades.chosen = true;
            commands.entity(player_ent).remove::<AvailablePlayerUpgrades>();
            break;
        }
        warn!("{SERVER_STR} Client {} sent a ChosenUpgrade event that was not found (likely does not have AvailablePlayerUpgrades anymore)", client_id.get());
    }
}
