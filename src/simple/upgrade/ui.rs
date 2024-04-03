use bevy::prelude::*;

use crate::simple::{behaviours::effect::{SerializedDamageEffect, SerializedEffectTrigger}, consts::CLIENT_STR, state::in_game::UpgradeContainerTag, visuals::ui::Fonts};

use super::{ChosenUpgrade, GeneratedAvailableUpgrades, Upgrade, UpgradeBehaviour};


#[derive(Component)]
pub struct UpgradeUI
{
    pub upgrade: Upgrade,
}

fn create_upgrade_ui_entity(commands: &mut Commands, upgrade: Upgrade, font_handle: Handle<Font>) -> Entity
{
    let background_entity = commands.spawn((
        NodeBundle {
            background_color: BackgroundColor(Color::DARK_GRAY),
            style: Style { 
                display: Display::Flex,
                overflow: Overflow::clip(),
                aspect_ratio: Some(0.75),
                width: Val::Px(200.0),
                min_width: Val::Percent(10.0),
                max_width: Val::Percent(40.0),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                padding: UiRect::all(Val::Px(10.0)),
                border: UiRect::all(Val::Px(5.0)),
                flex_direction: FlexDirection::Column,
                flex_wrap: FlexWrap::NoWrap,
                flex_basis: Val::Auto,
                row_gap: Val::Px(5.0),
                ..default()
            },
            ..default()
        },
        Interaction::None,
        Name::new(format!("'{}' Upgrade Background", &upgrade.name)),
        UpgradeUI { upgrade: upgrade.clone() },
    )).with_children(|background_builder| {
        background_builder.spawn((
            TextBundle {
                text: Text::from_section(&upgrade.name, TextStyle { font: font_handle.clone(), font_size: 20.0, color: Color::rgb(1.0, 0.2, 0.2) }),
                style: Style { display: Display::Flex, padding: UiRect::all(Val::Px(2.0)), ..default() },
                ..default()
            },
            Name::new(format!("'{}' Upgrade Name", &upgrade.name)),
        ));
        background_builder.spawn((
            ImageBundle {
                image: UiImage::default(),
                style: Style { display: Display::Flex, border: UiRect::all(Val::Px(2.0)), ..default() },
                ..default()
            },
            Name::new(format!("'{}' Upgrade Sprite", &upgrade.name)),
        ));
        background_builder.spawn((
            TextBundle {
                text: Text::from_section(upgrade.description, TextStyle { font: font_handle.clone(), font_size: 12.0, color: Color::WHITE }),
                style: Style { 
                    display: Display::Flex,
                    border: UiRect::all(Val::Px(3.0)),
                    overflow: Overflow { x: OverflowAxis::Visible, y: OverflowAxis::Clip },
                    flex_wrap: FlexWrap::Wrap,
                    ..default() 
                },
                ..default()
            },
            Name::new(format!("'{}' Upgrade Description", &upgrade.name)),
        ));
    }).id();
    
    background_entity
}

pub fn c_create_upgrade_ui(  // TODO: rename this function
    mut commands: Commands,
    fonts: Res<Fonts>,
    root_node_query: Query<Entity, With<UpgradeContainerTag>>,
    mut upgrade_events: EventReader<GeneratedAvailableUpgrades>,
) {
    for GeneratedAvailableUpgrades { upgrades } in upgrade_events.read()
    {
        info!("{CLIENT_STR} Received available upgrades from server!");
        let Ok(root_node) = root_node_query.get_single() else { error!("{CLIENT_STR} No Root UI node found to insert upgrades onto!"); return; };

        for upgrade in upgrades
        {
            let upgrade_ent = create_upgrade_ui_entity(&mut commands, upgrade.clone(), fonts.upgrade_font.clone());
            commands.entity(root_node).add_child(upgrade_ent);
        }
    }
}



pub fn c_handle_upgrade_clicked(
    mut commands: Commands,
    mut upgrade_set: ParamSet<(
        Query<(&Interaction, &UpgradeUI), Changed<Interaction>>, // Clicked upgrades
        Query<Entity, With<UpgradeUI>>, // All upgrades
    )>,
    mut chosen_upgrade_events: EventWriter<ChosenUpgrade>,
) {
    let mut destroy_upgrades = false;
    for (interaction, upgrade) in &upgrade_set.p0()
    {
        if interaction != &Interaction::Pressed
        {
            continue;
        }

        info!("{CLIENT_STR} Sending Upgrade chosen event to server");
        chosen_upgrade_events.send(ChosenUpgrade { upgrade: upgrade.upgrade.clone() });
        destroy_upgrades = true;
    }

    if !destroy_upgrades
    {
        return;
    }

    debug!("{CLIENT_STR} Destroying upgrade UIs");
    for ent in &upgrade_set.p1()
    {
        if let Some(coms) = commands.get_entity(ent)
        {
            coms.despawn_recursive();
        }
    }
}
