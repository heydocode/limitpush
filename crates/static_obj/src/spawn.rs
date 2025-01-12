use dep_reexp::bevy::prelude::*;
use dep_reexp::bevy::ecs as bevy_ecs;

pub struct SpawnStaticObjects;

impl Plugin for SpawnStaticObjects {
    fn build(&self, app: &mut App) {
        app.add_systems(PreStartup, setup);
    }
}

#[derive(Component)]
pub struct Light;

fn setup(
    mut commands: Commands,
) {
    commands.spawn((PointLight::default(), Light, Transform::from_xyz(3., 4., 6.), Name::from("Light")));
}