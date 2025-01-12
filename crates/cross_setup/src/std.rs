use dep_reexp::bevy::prelude::*;
use crate::bevy_ecs;

pub struct StdPlugin;

impl Plugin for StdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

#[derive(Component)]
pub struct Camera;

fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Camera, Transform::from_xyz(2.5, 2.5, 2.5).looking_at(Vec3::ZERO, Vec3::Z)));
}
