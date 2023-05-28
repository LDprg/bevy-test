use bevy::prelude::*;
use bevy_prototype_lyon::prelude::*;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
    }
}

fn setup(mut commands: Commands) {
    let shape = shapes::RegularPolygon {
        sides: 6,
        feature: shapes::RegularPolygonFeature::Radius(200.0),
        ..shapes::RegularPolygon::default()
    };

    commands.spawn(Camera2dBundle::default());
    commands.spawn(
        ShapeBundle {
            path: GeometryBuilder::build_as(&shape),
            ..default()
        }
    );
}
