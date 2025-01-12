use crate::spawn::Cube;
use dep_reexp::bevy::prelude::*;

pub struct UpdateDynamicObjects;

impl Plugin for UpdateDynamicObjects {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotate_cube_system);
    }
}

fn rotate_cube_system(time: Res<Time>, mut cube: Query<&mut Transform, With<Cube>>) {
    // NOTE: Ensuring an instance of cube exists is a good practice.
    if let Err(_) = cube.get_single_mut() {
        error!("An error occured: unable to get mutable cube!");
    } else {
        cube.single_mut().rotate_z(time.delta_secs());
    }
}
