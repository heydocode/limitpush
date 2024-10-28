use avian3d::prelude::Collider;
use avian3d::prelude::RigidBody;
use bevy::prelude::*;
use states::screens::Screen;

use crate::objects_queue::QueuedObjects;

pub mod lobby;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(lobby::plugin);
    app.add_systems(OnEnter(Screen::Pipeline), spawn_level);
    app.insert_resource(EverythingSpawned(false)); // This resource allows to know if everything is loaded
    app.insert_resource(SceneIsHidden(false)); // This resource allows to know if everything is loaded

    // todo add here condition of "sceneishidden" struct
    app.add_systems(OnEnter(Screen::Playing), show_level);
    app.add_systems(OnExit(Screen::Playing), hide_level);
}

#[derive(Component)]
pub struct Object;

#[derive(Bundle)]
pub struct ObjectBundle {
    pbr: PbrBundle,
    name: Name,
    object: Object,
    body: RigidBody,
    collider: Collider,
}

#[derive(Default, Resource)]
pub struct EverythingSpawned(pub bool);

#[derive(Default, Resource)]
pub struct SceneIsHidden(pub bool);

pub fn spawn_level(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<StandardMaterial>>,
    mut queued_objects: ResMut<QueuedObjects>, // New resource for queued objects
    mut everything_spawned: ResMut<EverythingSpawned>,
) {
    if queued_objects.is_empty() {
        if everything_spawned.0 {
            println!("A function call has been avoided");
            return;
        }
        everything_spawned.0 = true;
    } else {
        println!("Spawning queued objects");

        // First HDR color
        let material_emissive1 = materials.add(StandardMaterial {
            emissive: Color::linear_rgb(23000.0, 9000.0, 3000.0).into(),
            ..default()
        });
        // Second HDR color
        let material_emissive2 = materials.add(StandardMaterial {
            emissive: Color::linear_rgb(3000.0, 23000.0, 9000.0).into(),
            ..default()
        });
        // Third HDR color
        let material_emissive3 = materials.add(StandardMaterial {
            emissive: Color::linear_rgb(9000.0, 3000.0, 23000.0).into(),
            ..default()
        });
        // Default color (black)
        let default_material = materials.add(Color::srgb(0.0, 0.0, 0.0));

        let mut object_creator = |form: String,
                                  variant: u32,
                                  size: f32,
                                  xyz: (f32, f32, f32),
                                  name: String|
         -> ObjectBundle {
            #[allow(clippy::useless_conversion)]
            let matched_mesh = match form.to_lowercase().as_str() {
                "cube" => Mesh::from(Cuboid::new(size, size, size)),
                "capsule" => Mesh::from(Capsule3d::new(size / 3.0, size)),
                "torus" => Mesh::from(Torus::new(size / 2., size * 0.75)),
                "cylinder" => Mesh::from(Cylinder::new(size / 2.0, size)),
                "plane" => Mesh::from(Cuboid::new(size, 2.0, size)),
                "platform" => Mesh::from(Cuboid::new(4.0, 1.0, 4.0)),
                "sphere" => Mesh::from(Sphere::new(size / 2.).mesh().uv(32, 18)),
                _ => Mesh::from(Plane3d::default().mesh().size(10.0, 10.0)),
            };
            ObjectBundle {
                pbr: PbrBundle {
                    mesh: meshes.add(matched_mesh.clone()),
                    material: match variant {
                        1 => material_emissive1.clone(),
                        2 => material_emissive2.clone(),
                        3 => material_emissive3.clone(),
                        12 => materials.add(Color::srgb(0.3, 0.5, 0.3)),
                        _ => default_material.clone(),
                    },
                    transform: Transform::from_xyz(xyz.0, xyz.1, xyz.2),
                    ..default()
                },
                name: Name::new(name),
                object: Object,
                body: RigidBody::Static,
                collider: Collider::convex_decomposition_from_mesh(&matched_mesh).unwrap(),
            }
        };

        // Spawn all objects in the queue
        while let Some(object_data) = queued_objects.dequeue() {
            commands.spawn(object_creator(
                object_data.form,
                object_data.variant,
                object_data.size,
                object_data.position,
                object_data.name,
            ));
        }
        everything_spawned.0 = true;
    }
}

pub fn hide_level(
    mut query: Query<(Entity, &mut Visibility), With<Object>>, // Find the objects to hide
    mut scene_is_hidden: ResMut<SceneIsHidden>,
) {
    if scene_is_hidden.0 {
        return;
    }
    scene_is_hidden.0 = true;
    for (_entity, mut visibility) in query.iter_mut() {
        *visibility = Visibility::Hidden; // Hide the object instead of despawning it
    }
}

pub fn show_level(
    mut query: Query<(Entity, &mut Visibility), With<Object>>, // Find the objects to hide
    mut scene_is_hidden: ResMut<SceneIsHidden>,
) {
    if !scene_is_hidden.0 {
        return;
    }
    for (_entity, mut visibility) in query.iter_mut() {
        *visibility = Visibility::Visible; // Show the object instead of spawning it
    }
    scene_is_hidden.0 = false;
}
