use bevy::prelude::*;

pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, _app: &mut App) {}
}

#[derive(Component)]
pub struct Paddle;

#[derive(Component)]
pub struct Ball;
