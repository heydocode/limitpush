use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(PreStartup, setup);
    app.add_systems(Update, rotate_cube_system);
}

#[derive(Component)]
pub struct Cube;

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
) {
    commands.spawn((
        Cube,
        Mesh3d(meshes.add(Cuboid::default())),
        MeshMaterial3d(materials.add(StandardMaterial {
            base_color: Color::srgb(0.6, 0.7, 0.9),
            ..Default::default()
        })),
        Name::from("Cube"),
    ));
    commands.spawn((
        Mesh3d(meshes.add(Cuboid::new(15., 15., 1.))),
        Transform::from_xyz(0., 0., -6.),
    ));
}

fn rotate_cube_system(time: Res<Time>, mut cube: Query<&mut Transform, With<Cube>>) {
    // NOTE: Ensuring an instance of cube exists is a good practice.
    if cube.get_single_mut().is_err() {
        error!("An error occured: unable to get mutable cube!");
    } else {
        cube.single_mut().rotate_z(time.delta_secs());
    }
}
