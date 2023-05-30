use bevy::{app::AppExit, prelude::*};

use super::{AppState, SCOREBAR_HEIGHT};

const NORMAL_BUTTON: Color = Color::rgb(0.15, 0.15, 0.15);
const HOVERED_BUTTON: Color = Color::rgb(0.25, 0.25, 0.25);
const PRESSED_BUTTON: Color = Color::rgb(0.8, 0.15, 0.6);

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
    }
}

fn cleanup_menu(mut commands: Commands, menu_data: Res<MenuData>) {
    for &entity in menu_data.button_entitys.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

fn setup_menu(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    let start_button = commands
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
        })
        .id();

    let exit_button = commands
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
        match *interaction {
            Interaction::Clicked => {
                exit.send(AppExit);
            }
            _ => {}
        }
    }
}

fn start_button_system(
    mut next_state: ResMut<NextState<AppState>>,
    interaction_query: Query<&Interaction, (Changed<Interaction>, With<Button>, With<StartButton>)>,
) {
    for interaction in interaction_query.iter() {
        match *interaction {
            Interaction::Clicked => {
                next_state.set(AppState::InGame);
            }
            _ => {}
        }
    }
}
