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
    // TODO! Load reusable material and mesh assets for performance optimization
    let beige_material = materials.add(StandardMaterial {
        base_color: Color::srgb(1.0, 0.9, 0.7),
        ..default()
    });
    let chestnut_material = materials.add(StandardMaterial {
        base_color: Color::srgb(0.6, 0.3, 0.2),
        ..default()
    });
    let white_material = materials.add(StandardMaterial {
        base_color: Color::WHITE,
        ..default()
    });

    // Load common meshes
    #[allow(clippy::useless_conversion)]
    let sphere_mesh = Mesh::from(Sphere::new(4.).mesh().uv(4, 2));
    #[allow(clippy::useless_conversion)]
    let cuboid_mesh = Mesh::from(Cuboid::new(0.3, 0.6, 0.3)); // Reusable for arms, legs, and fingers

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
            // TODO! Construct the character's body parts with individual shapes

            // **Head**: Beige sphere with cuboid eyebrows and eyes
            parent
                .spawn(PbrBundle {
                    mesh: meshes.add(sphere_mesh.clone()),
                    material: beige_material.clone(),
                    transform: Transform::from_xyz(0.0, 2.5, 0.0),
                    ..default()
                })
                .with_children(|head| {
                    // **Eyebrows**
                    head.spawn(PbrBundle {
                        mesh: meshes.add(cuboid_mesh.clone()),
                        material: chestnut_material.clone(),
                        transform: Transform::from_xyz(-0.2, 0.2, 0.4),
                        ..default()
                    });
                    head.spawn(PbrBundle {
                        mesh: meshes.add(cuboid_mesh.clone()),
                        material: chestnut_material.clone(),
                        transform: Transform::from_xyz(0.2, 0.2, 0.4),
                        ..default()
                    });

                    // **Eyes**
                    head.spawn(PbrBundle {
                        mesh: meshes.add(cuboid_mesh.clone()),
                        material: white_material.clone(),
                        transform: Transform::from_xyz(-0.15, 0.1, 0.5),
                        ..default()
                    });
                    head.spawn(PbrBundle {
                        mesh: meshes.add(cuboid_mesh.clone()),
                        material: white_material.clone(),
                        transform: Transform::from_xyz(0.15, 0.1, 0.5),
                        ..default()
                    });
                });

            // **Arms**: Two cuboids per arm for upper and lower segments
            parent
                .spawn(PbrBundle {
                    mesh: meshes.add(cuboid_mesh.clone()),
                    material: beige_material.clone(),
                    transform: Transform::from_xyz(-0.8, 1.5, 0.0),
                    ..default()
                })
                .with_children(|arm| {
                    arm.spawn(PbrBundle {
                        mesh: meshes.add(cuboid_mesh.clone()),
                        material: beige_material.clone(),
                        transform: Transform::from_xyz(0.0, -0.6, 0.0),
                        ..default()
                    });
                });

            parent
                .spawn(PbrBundle {
                    mesh: meshes.add(cuboid_mesh.clone()),
                    material: beige_material.clone(),
                    transform: Transform::from_xyz(0.8, 1.5, 0.0),
                    ..default()
                })
                .with_children(|arm| {
                    arm.spawn(PbrBundle {
                        mesh: meshes.add(cuboid_mesh.clone()),
                        material: beige_material.clone(),
                        transform: Transform::from_xyz(0.0, -0.6, 0.0),
                        ..default()
                    });
                });

            // **Hands**: Six cuboids each (four for fingers, two for thumb)
            for x_offset in [-1.1, 1.1].iter() {
                parent
                    .spawn(SpatialBundle {
                        transform: Transform::from_xyz(*x_offset, 0.7, 0.0),
                        ..default()
                    })
                    .with_children(|hand| {
                        for i in 0..4 {
                            hand.spawn(PbrBundle {
                                mesh: meshes.add(cuboid_mesh.clone()),
                                material: beige_material.clone(),
                                transform: Transform::from_xyz(0.0, -0.1 * i as f32, 0.0),
                                ..default()
                            });
                        }
                        hand.spawn(PbrBundle {
                            mesh: meshes.add(cuboid_mesh.clone()),
                            material: beige_material.clone(),
                            transform: Transform::from_xyz(0.1, -0.3, 0.0),
                            ..default()
                        });
                        hand.spawn(PbrBundle {
                            mesh: meshes.add(cuboid_mesh.clone()),
                            material: beige_material.clone(),
                            transform: Transform::from_xyz(-0.1, -0.3, 0.0),
                            ..default()
                        });
                    });
            }

            // **Legs**: Two cuboids per leg
            for x_offset in [-0.5, 0.5].iter() {
                parent
                    .spawn(SpatialBundle {
                        transform: Transform::from_xyz(*x_offset, 1.0, 0.0),
                        ..default()
                    })
                    .with_children(|leg| {
                        leg.spawn(PbrBundle {
                            mesh: meshes.add(cuboid_mesh.clone()),
                            material: beige_material.clone(),
                            transform: Transform::from_xyz(0.0, -0.5, 0.0),
                            ..default()
                        });
                    });
            }

            // **Feet**: One cuboid per foot at the bottom of each leg
            for x_offset in [-0.5, 0.5].iter() {
                parent.spawn(PbrBundle {
                    mesh: meshes.add(cuboid_mesh.clone()),
                    material: chestnut_material.clone(),
                    transform: Transform::from_xyz(*x_offset, 0.2, 0.1),
                    ..default()
                });
            }
        });
}
