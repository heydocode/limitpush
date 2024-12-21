use data::components::Camera;
use dep_reexp::bevy::prelude::*;

pub struct StdPlugin;

impl Plugin for StdPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, camera_setup);
    }
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((Camera3d::default(), Camera, Transform::from_xyz(2.5, 2.5, 2.5).looking_at(Vec3::ZERO, Vec3::Z)));
}
