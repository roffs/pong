mod player;

use bevy::prelude::*;
use player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3647, 0.6627, 0.9607)))
        .add_plugins(DefaultPlugins)
        .add_plugins(PlayerPlugin)
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Wall;

fn setup(mut commands: Commands, window_query: Query<&Window>) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn top and bottom walls
    let window = window_query.get_single().unwrap();

    let wall_height = 15.0;
    let wall_width = window.width();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(wall_width, wall_height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.,
                (window.height() - wall_height) / 2.,
                0.,
            )),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::rgb(1.0, 1.0, 1.0),
                custom_size: Some(Vec2::new(wall_width, wall_height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.,
                (-window.height() + wall_height) / 2.,
                0.,
            )),
            ..default()
        },
        Wall,
    ));
}