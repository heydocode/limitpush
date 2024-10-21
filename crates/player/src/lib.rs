#![allow(clippy::type_complexity)]

use bevy::prelude::*;

use states::screens::{loading::PlayerAssets, Screen};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Pipeline), spawn_player);
    app.add_systems(Update, apply_movement.run_if(in_state(Screen::Playing)));
    app.register_type::<Player>();
}

#[derive(Component, Reflect)]
pub struct Player {
    pub health: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub jump_intensity: f32,
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
    mut player_query: Query<(&mut Transform, &Player)>,
) {
    if let Ok((mut player_transform, player)) = player_query.get_single_mut() {
        let speed: f32 = match input.pressed(KeyCode::ShiftLeft) {
            true => player.max_speed,
            false => player.min_speed,
        };

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
        player_transform.translation.x += direction.x * speed;
        player_transform.translation.y += direction.y * speed;
        player_transform.translation.z += direction.z * speed;
    }
}
