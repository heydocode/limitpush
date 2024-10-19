use bevy::{prelude::*, render::view::RenderLayers};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::states::screens::Screen;

use super::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, (
        player_camera, 
    ).run_if(in_state(Screen::Playing)));
    app.add_systems(OnEnter(Screen::Loading), spawn_camera);
}

#[derive(Component)]
pub struct MainCamera {
    pub min_distance: f64,
    pub min_speed: f64,
    pub max_distance: f64,
    pub max_speed: f64,
    pub ideal_distance: f64,
    pub height_offset: f32,
}

fn spawn_camera(mut commands: Commands) {
    let camera = (
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        MainCamera {
            min_distance: 0.1,
            min_speed: 0.05,
            max_distance: 10.0,
            max_speed: 15.0,
            ideal_distance: 7.5,
            height_offset: 2.5,
        },
        PanOrbitCamera::default(),
    );

    commands.spawn(camera).insert(RenderLayers::default());
}

fn player_camera(
    player: Query<&Transform, (With<Player>, Without<MainCamera>)>,
    mut camera: Query<(&mut Transform, &MainCamera), With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player.get_single() {
        // Define offsets and calculate the player position with the height offset
        let player_position = Vec3::new(
            player_transform.translation.x,
            player_transform.translation.y + 3.0, // Base height offset
            player_transform.translation.z,
        );

        if let Ok((mut camera_transform, main_camera)) = camera.get_single_mut() {
            let camera_position = camera_transform.translation;

            // Ideal camera position behind the player with a specific distance
            let ideal_position = player_position - Vec3::new(0.0, 0.0, main_camera.ideal_distance as f32);

            // Adjust height based on player movement
            let dynamic_height = player_transform.translation.y + main_camera.height_offset;
            let target_position = Vec3::new(
                ideal_position.x,
                dynamic_height, // Adjust height dynamically
                ideal_position.z,
            );

            // Smooth camera movement using a damping factor for more fluidity
            let damping_factor = 0.1; // Adjust damping for smoother transitions
            let smooth_position = camera_position.lerp(target_position, damping_factor);

            // Calculate the distance between the ideal position and the current camera position
            let distance = ideal_position.distance(camera_position);

            // Move the camera based on distance and speed factor
            if distance > main_camera.min_distance as f32 {
                let speed_factor = (distance - main_camera.min_distance as f32)
                    / (main_camera.max_distance as f32 - main_camera.min_distance as f32);
                let speed = main_camera.min_speed as f32
                    + speed_factor * (main_camera.max_speed as f32 - main_camera.min_speed as f32);

                // Calculate the direction towards the target
                let direction = (smooth_position - camera_position).normalize();

                // Move the camera smoothly towards the target position
                camera_transform.translation += direction * speed * time.delta_seconds();
            }

            // Optional: Collision avoidance logic can be added here
            // For example, using raycasting to check for obstacles

            // Make the camera look at the player position
            camera_transform.look_at(player_position, Vec3::Y);
        }
    }
}