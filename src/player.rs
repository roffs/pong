use bevy::prelude::*;

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_players)
            .add_systems(Update, move_players);
    }
}

#[derive(Component)]
struct Player;

fn spawn_players(mut commands: Commands, window_query: Query<&Window>) {
    let window = window_query.get_single().unwrap();

    let player_height = 150.0;
    let player_width = 15.0;

    let gap = 10.0;

    commands.spawn((
        SpriteBundle {
            sprite: Sprite {
                color: Color::WHITE,
                custom_size: Some(Vec2::new(player_width, player_height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                ((-window.width() + player_width) / 2.0) + gap,
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
                custom_size: Some(Vec2::new(player_width, player_height)),
                ..default()
            },
            transform: Transform::from_translation(Vec3::new(
                ((window.width() - player_width) / 2.0) - gap,
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
