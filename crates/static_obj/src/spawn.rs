use dep_reexp::bevy::prelude::*;
use data::components::Light;

pub struct SpawnStaticObjects;

impl Plugin for SpawnStaticObjects {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

fn setup(
    mut commands: Commands,
) {
    commands.spawn((PointLight::default(), Light, Transform::from_xyz(3., 4., 6.)));
}