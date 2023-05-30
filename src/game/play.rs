use rand::Rng;
use std::f32::consts::PI;

use bevy::{prelude::*, sprite::Anchor};

use super::{AppState, Score, PADDLE_HEIGHT, PADDLE_WIDTH, SCOREBAR_HEIGHT, SPEED};

const BALL_SPEED: f32 = 500.;
const BALL_RADIUS: f32 = 15.;

const BALL_START: Vec3 = Vec3::new(0., -SCOREBAR_HEIGHT / 2., 1.);

#[derive(Debug, PartialEq, Eq, Clone)]
pub enum Player {
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
pub struct Ball {
    velocity: f32,
    direction: Vec2,
}

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_systems(Update, move_paddle.run_if(in_state(AppState::InGame)));
        app.add_systems(Update, move_ball.run_if(in_state(AppState::InGame)));
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

    let mut ball_direction =
        Vec2::from_angle(rand::thread_rng().gen_range(PI / 4...PI * 3. / 4.)).rotate(Vec2::Y);
    ball_direction.x *= rand::thread_rng().gen_bool(0.5) as i32 as f32 * -2. + 1.;

    commands.spawn((
        SpriteBundle {
            texture: asset_server.load("img/Ball.png"),
            transform: Transform::from_xyz(BALL_START.x, BALL_START.y, BALL_START.z),
            ..default()
        },
        Ball {
            velocity: BALL_SPEED,
            direction: ball_direction,
        },
    ));

    println!(
        "{:?}",
        Vec2::from_angle(rand::thread_rng().gen_range(0. ..=PI * 2.))
    );
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

fn reset_ball(transform: &mut Transform, ball: &mut Ball) {
    transform.translation = BALL_START;
    ball.direction =
        Vec2::from_angle(rand::thread_rng().gen_range(PI / 8...PI * 3. / 8.)).rotate(Vec2::Y);
    ball.direction.x *= rand::thread_rng().gen_bool(0.5) as i32 as f32 * -2. + 1.;
    ball.direction.y *= rand::thread_rng().gen_bool(0.5) as i32 as f32 * -2. + 1.;
}

fn move_ball(
    time: Res<Time>,
    windows: Query<&Window>,
    mut score: ResMut<Score>,
    mut query: Query<(&mut Transform, &mut Ball), Without<Paddle>>,
    query_paddle: Query<(&Transform, &Paddle), Without<Ball>>,
) {
    for (mut transform, mut ball) in query.iter_mut() {
        ball.direction.x += rand::thread_rng().gen_range(-0.01..0.01);
        ball.direction.y += rand::thread_rng().gen_range(-0.01..0.01);
        ball.direction = ball.direction.normalize();

        let mut translation = transform.translation
            + (ball.direction * ball.velocity * time.delta_seconds()).extend(0.);

        // Walls y
        let translation_y = translation.y.clamp(
            -windows.single().height() / 2. + BALL_RADIUS,
            windows.single().height() / 2. - BALL_RADIUS - SCOREBAR_HEIGHT,
        );

        if translation.y != translation_y {
            ball.direction.y *= -1.;
        }

        // Walls x
        let translation_x = translation
            .x
            .max(-windows.single().width() / 2. + BALL_RADIUS);

        if translation.x != translation_x {
            reset_ball(&mut transform, &mut ball);
            score.player2 += 1;
            continue;
        }

        let translation_x = translation
            .x
            .min(windows.single().width() / 2. - BALL_RADIUS);

        if translation.x != translation_x {
            reset_ball(&mut transform, &mut ball);
            score.player1 += 1;
            continue;
        }

        // Paddles
        let translation_x = translation
            .x
            .max(-windows.single().width() / 2. + BALL_RADIUS + PADDLE_WIDTH * 2.);

        for (paddle_transform, paddle) in query_paddle.iter() {
            if paddle.0 == Player::Player1
                && translation.x != translation_x
                && translation_y <= paddle_transform.translation.y + PADDLE_HEIGHT / 2.
                && translation_y >= paddle_transform.translation.y - PADDLE_HEIGHT / 2.
            {
                ball.direction.x *= -1.;

                translation.x = (transform.translation
                    + (ball.direction * ball.velocity * time.delta_seconds()).extend(0.))
                .x;

                break;
            }
        }

        let translation_x = translation
            .x
            .min(windows.single().width() / 2. - BALL_RADIUS - PADDLE_WIDTH * 2.);

        for (paddle_transform, paddle) in query_paddle.iter() {
            if paddle.0 == Player::Player2
                && translation.x != translation_x
                && translation_y <= paddle_transform.translation.y + PADDLE_HEIGHT / 2.
                && translation_y >= paddle_transform.translation.y - PADDLE_HEIGHT / 2.
            {
                ball.direction.x *= -1.;

                translation.x = (transform.translation
                    + (ball.direction * ball.velocity * time.delta_seconds()).extend(0.))
                .x;
                break;
            }
        }

        // Correct X limits
        let translation_x = translation.x.clamp(
            -windows.single().width() / 2. + BALL_RADIUS,
            windows.single().width() / 2. - BALL_RADIUS,
        );

        transform.translation.y = translation_y;
        transform.translation.x = translation_x;
    }
}
