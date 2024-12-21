use dep_reexp::bevy::prelude::*;
use data::components::Cube;

pub struct UpdateDynamicObjects;

impl Plugin for UpdateDynamicObjects {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotate_cube_system);
    }
}

fn rotate_cube_system(time: Res<Time>, mut cube: Query<&mut Transform, With<Cube>>) {
    cube.single_mut().rotate_z(time.delta_secs());
}