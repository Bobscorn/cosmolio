use bevy::prelude::*;

use crate::simple::behaviours::effect::{SerializedDamageEffect, SerializedEffectTrigger};

use super::{Upgrade, UpgradeBehaviour};


#[derive(Component)]
pub struct UpgradeUI
{
    pub upgrade: Upgrade,
}

fn create_upgrade_ui_entity(mut commands: Commands, upgrade: Upgrade, font_handle: Handle<Font>) -> Entity
{
    let root_entity = commands.spawn((
        NodeBundle {
            transform: Transform::from_scale(Vec3::ONE),
            ..default()
        }, 
        Name::new("Upgrade UI Root Node")
    )).with_children(|root_builder| {
        root_builder.spawn((
            NodeBundle {
                background_color: BackgroundColor::DEFAULT,
                style: Style { 
                    display: Display::Flex, 
                    overflow: Overflow::clip(), 
                    width: Val::Auto, 
                    height: Val::Auto, 
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
            Name::new("Upgrade UI Background")
        )).with_children(|background_builder| {
            background_builder.spawn((
                TextBundle {
                    text: Text::from_section("Cool Upgrade", TextStyle { font: font_handle.clone(), font_size: 60.0, color: Color::RED }),
                    style: Style { display: Display::Flex, padding: UiRect::all(Val::Px(2.0)), ..default() },
                    ..default()
                },
                Name::new("Upgrade UI Name Text"),
            ));
            background_builder.spawn((
                ImageBundle {
                    image: UiImage::default(),
                    style: Style { display: Display::Flex, border: UiRect::all(Val::Px(2.0)), ..default() },
                    ..default()
                },
                Name::new("Upgrade UI Sprite Image")
            ));
            background_builder.spawn((
                TextBundle {
                    text: Text::from_section(upgrade.description, TextStyle { font: font_handle.clone(), font_size: 45.0, color: Color::BLACK }),
                    style: Style { display: Display::Flex, border: UiRect::all(Val::Px(3.0)), ..default() },
                    ..default()
                },
                Name::new("Upgrade UI Description Text")
            ));
        });
    }).id();
    
    root_entity
}

pub fn s_create_upgrade_ui(
    mut commands: Commands,
    // stuff
) {
    let upgrade_behaviour = UpgradeBehaviour::AddEffect(SerializedEffectTrigger::OnDamage(SerializedDamageEffect::AddDamageEffect { amount: 5.0 }));
    let upgrade = Upgrade { behaviour: upgrade_behaviour, description: upgrade_behaviour.get_description() };
    
}
