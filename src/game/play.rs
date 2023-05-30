use bevy::{prelude::*, sprite::Anchor};

use super::{AppState, PADDLE_HEIGHT, PADDLE_WIDTH, SCOREBAR_HEIGHT, SPEED};

#[derive(Debug)]
enum Player {
    Player1,
    Player2,
}

impl Player {
    fn get_keys(&self) -> (KeyCode, KeyCode) {
        match self {
            Player::Player1 => (KeyCode::W, KeyCode::S),
            Player::Player2 => (KeyCode::Up, KeyCode::Down),
        }
    }
}

#[derive(Component)]
pub struct Paddle(Player);

#[derive(Component)]
pub struct Ball;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_paddle.run_if(in_state(AppState::InGame)));
    }
}

fn setup(mut commands: Commands, windows: Query<&Window>, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/Player.png"),
            sprite: Sprite {
                anchor: Anchor::CenterLeft,
                flip_x: true,
                ..default()
            },
            transform: Transform::from_xyz(
                PADDLE_WIDTH - windows.single().width() / 2.,
                -SCOREBAR_HEIGHT / 2.,
                1.,
            ),
            ..default()
        },
        Paddle(Player::Player1),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/Computer.png"),
            sprite: Sprite {
                anchor: Anchor::CenterRight,
                flip_x: true,
                ..default()
            },
            transform: Transform::from_xyz(
                -PADDLE_WIDTH + windows.single().width() / 2.,
                -SCOREBAR_HEIGHT / 2.,
                1.,
            ),
            ..default()
        },
        Paddle(Player::Player2),
    ));

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/Ball.png"),
            transform: Transform::from_xyz(0., -SCOREBAR_HEIGHT / 2., 1.),
            ..default()
        },
        Ball,
    ));
}

fn move_paddle(
    time: Res<Time>,
    keys: Res<Input<KeyCode>>,
    windows: Query<&Window>,
    mut query: Query<(&mut Transform, &Paddle)>,
) {
    for (mut transform, paddle) in query.iter_mut() {
        let (up, down) = paddle.0.get_keys();
        let direction = if keys.pressed(up) {
            1.
        } else if keys.pressed(down) {
            -1.
        } else {
            0.
        };

        let translation = transform.translation.y + direction * time.delta_seconds() * SPEED;
        transform.translation.y = translation.clamp(
            -windows.single().height() / 2. + PADDLE_HEIGHT / 2.,
            windows.single().height() / 2. - PADDLE_HEIGHT / 2. - SCOREBAR_HEIGHT,
        );
    }
}
