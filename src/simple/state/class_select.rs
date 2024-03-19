use bevy::prelude::*;
use bevy_replicon::network_event::server_event::ToClients;

use crate::simple::{classes::class::{ClassBaseData, ClassType, Classes}, client::GeneralClientEvents, consts::{CLIENT_STR, SERVER_STR}, visuals::ui::Fonts};

use super::{GameState, ServerStateEvent};

/// Tag component for the root UI node of the Class select UI
#[derive(Component)]
pub struct ClassSelectUIRoot;

/// Data component for each UI Node that will catch UI events
#[derive(Component)]
pub struct ClassSelectUI
{
    pub class: ClassType,
}

#[derive(Component)]
pub struct GoInGameButtonTag;

pub fn setup_class_select_ui(
    mut commands: Commands,
    classes: Res<Classes>,
    class_datas: Res<Assets<ClassBaseData>>,
    fonts: Res<Fonts>,
) {
    let root_style = Style {
        display: Display::Flex,
        justify_content: JustifyContent::FlexStart,
        flex_direction: FlexDirection::Column,
        width: Val::Percent(100.0),
        height: Val::Percent(100.0),
        ..default()
    };

    let upper_panel_style = Style {
        display: Display::Flex,
        padding: UiRect::all(Val::Px(50.0)),
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        ..default()
    };

    let lower_panel_style = Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Row,
        justify_content: JustifyContent::SpaceAround,
        
        align_items: AlignItems::Center,
        min_width: Val::Percent(100.0),
        ..default()
    };

    let title_text_style = Style {
        display: Display::Flex,
        align_content: AlignContent::Center,
        ..default()
    };

    let title_text_text_style = TextStyle {
        font: fonts.upgrade_font.clone(),
        font_size: 60.0,
        color: Color::WHITE,
    };

    let class_node_style = Style {
        display: Display::Flex,
        flex_direction: FlexDirection::Column,
        justify_content: JustifyContent::Center,
        align_items: AlignItems::Center,
        row_gap: Val::Px(25.0),
        max_width: Val::Px(250.0),
        aspect_ratio: Some(1.5 / 2.5),
        ..default()
    };

    let class_title_style = Style {

        ..default()
    };

    let class_description_style = Style {
        display: Display::Flex,
        flex_wrap: FlexWrap::Wrap,
        ..default()
    };

    let class_description_text_style = TextStyle {
        font: fonts.upgrade_font.clone(),
        font_size: 20.0,
        color: Color::WHITE,
    };

    let class_title_text_style = TextStyle {
        font: fonts.upgrade_font.clone(),
        font_size: 40.0,
        color: Color::WHITE
    };

    commands.spawn((
        NodeBundle {
            style: root_style,
            ..default()
        },
        ClassSelectUIRoot,
        Name::new("Class Select Root UI Node"),
    )).with_children(|root_builder| {
        root_builder.spawn((
            NodeBundle {
                style: upper_panel_style,
                ..default()
            },
            Name::new("Class Select Upper Panel")
        )).with_children(|upper_panel| {
            upper_panel.spawn((
                TextBundle {
                    text: Text::from_section("Select a Class", title_text_text_style.clone()),
                    style: title_text_style,
                    ..default()
                },
                Name::new("Class Select Title Text")
            ));
        });

        root_builder.spawn((
            NodeBundle {
                style: lower_panel_style,
                ..default()
            },
            Name::new("Class Select Lower Panel")
        )).with_children(|panel_builder| {
            for (class_type, class_val) in &classes.classes
            {
                let class_data = class_datas.get(&class_val.base_data).expect("did not find class data in class select ui");
                panel_builder.spawn((
                    NodeBundle {
                        style: class_node_style.clone(),
                        ..default()
                    },
                    Interaction::None,
                    ClassSelectUI { class: *class_type },
                    Name::new(format!("Class '{class_type}' node"))
                )).with_children(|class_node| {
                    class_node.spawn((
                        TextBundle {
                            text: Text::from_section(class_data.name.clone(), class_title_text_style.clone()),
                            style: class_title_style.clone(),
                            ..default()
                        },
                        Name::new(format!("Class '{class_type}' title node"))
                    ));
                    class_node.spawn((
                        TextBundle {
                            text: Text::from_section(class_data.description.clone(), class_description_text_style.clone()),
                            style: class_description_style.clone(),
                            ..default()
                        },
                        Name::new(format!("Class '{class_type}' description node"))
                    ));
                });
            }
        });

        root_builder.spawn((
            ButtonBundle {
                style: Style {
                    display: Display::Flex,
                    margin: UiRect::all(Val::Px(150.0)),
                    ..default()
                },
                ..default()
            },
            GoInGameButtonTag,
            Name::new("Go In game button")
        )).with_children(|button| {
            button.spawn((
                TextBundle {
                    text: Text::from_section("Go In Game", title_text_text_style.clone()),
                    ..default()
                },
                Name::new("Go in game button text")
            ));
        });
    });
}

pub fn handle_class_select_ui(
    interacted_entities: Query<(&ClassSelectUI, &Interaction), Changed<Interaction>>,
    mut event_writer: EventWriter<GeneralClientEvents>,
) {
    for (class_s, interaction) in &interacted_entities
    {
        if interaction != &Interaction::Pressed
        {
            continue;
        }

        info!("{CLIENT_STR} Class {} clicked", class_s.class);
        event_writer.send(GeneralClientEvents::ChangeClass(class_s.class));
    }
}

pub fn s_handle_go_in_game_ui(
    interacted_entities: Query<&Interaction, (With<GoInGameButtonTag>, Changed<Interaction>)>,
    mut event_writer: EventWriter<ToClients<ServerStateEvent>>,
    mut next_state: ResMut<NextState<GameState>>,
) {
    for interaction in &interacted_entities
    {
        if interaction != &Interaction::Pressed
        {
            continue;
        }

        info!("{SERVER_STR} Going in game...");
        event_writer.send(ToClients { mode: bevy_replicon::network_event::server_event::SendMode::Broadcast, event: ServerStateEvent::GoInGame });
        next_state.set(GameState::InGame);
    }
}

pub fn teardown_class_select_ui(
    mut commands: Commands,
    class_select_roots: Query<Entity, With<ClassSelectUIRoot>>,
) {
    for entity in &class_select_roots
    {
        commands.entity(entity).despawn_recursive();
    }
}
