use bevy::prelude::*;

// I don't know why, but it fixes an error
use bevy::ecs as bevy_ecs;

pub struct XrPlugin;

impl Plugin for XrPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

#[derive(Component)]
pub struct Camera;

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        Camera3d::default(),
        Camera,
        Transform::from_xyz(2.5, 2.5, 2.5).looking_at(Vec3::ZERO, Vec3::Z),
    ));
}
