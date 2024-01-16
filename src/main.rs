mod ball;
mod player;

use ball::BallPlugin;
use bevy::prelude::*;
use player::PlayerPlugin;

fn main() {
    App::new()
        .insert_resource(ClearColor(Color::rgb(0.3647, 0.6627, 0.9607)))
        .add_plugins(DefaultPlugins)
        .add_plugins((PlayerPlugin, BallPlugin))
        .add_systems(Startup, setup)
        .run();
}

#[derive(Component)]
struct Wall;

pub const WALL_HEIGHT: f32 = 15.0;

fn setup(mut commands: Commands, window_query: Query<&Window>) {
    // Spawn camera
    commands.spawn(Camera2dBundle::default());

    // Spawn top and bottom walls
    let window = window_query.get_single().unwrap();

    let wall_width = window.width();

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(wall_width, WALL_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.,
                (window.height() - WALL_HEIGHT) / 2.,
                0.,
            )),
            ..default()
        },
        Wall,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(wall_width, WALL_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                0.,
                (-window.height() + WALL_HEIGHT) / 2.,
                0.,
            )),
            ..default()
        },
        Wall,
    ));
}
