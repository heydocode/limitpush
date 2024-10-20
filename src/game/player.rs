// use avian3d::prelude::{Collider, RigidBody};
use bevy::prelude::*;

use crate::states::screens::{loading::PlayerAssets, Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Pipeline), spawn_player);
    app.add_systems(Update, apply_movement.run_if(in_state(Screen::Playing)));
    app.register_type::<Player>();
}

#[derive(Component, Reflect)]
pub struct Player {
    pub health: f64,
    pub min_speed: f64,
    pub max_speed: f64,
    pub jump_intensity: f64,
}

fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    let player = (
        SceneBundle {
            scene: player_assets.base_character.clone(),
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            ..default()
        },
        Name::new("Player"),
        Player {
            health: 1.0,
            min_speed: 1.5,
            max_speed: 8.0,
            jump_intensity: 10.0,
        },
    );

    commands.spawn(player);
}

fn apply_movement(
    input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<&mut Transform, With<Player>>,
) {
    let mut transform = player_query.single_mut();
    let mut direction = Vec3::ZERO;
    if input.pressed(KeyCode::KeyW) {
        direction.z += 1.;
    }
    if input.pressed(KeyCode::KeyS) {
        direction.z -= 1.;
    }
    if input.pressed(KeyCode::KeyA) {
        direction.x += 1.;
    }
    if input.pressed(KeyCode::KeyD) {
        direction.x -= 1.;
    }
    if input.pressed(KeyCode::Space) {
        direction.y += 1.;
    }
    if input.pressed(KeyCode::ShiftLeft) {
        direction.y -= 1.;
    }
    transform.translation.x += direction.x * 1.;
    transform.translation.y += direction.y * 1.;
    transform.translation.z += direction.z * 1.;
}
