use avian3d::{math::*, prelude::*};
use bevy::prelude::*;
use states::screens::Screen;

use crate::{movement::CharacterControllerBundle, Player};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Pipeline), setup_player);
}

pub fn setup_player(
    mut commands: Commands,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut meshes: ResMut<Assets<Mesh>>,
) {
    commands
        .spawn((
            SpatialBundle {
                transform: Transform::from_xyz(0.0, 20.0, 0.0),
                ..default()
            },
            Name::new("Player"),
            Player {
                health: 1.0,
                min_speed: 1.5,
                max_speed: 8.0,
                jump_intensity: 10.0,
            },
            CharacterControllerBundle::new(Collider::capsule(0.6, 2.0), Vector::NEG_Y * 9.81 * 2.0)
                .with_movement(30.0, 0.92, 7.0, (30.0 as Scalar).to_radians()),
        ))
        .with_children(|parent| {
            parent.spawn(PbrBundle {
                mesh: meshes.add(Capsule3d::new(0.6, 2.0)),
                material: materials.add(StandardMaterial {
                    base_color: Color::srgb(0.0, 0.0, 0.0),
                    ..default()
                }),
                ..default()
            });
        });
}
