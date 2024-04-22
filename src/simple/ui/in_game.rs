use bevy::prelude::*;
use bevy_replicon::network_event::server_event::{SendMode, ToClients};

use crate::simple::{
    consts::CLIENT_STR, 
    data::Fonts,
    state::{InGameState, ServerStateEvent},
};

/// Entity (not necessarily a UI entity) at the root of the In Game UI
#[derive(Component)]
pub struct RootMenuTag;

/// UI Entity at the root of the Break screen
#[derive(Component)]
pub struct BreakMenuTag;

/// UI Button that when clicked will move to next wave (assuming currently in a break, and is server)
#[derive(Component)]
pub struct NextWaveButtonTag;

/// UI Entity that contains the clickable upgrades
#[derive(Component)]
pub struct UpgradeContainerTag;

/// UI Entity at the root of the Pause Menu
#[derive(Component)]
pub struct PauseMenuTag;

/// Tag for entity(/ies) that resume fighting when clicked
#[derive(Component)]
pub struct ResumeButtonTag;

/// UI Entity at the root of the HUD
#[derive(Component)]
pub struct HUDTag;

/// Run when entering the GameState::InGame state to spawn all the UI hierachies
/// The disabling/enabling of the UIs will happen in other systems
pub fn setup_uis(
    mut commands: Commands,
    fonts: Res<Fonts>,
) {
    commands.spawn((
        NodeBundle {
            style: Style {
                display: Display::Flex,
                min_height: Val::Percent(100.0),
                max_height: Val::Percent(100.0),
                min_width: Val::Percent(100.0),
                max_width: Val::Percent(100.0),
                ..default()
            },
            ..default()
        },
        RootMenuTag,
        Name::new("In Game UI Root")
    )).with_children(|root| {

        // HUD v
        let hud_style = Style
        {
            display: Display::Flex,
            flex_direction: FlexDirection::ColumnReverse,
            width: Val::Percent(100.0),
            height: Val::Percent(100.0),
            justify_content: JustifyContent::FlexStart,
            position_type: PositionType::Absolute,
            ..default()
        };

        let placeholder_hud_style = Style
        {
            display: Display::Flex,
            width: Val::Px(10.0),
            height: Val::Px(10.0),
            ..default()
        };

        root.spawn((
            NodeBundle {
                style: hud_style,
                ..default()
            },
            HUDTag,
            Name::new("HUD Root"),
        )).with_children(|x| {
            x.spawn((
                NodeBundle {
                    style: placeholder_hud_style,
                    ..default()
                },
                Name::new("Placeholder HUD")
            ));
        });

        // Upgrade UI
        let break_root_style = Style
        {
            display: Display::Flex,
            flex_direction: FlexDirection::Column,
            width: Val::Percent(100.0),
            position_type: PositionType::Absolute,
            ..default()
        };

        let break_title_style = Style
        {
            display: Display::Flex,
            width: Val::Percent(100.0),
            padding: UiRect::all(Val::Px(15.0)),
            justify_content: JustifyContent::Center,
            align_items: AlignItems::Center,
            ..default()
        };

        let upgrade_container_style = Style 
        { 
            display: Display::Flex,
            overflow: Overflow::clip(),
            min_height: Val::Px(100.0),
            height: Val::Auto,
            width: Val::Percent(100.0),
            align_items: AlignItems::Center,
            justify_content: JustifyContent::Center,
            padding: UiRect::all(Val::Px(10.0)),
            border: UiRect::all(Val::Px(5.0)),
            flex_direction: FlexDirection::Row,
            flex_wrap: FlexWrap::NoWrap,
            flex_basis: Val::Auto,
            column_gap: Val::Px(50.0),
            ..default()
        };

        root.spawn((
            NodeBundle {
                style: break_root_style,
                visibility: Visibility::Hidden, // Initially hidden (assumes default state Fighting)
                ..default()
            },
            BreakMenuTag,
            Name::new("Break UI Root")
        )).with_children(|break_root| {
            break_root.spawn((
                NodeBundle {
                    style: break_title_style,
                    ..default()
                },
                Name::new("Break Title Container")
            )).with_children(|title| {
                title.spawn((
                    TextBundle {
                        text: Text::from_section("Wave Complete", TextStyle { font: fonts.thick_font.clone(), font_size: 45.0, color: Color::WHITE }),
                        ..default()
                    },
                    Name::new("Break Title Text")
                ));
            });

            break_root.spawn((
                NodeBundle {
                    style: upgrade_container_style,
                    ..default()
                },
                UpgradeContainerTag,
                Name::new("Upgrades UI Container")
            ));

            break_root.spawn((
                NodeBundle {
                    style: Style {
                        display: Display::Flex,
                        padding: UiRect::all(Val::Px(30.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                Name::new("Next Wave Button Container"),
            )).with_children(|next_wave_but_cont| {
                next_wave_but_cont.spawn((
                    ButtonBundle {
                        style: Style { 
                            display: Display::Flex, 
                            padding: UiRect::all(Val::Px(5.0)),
                            justify_content: JustifyContent::Center,
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        ..default()
                    },
                    NextWaveButtonTag,
                    Name::new("Next Wave Button")
                )).with_children(|button| {
                    button.spawn((
                        TextBundle {
                            text: Text::from_section("Begin Next Wave", TextStyle { font: fonts.thick_font.clone(), font_size: 35.0, color: Color::BLACK }),
                            ..default()
                        },
                        Name::new("Next Wave Button Text")
                    ));
                });
            });
        });
        
        // Pause menu
        root.spawn((
            NodeBundle {
                style: Style {
                    display: Display::Flex,
                    flex_direction: FlexDirection::Column,
                    row_gap: Val::Px(50.0),
                    justify_content: JustifyContent::FlexStart,
                    align_items: AlignItems::Center,
                    position_type: PositionType::Absolute,
                    ..default()
                },
                background_color: BackgroundColor(Color::rgba(0.4, 0.4, 0.4, 0.4)),
                visibility: Visibility::Hidden, // Initially hidden (assumes default state is Fighting)
                ..default()
            },
            PauseMenuTag,
            Name::new("Pause Menu Root UI")
        )).with_children(|pause_menu| {
            pause_menu.spawn((
                TextBundle {
                    text: Text::from_section("Paused", TextStyle { font: fonts.thick_font.clone(), font_size: 35.0, color: Color::WHITE }),
                    ..default()
                },
                Name::new("Pause Menu Text")
            ));
            pause_menu.spawn((
                ButtonBundle {
                    style: Style {
                        display: Display::Flex,
                        padding: UiRect::all(Val::Px(5.0)),
                        justify_content: JustifyContent::Center,
                        align_items: AlignItems::Center,
                        ..default()
                    },
                    ..default()
                },
                ResumeButtonTag,
                Name::new("Resume Button")
            )).with_children(|resume_button| {
                resume_button.spawn((
                    TextBundle {
                        text: Text::from_section("Resume", TextStyle { font: fonts.thick_font.clone(), font_size: 25.0, color: Color::WHITE }),
                        ..default()
                    },
                    Name::new("Resume Button Text")
                ));
            });
        });
    });
}

pub fn cleanup_uis(
    mut commands: Commands,
    menu_root: Query<Entity, With<RootMenuTag>>,
) {
    for ent in &menu_root
    {
        commands.entity(ent).despawn_recursive();
    }
}

pub fn on_pause(
    mut pause_ui: Query<&mut Visibility, With<PauseMenuTag>>,
) {
    info!("{CLIENT_STR} Showing Pause UI");
    for mut vis in &mut pause_ui
    {
        *vis = Visibility::Inherited;
    }
}

pub fn on_resume(
    mut pause_ui: Query<&mut Visibility, With<PauseMenuTag>>,
) {
    info!("{CLIENT_STR} Hiding Pause UI");
    for mut vis in &mut pause_ui
    {
        *vis = Visibility::Hidden;
    }
}

pub fn on_enter_upgrade_select(
    mut pause_ui: Query<&mut Visibility, (With<PauseMenuTag>, Without<BreakMenuTag>, Without<HUDTag>)>,
    mut upgrade_ui: Query<&mut Visibility, (With<BreakMenuTag>, Without<PauseMenuTag>, Without<HUDTag>)>,
    mut hud: Query<&mut Visibility, (With<HUDTag>, Without<PauseMenuTag>, Without<BreakMenuTag>)>,
) {
    info!("{CLIENT_STR} Showing Upgrade UI, hiding Pause and HUD UI");
    for mut vis in &mut pause_ui
    {
        *vis = Visibility::Hidden;
    }
    for mut vis in &mut hud
    {
        *vis = Visibility::Hidden;
    }
    for mut vis in &mut upgrade_ui
    {
        *vis = Visibility::Inherited;
    }
}

pub fn on_upgrade_select_to_fighting(
    mut upgrade_ui: Query<&mut Visibility, (With<BreakMenuTag>, Without<HUDTag>)>,
    mut hud: Query<&mut Visibility, (With<HUDTag>, Without<BreakMenuTag>)>,
) {
    info!("{CLIENT_STR} Showing HUD UI, hiding Upgrade UI");
    for mut vis in &mut hud
    {
        *vis = Visibility::Inherited;
    }
    for mut vis in &mut upgrade_ui
    {
        *vis = Visibility::Hidden;
    }
}

pub fn handle_resume_button(
    buttons: Query<&Interaction, (With<ResumeButtonTag>, Changed<Interaction>)>,
    mut st_events: EventWriter<ToClients<ServerStateEvent>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    for int in &buttons
    {
        if int != &Interaction::Pressed
        {
            continue;
        }

        info!("Resume Button pressed");
        st_events.send(ToClients { mode: SendMode::Broadcast, event: ServerStateEvent::ResumeFighting });
        next_state.set(InGameState::Fighting);
    }
}

pub fn s_handle_next_wave_button(
    wave_buttons: Query<&Interaction, (With<NextWaveButtonTag>, Changed<Interaction>)>,
    mut st_events: EventWriter<ToClients<ServerStateEvent>>,
    mut next_state: ResMut<NextState<InGameState>>,
) {
    for int in &wave_buttons
    {
        if int != &Interaction::Pressed
        {
            continue;
        }

        info!("Next Wave Button pressed");
        // TODO: do something?
        st_events.send(ToClients { mode: SendMode::Broadcast, event: ServerStateEvent::BreakToFighting });
        next_state.set(InGameState::Fighting);
    }
}

// #[cfg(test)]
// mod tests
// {
//     /// Probably best to test this once code has been organised into smaller plugins
//     use bevy::app::App;

//     use crate::simple::state::InGameState;


//     #[test]
//     pub fn test_ui_elements()
//     {
//         let mut app = App::new()
//             .add_state::<InGameState>()
//             .add_systems(Update, (

//             ));


//     }
// }
