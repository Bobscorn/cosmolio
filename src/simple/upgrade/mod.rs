pub mod ui;
pub mod description;

use bevy::prelude::*;
use bevy_replicon::{network_event::{client_event::FromClient, server_event::{SendMode, ToClients}}, renet::ClientId};
use serde::{Deserialize, Serialize};

use crate::simple::{behaviours::effect::{ChildType, SerializedDamageEffect, SpawnLocation, SpawnType}, consts::SERVER_STR};

use super::{behaviours::effect::{ActorContext, SerializedActorEffect, SerializedEffectTrigger}, player::Player};

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

#[derive(Clone, Copy, PartialEq, Deserialize, Serialize)]
pub enum UpgradeBehaviour
{
    AddEffect(SerializedEffectTrigger),
    // ReplaceEffects{ criteria: ReplaceCriteria,  },
    // RemoveEffects // Maybe?
}

#[derive(Clone, PartialEq, Deserialize, Serialize)]
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


fn generate_upgrades_for_client(_player_context: &ActorContext) -> Vec<Upgrade>
{
    let behaviour = UpgradeBehaviour::AddEffect(SerializedEffectTrigger::OnDamage(SerializedDamageEffect::AddDamageEffect { amount: -1.5 }));
    let behaviour_2 = UpgradeBehaviour::AddEffect(SerializedEffectTrigger::OnDamage(SerializedDamageEffect::RegularEffect{ effect: SerializedActorEffect::SpawnEffect(SpawnType::Explosion { radius: 50.0, damage: 1.0, knockback_strength: 50.0 }, SpawnLocation::AtCaster)}));
    vec![Upgrade { behaviour, name: "Giga Insane Dmg".into(), description: behaviour.describe() }, Upgrade { behaviour: behaviour_2, name: "Oopsies Missile".into(), description: behaviour_2.describe() }]
}

fn add_upgrade_to_actor(actor: &mut ActorContext, upgrade: Upgrade)
{
    match upgrade.behaviour
    {
        UpgradeBehaviour::AddEffect(effect) => {
            actor.effects.push(effect);
        }
    }
}

pub fn s_generate_and_emit_available_upgrades(
    mut commands: Commands,
    mut available_upgrades: EventWriter<ToClients<GeneratedAvailableUpgrades>>,
    players: Query<(Entity, &Player, &ActorContext)>,
    input_epico: Res<Input<KeyCode>>,
) {
    if input_epico.just_pressed(KeyCode::Y)
    {
        info!("{SERVER_STR} Generate availabe upgrades go!");
        for (player_ent, player_id, context) in &players
        {
            let upgrades = generate_upgrades_for_client(context);
            commands.entity(player_ent).insert(AvailablePlayerUpgrades { chosen: false, upgrades: upgrades.clone() });
            available_upgrades.send(ToClients { mode: SendMode::Direct(ClientId::from_raw(player_id.0)), event: GeneratedAvailableUpgrades { upgrades } });
            debug!("{SERVER_STR} Sending upgrades to client {}", player_id.0);
        }
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
            if player_id.0 != client_id.raw()
            {
                continue;
            }

            if available_upgrades.chosen
            {
                warn!("{SERVER_STR} Client {} sent multiple ChosenUpgrade events at once! It should only ever get one upgrade at a time", client_id.raw());
                continue;
            }
            if !available_upgrades.upgrades.iter().any(|x| x == &event.upgrade)
            {
                warn!("{SERVER_STR} Received invalid chosen upgrade from client: {}", player_id.0);
                debug!("{SERVER_STR} Chosen upgrade has description: {}", event.upgrade.description);
                continue;
            }

            info!("{SERVER_STR} Added upgrade to player's actor context. Description is: {}", event.upgrade.description);
            add_upgrade_to_actor(&mut actor_context, event.upgrade.clone());
            available_upgrades.chosen = true;
            commands.entity(player_ent).remove::<AvailablePlayerUpgrades>();
            break;
        }
        warn!("{SERVER_STR} Client {} sent a ChosenUpgrade event that was not found (likely does not have AvailablePlayerUpgrades anymore)", client_id.raw());
    }
}
