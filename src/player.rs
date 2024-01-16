use bevy::prelude::*;

use crate::WALL_HEIGHT;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players)
            .add_systems(Update, (move_players, confine_players).chain());
    }
}

#[derive(Component)]
pub struct Player;

pub const PLAYER_HEIGHT: f32 = 150.0;
pub const PLAYER_WIDTH: f32 = 15.0;

fn spawn_players(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();

    let gap = 10.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                ((-window.width() + PLAYER_WIDTH) / 2.0) + gap,
                0.,
                0.,
            )),
            ..default()
        },
        Player,
    ));

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(PLAYER_WIDTH, PLAYER_HEIGHT)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                ((window.width() - PLAYER_WIDTH) / 2.0) - gap,
                0.0,
                0.,
            )),
            ..default()
        },
        Player,
    ));
}

const PLAYER_SPEED: f32 = 500.0;
fn move_players(
    mut players_query: Query<&mut Transform, With<Player>>,
    input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    for mut transform in players_query.iter_mut() {
        if input.pressed(KeyCode::W) {
            transform.translation += Vec3::Y * PLAYER_SPEED * time.delta_seconds()
        }

        if input.pressed(KeyCode::S) {
            transform.translation -= Vec3::Y * PLAYER_SPEED * time.delta_seconds()
        }
    }
}

fn confine_players(
    mut players_query: Query<&mut Transform, With<Player>>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();

    let max_y = ((window.height() - PLAYER_HEIGHT) / 2.0) - WALL_HEIGHT;
    let min_y = ((-window.height() + PLAYER_HEIGHT) / 2.0) + WALL_HEIGHT;

    for mut transform in players_query.iter_mut() {
        transform.translation.y = transform.translation.y.clamp(min_y, max_y);
    }
}
