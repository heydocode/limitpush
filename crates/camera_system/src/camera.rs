use bevy::{prelude::*, render::view::RenderLayers};

use crate::pan_orbit_camera::{self};

use crate::pan_orbit_camera::PanOrbitCamera;
use player::Player;
use states::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, player_camera.run_if(in_state(Screen::Playing)));
    app.add_systems(Startup, spawn_camera);
    app.add_plugins(pan_orbit_camera::plugin);
    app.register_type::<MainCamera>();
}

#[derive(Component, Reflect)]
pub struct MainCamera {
    pub min_distance: f32,
    pub min_speed: f32,
    pub max_distance: f32,
    pub max_speed: f32,
    pub ideal_distance: f32,
    pub height_offset: f32,
}

// The decay rate used by the smooth following:
#[derive(Resource)]
struct DecayRate(f32);

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0).looking_at(Vec3::ZERO, Vec3::Y),
            ..default()
        },
        Name::new("Camera"),
        MainCamera {
            min_distance: 0.0,
            min_speed: 0.05,
            max_distance: 8.0,
            max_speed: 50.0,
            ideal_distance: 7.5,
            height_offset: 4.0,
        },
        PanOrbitCamera {
            // Fix all target-specific issues
            // Indeed, for some reason PanOrbitCamera
            // works differently on wasm builds...
            zoom_upper_limit: Some(1000.0),
            force_update: true,
            ..default()
        },
        RenderLayers::default(),
    ));

    commands.insert_resource(DecayRate(2.0));
}

fn player_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &MainCamera), Without<Player>>,
    time: Res<Time>,
    decay_rate: Res<DecayRate>, // Add decay rate for smoother movement
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

            let distance = camera_transform.translation.distance(target_position);
            let delta_time = time.delta_seconds();
            let decay_rate = decay_rate.0; // Use the decay rate for smooth interpolation

            // Apply smooth movement towards the target using a decay rate
            if distance > camera.min_distance {
                camera_transform.translation = camera_transform
                    .translation
                    .lerp(target_position, decay_rate * delta_time);
            } else {
                camera_transform.translation = target_position; // Snap to position if close enough
            }

            // Always look at the player
            camera_transform.look_at(player_position, Vec3::Y);
        }
    }
}
