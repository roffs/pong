use bevy::{prelude::*, sprite::MaterialMesh2dBundle};

use crate::ball::{Ball, BALL_RADIUS};

#[derive(Resource, Default)]
struct Score {
    left: u32,
    right: u32,
}

pub struct ScorePlugin;

impl Plugin for ScorePlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<Score>()
            .add_event::<Goal>()
            .add_systems(Update, (detect_goals, process_goal));
    }
}

#[derive(Event)]
enum Goal {
    Left,
    Right,
}

fn detect_goals(
    mut event: EventWriter<Goal>,
    ball: Query<&Transform, With<Ball>>,
    window_query: Query<&Window>,
) {
    let window = window_query.get_single().unwrap();

    if let Ok(transform) = ball.get_single() {
        if (transform.translation.x - BALL_RADIUS) < (-window.width() / 2.0) {
            event.send(Goal::Left);
        }

        if (transform.translation.x + BALL_RADIUS) > (window.width() / 2.0) {
            event.send(Goal::Right);
        }
    }
}

fn process_goal(
    mut commands: Commands,
    ball_query: Query<Entity, With<Ball>>,
    mut goal_event: EventReader<Goal>,
    mut score: ResMut<Score>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for goal in goal_event.read() {
        match goal {
            Goal::Left => score.left += 1,
            Goal::Right => score.right += 1,
        };

        if let Ok(ball) = ball_query.get_single() {
            commands.entity(ball).despawn();
        }

        commands.spawn((
            MaterialMesh2dBundle {
                mesh: meshes.add(shape::Circle::new(BALL_RADIUS).into()).into(),
                material: materials.add(ColorMaterial::from(Color::WHITE)),
                transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
                ..default()
            },
            Ball::new(),
        ));
    }

    println!("SCORE: {} - {}", score.left, score.right);
}
