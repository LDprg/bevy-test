use bevy::{prelude::*, sprite::Anchor};

const PADDLE_WIDTH: f32 = 17.;

enum Player {
    Player1,
    Player2,
}

#[derive(Component)]
pub struct Paddle(Player);

#[derive(Component)]
pub struct Ball;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/Player.png"),
            sprite: Sprite {
                anchor: Anchor::CenterLeft,
                ..default()
            },
            transform: Transform::from_xyz(PADDLE_WIDTH - windows.single().width() / 2., 0., 1.),
            ..default()
        },
        Paddle(Player::Player1),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/Computer.png"),
            sprite: Sprite {
                anchor: Anchor::CenterRight,
                ..default()
            },
            transform: Transform::from_xyz(-PADDLE_WIDTH + windows.single().width() / 2., 0., 1.),
            ..default()
        },
        Paddle(Player::Player2),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/Ball.png"),
            transform: Transform::from_xyz(0., 0., 1.),
            ..default()
        },
        Ball,
    ));
}
