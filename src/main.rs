use crate::game::GamePlugin;
use bevy::{prelude::*, window::PresentMode};

pub mod game;

const WINDOW_WIDTH: f32 = 802.;
const WINDOW_HEIGHT: f32 = 455.;

fn main() {
    App::new()
        .insert_resource(Msaa::Sample4)
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Ldprg Pong!".into(),
                resolution: (WINDOW_WIDTH, WINDOW_HEIGHT).into(),
                resize_constraints: WindowResizeConstraints {
                    min_width: WINDOW_WIDTH,
                    min_height: WINDOW_HEIGHT,
                    ..Default::default()
                },
                present_mode: PresentMode::AutoVsync,
                fit_canvas_to_parent: true,
                prevent_default_event_handling: false,
                ..default()
            }),
            ..default()
        }))
        .add_plugin(GamePlugin)
        .run();
}
