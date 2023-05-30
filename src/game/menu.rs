use bevy::prelude::*;

use super::{AppState, SCOREBAR_HEIGHT};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.8, 0.15, 0.6);

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct ExitButton;

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu);
        app.add_systems(Update, button_system.run_if(in_state(AppState::Menu)));
    }
}

fn setup_menu(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(windows.single().height() + SCOREBAR_HEIGHT),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(65.),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    StartButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Start Game",
                        TextStyle {
                            font: asset_server.load("./font/Teko-SemiBold.ttf"),
                            font_size: 40.,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });

    commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Px(windows.single().height() + SCOREBAR_HEIGHT),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
                ..default()
            },
            ..default()
        })
        .with_children(|parent| {
            parent
                .spawn((
                    ButtonBundle {
                        style: Style {
                            width: Val::Px(150.),
                            height: Val::Px(65.),
                            // horizontally center child text
                            justify_content: JustifyContent::Center,
                            // vertically center child text
                            align_items: AlignItems::Center,
                            ..default()
                        },
                        background_color: NORMAL_BUTTON.into(),
                        ..default()
                    },
                    ExitButton,
                ))
                .with_children(|parent| {
                    parent.spawn(TextBundle::from_section(
                        "Exit",
                        TextStyle {
                            font: asset_server.load("./font/Teko-SemiBold.ttf"),
                            font_size: 40.,
                            color: Color::rgb(0.9, 0.9, 0.9),
                        },
                    ));
                });
        });
}

fn button_system(
    mut interaction_query: Query<
        (&Interaction, &mut BackgroundColor),
        (Changed<Interaction>, With<Button>),
    >,
) {
    for (interaction, mut color) in &mut interaction_query {
        match *interaction {
            Interaction::Clicked => {
                *color = PRESSED_BUTTON.into();
            }
            Interaction::Hovered => {
                *color = HOVERED_BUTTON.into();
            }
            Interaction::None => {
                *color = NORMAL_BUTTON.into();
            }
        }
    }
}
