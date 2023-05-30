use bevy::{app::AppExit, prelude::*};

use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use super::{AppState, SCOREBAR_HEIGHT, ScoreText};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.8, 0.15, 0.6);

const BUTTON_WIDTH: f32 = 200.;
const BUTTON_HEIGHT: f32 = 50.;

#[derive(Component)]
struct StartButton;

#[derive(Component)]
struct ExitButton;

#[derive(Resource)]
struct MenuData {
    button_entitys: [Entity; 2],
}

pub struct MenuPlugin;

impl Plugin for MenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(AppState::Menu), setup_menu);
        app.add_systems(OnExit(AppState::Menu), cleanup_menu);
        app.add_systems(Update, button_system.run_if(in_state(AppState::Menu)));
        app.add_systems(Update, start_button_system.run_if(in_state(AppState::Menu)));
        app.add_systems(Update, exit_button_system.run_if(in_state(AppState::Menu)));
        app.add_systems(Update, update_rescale.run_if(in_state(AppState::Menu)));
        app.add_systems(Update, update_rescale_text.run_if(in_state(AppState::Menu)));
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    for &entity in menu_data.button_entitys.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_menu(mut commands: Commands, asset_server: Res<AssetServer>) {
    let start_button = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100. * (1. + SCOREBAR_HEIGHT/WINDOW_HEIGHT)),
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
                            width: Val::Px(BUTTON_WIDTH),
                            height: Val::Px(BUTTON_HEIGHT),
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
        })
        .id();

    let exit_button = commands
        .spawn(NodeBundle {
            style: Style {
                width: Val::Percent(100.),
                height: Val::Percent(100. * (1. + SCOREBAR_HEIGHT/WINDOW_HEIGHT)),
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
                            width: Val::Px(BUTTON_WIDTH),
                            height: Val::Px(BUTTON_HEIGHT),
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
        })
        .id();

    commands.insert_resource(MenuData {
        button_entitys: [start_button, exit_button],
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

fn exit_button_system(
    mut exit: EventWriter<AppExit>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<ExitButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            exit.send(AppExit);
        }
    }
}

fn start_button_system(
    mut next_state: ResMut<NextState<AppState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<StartButton>)>,
) {
    for interaction in interaction_query.iter() {
        if *interaction == Interaction::Clicked {
            next_state.set(AppState::InGame);
        }
    }
}

fn update_rescale(mut query: Query<&mut Style, With<Button>>, windows: Query<&Window>) {
    for mut style in query.iter_mut() {
        style.width = Val::Px(BUTTON_WIDTH * windows.iter().next().unwrap().width() / WINDOW_WIDTH);
        style.height = Val::Px(BUTTON_HEIGHT * windows.iter().next().unwrap().height() / WINDOW_HEIGHT);
    }
}

fn update_rescale_text(mut query: Query<&mut Text, Without<ScoreText>>, windows: Query<&Window>) {
    for mut text in query.iter_mut() {
        text.sections[0].style.font_size = 40. * windows.iter().next().unwrap().width() / WINDOW_WIDTH;
    }
}