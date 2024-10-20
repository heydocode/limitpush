use bevy::{prelude::*, render::view::RenderLayers};
use bevy_panorbit_camera::PanOrbitCamera;

use crate::states::screens::Screen;
use super::player::Player;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, player_camera.run_if(in_state(Screen::Playing)));
    app.add_systems(Startup, spawn_camera);
}

#[derive(Component)]
pub struct MainCamera {
    pub min_distance: f32,
    pub min_speed: f32,
    pub max_distance: f32,
    pub max_speed: f32,
    pub ideal_distance: f32,
    pub height_offset: f32,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
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
        RenderLayers::default(),
    ));
}

fn player_camera(
    player_query: Query<&Player, With<Player>>, // TODO! WHERE IS THE PROBLEM WHY WHEN THE CAMERA GO AWAY FROM THE PLAYER THE CAMERA DONT GO TO THE PLAYER?
    mut camera_query: Query<(&mut Transform, &MainCamera), With<MainCamera>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_position = player_transform.position + Vec3::Y * 3.0;

        if let Ok((mut camera_transform, camera)) = camera_query.get_single_mut() {
            // Target position directly behind the player, adjusted for ideal distance
            let target_position = player_position - Vec3::Z * camera.ideal_distance;
            let target_position = Vec3::new(
                target_position.x,
                player_transform.position.y + camera.height_offset,
                target_position.z,
            );

            // Interpolate camera movement using a damping factor for smooth transitions
            let damping_factor = 0.1;
            camera_transform.translation = camera_transform
                .translation
                .lerp(target_position, damping_factor);

            // Calculate distance and speed factor for camera movement
            let distance = camera_transform.translation.distance(player_position);
            if distance > camera.min_distance {
                let speed_factor = (distance - camera.min_distance)
                    / (camera.max_distance - camera.min_distance);
                let speed = camera.min_speed + speed_factor * (camera.max_speed - camera.min_speed);

                // Move camera smoothly towards the target
                let direction = (target_position - camera_transform.translation).normalize();
                camera_transform.translation += direction * speed * time.delta_seconds();
            }

            // Ensure the camera always looks at the player
            camera_transform.look_at(player_position, Vec3::Y);
        }
    }
}
