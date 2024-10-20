use bevy::{prelude::*, render::view::RenderLayers};
use bevy_panorbit_camera::PanOrbitCamera;

use super::player::Player;
use crate::states::screens::Screen;

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
            min_distance: 0.0,
            min_speed: 0.05,
            max_distance: 8.0,
            max_speed: 50.0,
            ideal_distance: 7.5,
            height_offset: 2.5,
        },
        PanOrbitCamera {
            modifier_pan: Some(KeyCode::Abort),
            // Fix all target-specific issues
            // Indeed, for some reason PanOrbitCamera
            // works differently on wasm builds...
            zoom_upper_limit: Some(1000.0),
            ..default()
        },
        RenderLayers::default(),
    ));
}

fn player_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &MainCamera), Without<Player>>,
    time: Res<Time>,
) {
    if let Ok(player_transform) = player_query.get_single() {
        let player_position = player_transform.translation + Vec3::Y * 3.0;

        if let Ok((mut camera_transform, camera)) = camera_query.get_single_mut() {
            let target_position = player_position - Vec3::Z * camera.ideal_distance;
            let target_position = Vec3::new(
                target_position.x,
                player_transform.translation.y + camera.height_offset,
                target_position.z,
            );

            let direction = (target_position - camera_transform.translation).normalize();
            let distance = camera_transform.translation.distance(target_position);

            // Move the camera smoothly towards the target
            if distance > camera.min_distance {
                let speed_factor =
                    (distance - camera.min_distance) / (camera.max_distance - camera.min_distance);
                let speed = camera.min_speed + speed_factor * (camera.max_speed - camera.min_speed);
                camera_transform.translation += direction * speed * time.delta_seconds();
            } else {
                camera_transform.translation = target_position; // Snap to position if close enough
            }

            // Always look at the player
            camera_transform.look_at(player_position, Vec3::Y);
        } else {
            #[cfg(all(feature = "dev", not(target_family = "wasm")))]
            eprintln!("The camera components aren't available");
        }
    } else {
        #[cfg(all(feature = "dev", not(target_family = "wasm")))]
        eprintln!("The player is out of scope and is no more calculated");
    }
}
