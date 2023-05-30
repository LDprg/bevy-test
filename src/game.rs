use crate::{WINDOW_HEIGHT, WINDOW_WIDTH};

use self::play::PlayPlugin;
use self::{menu::MenuPlugin, play::Player};
use bevy::{prelude::*, render::camera::ScalingMode, sprite::Anchor};

pub mod menu;
pub mod play;

pub const PADDLE_WIDTH: f32 = 17.;
pub const PADDLE_HEIGHT: f32 = 120.;

pub const SCOREBAR_HEIGHT: f32 = 47.;

pub const SPEED: f32 = 500.;

#[derive(Debug, Clone, Copy, Default, Eq, PartialEq, Hash, States)]
pub enum AppState {
    Menu,
    #[default]
    InGame,
}

#[derive(Resource)]
pub struct Score {
    player1: u32,
    player2: u32,
}

#[derive(Component)]
pub struct ScoreText(Player);

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayPlugin);
        app.add_plugin(MenuPlugin);
        app.add_state::<AppState>();
        app.add_systems(Startup, setup);
        app.add_systems(Update, update_score.run_if(in_state(AppState::InGame)));
    }
}

fn setup(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.insert_resource(Score {
        player1: 0,
        player2: 0,
    });

    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(WINDOW_HEIGHT),
            ..default()
        },
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("./img/Board.png"),
        ..default()
    });

    let score_bar = asset_server.load("./img/ScoreBar.png");
    let font_teko = asset_server.load("./font/Teko-SemiBold.ttf");

    commands.spawn(SpriteBundle {
        texture: score_bar.clone(),
        sprite: Sprite {
            anchor: Anchor::TopLeft,
            ..default()
        },
        transform: Transform::from_xyz(-WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2., 1.),
        ..default()
    });

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font: font_teko.clone(),
                font_size: 49.,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Relative,
            top: Val::Px(0.),
            left: Val::Px(WINDOW_WIDTH / 4.),
            ..default()
        }),
        ScoreText(Player::Player1),
    ));

    commands.spawn(SpriteBundle {
        texture: score_bar,
        sprite: Sprite {
            anchor: Anchor::TopRight,
            flip_x: true,
            ..default()
        },
        transform: Transform::from_xyz(WINDOW_WIDTH / 2., WINDOW_HEIGHT / 2., 1.),
        ..default()
    });

    commands.spawn((
        TextBundle::from_section(
            "0",
            TextStyle {
                font: font_teko,
                font_size: 49.,
                color: Color::WHITE,
            },
        )
        .with_text_alignment(TextAlignment::Center)
        .with_style(Style {
            position_type: PositionType::Absolute,
            top: Val::Px(0.),
            right: Val::Px(WINDOW_WIDTH / 4.),
            ..default()
        }),
        ScoreText(Player::Player2),
    ));
}

fn update_score(score_glob: Res<Score>, mut query: Query<(&mut Text, &ScoreText)>) {
    for (mut text, score) in query.iter_mut() {
        text.sections[0].value = match score.0 {
            Player::Player1 => score_glob.player1.to_string(),
            Player::Player2 => score_glob.player2.to_string(),
        };
    }
}
