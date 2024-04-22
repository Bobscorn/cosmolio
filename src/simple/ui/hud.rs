use bevy::prelude::*;

use crate::simple::{gameplay::{Enemy, actor::{ActorContext, Stat}}, player::LocalPlayer};

#[derive(Component)]
pub struct InfoText;


pub fn c_update_info_text(
    enemies: Query<(), With<Enemy>>,
    local_player_health: Query<&ActorContext, (Without<Enemy>, Without<InfoText>, With<LocalPlayer>)>,
    mut text: Query<&mut Text, (Without<Enemy>, With<InfoText>)>,
) {
    let Ok(mut text) = text.get_single_mut() else { return; };
    let enemy_count = enemies.iter().count();
    if enemy_count > 0
    {
        text.sections[0].value = String::from(format!("{enemy_count} enemies"));
    }
    else 
    {
        text.sections[0].value = "No enemies".into();
    }
    if let Ok(local_player) = local_player_health.get_single()
    {
        if local_player.stats.contains_key(&Stat::Health)
        {
            let extra = match local_player.stats.contains_key(&Stat::MaxHealth)
            {
                true => format!("/{}", local_player.stats[&Stat::MaxHealth]),
                false => "".into()
            };
            text.sections[2].value = format!("You have {}{} health", local_player.stats[&Stat::Health], extra);
        }
        else
        {
            text.sections[2].value = format!("You have no health stat... (you are therefore unkillable)");
        }
    }
    else
    {
        text.sections[2].value = format!("You are dead.");
    }
}
