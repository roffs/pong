use std::f32::consts;

use bevy::{prelude::*, sprite::MaterialMesh2dBundle};
use rand::random;

pub struct BallPlugin;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_ball)
            .add_systems(Update, update_ball_position);
    }
}

#[derive(Component)]
pub struct Ball {
    direction: Vec3,
    speed: f32,
}

pub fn spawn_ball(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    commands.spawn((
        MaterialMesh2dBundle {
            mesh: meshes.add(shape::Circle::new(15.0).into()).into(),
            material: materials.add(ColorMaterial::from(Color::WHITE)),
            transform: Transform::from_translation(Vec3::new(0., 0., 0.)),
            ..default()
        },
        Ball {
            direction: Vec3::new(random(), random(), 0.0).normalize(),
            speed: 200.0,
        },
    ));
}

pub fn update_ball_position(mut ball_query: Query<(&mut Transform, &Ball)>, time: Res<Time>) {
    let (mut transform, ball) = ball_query.single_mut();

    transform.translation += ball.direction * ball.speed * time.delta_seconds();
}
