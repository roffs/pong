use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::random;

use crate::{
    player::{Player1, Player2, PLAYER_HEIGHT, PLAYER_WIDTH},
    WALL_HEIGHT,
};

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball).add_systems(
            Update,
            (
                update_ball_position,
                bounce_ball_on_walls,
                bounce_ball_on_players,
            ),
        );
    }
}

#[derive(Component)]
pub struct Ball {
    direction: Vec3,
    speed: f32,
}

const BALL_RADIUS: f32 = 15.0;

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ball {
            direction: Vec3::new(random(), random(), 0.0).normalize(),
            speed: 400.0,
        },
    ));
}

fn update_ball_position(mut ball_query: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    let (mut transform, ball) = ball_query.single_mut();

    transform.translation += ball.direction * ball.speed * time.delta_seconds();
}

fn bounce_ball_on_walls(
    mut ball_query: Query<(&Transform, &mut Ball)>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok((transform, mut ball)) = ball_query.get_single_mut() {
        let max_y = (window.height() / 2.0) - BALL_RADIUS - WALL_HEIGHT;
        let min_y = (-window.height() / 2.0) + BALL_RADIUS + WALL_HEIGHT;

        if transform.translation.y > max_y || transform.translation.y < min_y {
            ball.direction.y *= -1.0;
        }
    }
}

fn bounce_ball_on_players(
    mut ball_query: Query<(&Transform, &mut Ball)>,
    player_query: Query<&Transform, AnyOf<(&Player1, &Player2)>>,
) {
    if let Ok((ball_transform, mut ball)) = ball_query.get_single_mut() {
        for player_transform in player_query.into_iter() {
            let horizontal_collision_check =
                (player_transform.translation.x - ball_transform.translation.x).abs()
                    < PLAYER_WIDTH / 2.0 + BALL_RADIUS;

            let vertical_collision_check =
                (player_transform.translation.y - ball_transform.translation.y).abs()
                    < PLAYER_HEIGHT / 2.0 + BALL_RADIUS;

            if horizontal_collision_check && vertical_collision_check {
                ball.direction.x *= -1.0;
                ball.speed += 15.0;
            }
        }
    }
}
