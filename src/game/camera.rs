use bevy::{prelude::*, render::view::RenderLayers};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::screens::Screen;

use super::player::Player;

pub(super) fn plugin(app: &mut App) {
    // app.add_plugins(playing_camera::plugin);
    app.add_systems(OnEnter(Screen::Loading), spawn_camera);
}

#[derive(Component)]
pub struct MainCamera {
    pub min_distance: f64,
    pub min_speed: f64,
    pub max_distance: f64,
    pub max_speed: f64,
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            // The camera should change transform & be linked to a player when the loading screen stops
            transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera {
            min_distance: 1.0,
            min_speed: 0.1,
            max_distance: 10.0,
            max_speed: 15.0,
        },
        PanOrbitCamera::default(),
    );

    commands.spawn(camera).insert(RenderLayers::default());
}

fn _player_camera(
    q_target: Query<&Transform, (With<Player>, Without<MainCamera>)>, // Query the player's transform
    mut q_camera: Query<(&mut Transform, &MainCamera), With<MainCamera>>, // Query the camera's transform and MainCamera component
    time: Res<Time>, // Time resource for delta time (used for smooth movement)
) {
    // Get the player's transform
    if let Ok(player_transform) = q_target.get_single() {
        let player_position = player_transform.translation;

        // Get the camera's transform and its MainCamera component
        if let Ok((mut camera_transform, main_camera)) = q_camera.get_single_mut() {
            let camera_position = camera_transform.translation;

            // Calculate the distance between the player and the camera
            let distance = player_position.distance(camera_position);

            // Only move the camera if the distance is greater than the minimum distance
            if distance as f64 > main_camera.min_distance {
                // Calculate the speed based on the distance
                let speed_factor = (distance as f64 - main_camera.min_distance)
                    / (main_camera.max_distance - main_camera.min_distance);
                let speed = main_camera.min_speed
                    + speed_factor * (main_camera.max_speed - main_camera.min_speed);

                // Calculate direction to move the camera
                let direction = (player_position - camera_position).normalize();

                // Move the camera towards the player, scaled by speed and delta time
                camera_transform.translation += direction * speed as f32 * time.delta_seconds();

                eprintln!("The camera has been updated!");
            }

            // Make the camera look at the player
            camera_transform.look_at(player_position, Vec3::Y);
        }
    }
}
