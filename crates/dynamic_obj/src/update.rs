use data::components::Cube;
use dep_reexp::bevy::prelude::*;

pub struct UpdateDynamicObjects;

impl Plugin for UpdateDynamicObjects {
    fn build(&self, app: &mut App) {
        app.add_systems(Update, rotate_cube_system);
    }
}

fn rotate_cube_system(time: Res<Time>, mut cube: Query<&mut Transform, With<Cube>>) {
    // USEFUL Please do these checks for debug, because in debug you
    //  can delete/edit entities on the fly, and because that let you
    //  know where is the problem, but at release build it will only
    //  slow down the game (assuming you've debuged it correctly)
    #[cfg(feature = "debug")]
    {
        if let Err(_) = cube.get_single_mut() {
            error!("An error occured: unable to get mutable cube!");
        } else {
            cube.single_mut().rotate_z(time.delta_secs());
        }
    }

    #[cfg(not(feature = "debug"))]
    // NOTE Make sure it doesn't crash!
    cube.single_mut().rotate_z(time.delta_secs());
}
