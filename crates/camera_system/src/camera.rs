use bevy::{prelude::*, render::view::RenderLayers};

use bevy_panorbit_camera::{PanOrbitCamera, PanOrbitCameraPlugin};
use player::Player;
use states::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_systems(Update, player_camera.run_if(in_state(Screen::Playing)));
    app.add_systems(Startup, spawn_camera);
    app.register_type::<MainCamera>();
    app.add_plugins(PanOrbitCameraPlugin);
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

#[derive(Resource)]
struct DecayRate(f32);

fn spawn_camera(mut commands: Commands) {
    commands.spawn((
        Camera3dBundle {
            transform: Transform::from_xyz(0.0, 10.0, 10.0),
            ..default() // TODO! Implement a collider for the camera in order to avoid the camera going through objects and colliders
        },
        Name::new("Camera"),
        MainCamera {
            min_distance: 0.0,
            min_speed: 0.05,
            max_distance: 8.0,
            max_speed: 50.0,
            ideal_distance: -15.0,
            height_offset: 5.0,
        },
        PanOrbitCamera::default(),
        RenderLayers::default(),
    ));

    commands.insert_resource(DecayRate(2.0));
}

fn player_camera(
    player_query: Query<&Transform, With<Player>>,
    mut camera_query: Query<(&mut Transform, &MainCamera), Without<Player>>,
    time: Res<Time>,
    decay_rate: Res<DecayRate>,
) {
    // TODO! Implement the camera collider here too: the camera
    // will obviously always try to get the ideal_distance, so
    // we have to change its priorities when the camera collides
    // with another collider (not KinematicCharacters, to avoid
    // camera lags on multiplayer if I'll implement that)
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
            let decay_rate = decay_rate.0;

            if distance > camera.min_distance {
                camera_transform.translation = camera_transform
                    .translation
                    .lerp(target_position, decay_rate * delta_time);
            } else {
                camera_transform.translation = target_position;
            }
            camera_transform.look_at(player_position, Vec3::Y);
        }
    }
}
