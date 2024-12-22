use dep_reexp::bevy::prelude::*;
use data::components::Cube;

pub struct SpawnDynamicObjects;

impl Plugin for SpawnDynamicObjects {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

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