use self::play::PlayPlugin;
use bevy::{prelude::*, render::camera::ScalingMode, sprite::Anchor};

pub mod play;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugin(PlayPlugin);
        app.add_systems(Startup, setup);
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
        texture: asset_server.load("Board.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("Board.png"),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("ScoreBar.png"),
        sprite: Sprite {
            anchor: Anchor::TopLeft,
            ..default()
        },
        transform: Transform::from_xyz(
            -windows.single().width() / 2.,
            windows.single().height() / 2.,
            0.,
        ),
        ..default()
    });

    commands.spawn(SpriteBundle {
        texture: asset_server.load("ScoreBar.png"),
        sprite: Sprite {
            anchor: Anchor::TopRight,
            flip_x: true,
            ..default()
        },
        transform: Transform::from_xyz(
            windows.single().width() / 2.,
            windows.single().height() / 2.,
            0.,
        ),
        ..default()
    });
}
