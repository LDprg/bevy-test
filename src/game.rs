use self::menu::MenuPlugin;
use self::play::PlayPlugin;
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

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_plugin(PlayPlugin);
        app.add_plugin(MenuPlugin);
        app.add_state::<AppState>();
    }
}

fn setup(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    commands.spawn(Camera2dBundle {
        projection: OrthographicProjection {
            scaling_mode: ScalingMode::FixedVertical(windows.single().height()),
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
        transform: Transform::from_xyz(
            -windows.single().width() / 2.,
            windows.single().height() / 2.,
            1.,
        ),
        ..default()
    });

    commands.spawn(
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
            left: Val::Px(windows.single().width() / 4.),
            ..default()
        }),
    );

    commands.spawn(SpriteBundle {
        texture: score_bar,
        sprite: Sprite {
            anchor: Anchor::TopRight,
            flip_x: true,
            ..default()
        },
        transform: Transform::from_xyz(
            windows.single().width() / 2.,
            windows.single().height() / 2.,
            1.,
        ),
        ..default()
    });

    commands.spawn(
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
            right: Val::Px(windows.single().width() / 4.),
            ..default()
        }),
    );
}
