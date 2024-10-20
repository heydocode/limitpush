// use avian3d::prelude::{Collider, RigidBody};
use bevy::{
    color::palettes::tailwind::*, ecs::world::Command, prelude::*,
    render::mesh::VertexAttributeValues, utils::HashMap,
};
use bevy_panorbit_camera::PanOrbitCameraPlugin;
use noise::{BasicMulti, NoiseFn, Perlin};
use std::f32::consts::PI;

use crate::{game::player::Player, states::screens::Screen};

pub(super) fn plugin(app: &mut App) {
    app.insert_resource(TerrainStore(HashMap::default()))
        .add_plugins((PanOrbitCameraPlugin,))
        .add_systems(OnEnter(Screen::Pipeline), startup)
        .add_systems(Update, (manage_chunks,).run_if(in_state(Screen::Playing)));
}

fn startup(mut commands: Commands) {
    commands.add(SpawnTerrain(IVec2::new(-1, -1)));
    commands.add(SpawnTerrain(IVec2::new(-1, 0)));
    commands.add(SpawnTerrain(IVec2::new(-1, 1)));
    commands.add(SpawnTerrain(IVec2::new(0, -1)));
    commands.add(SpawnTerrain(IVec2::new(0, 0)));
    commands.add(SpawnTerrain(IVec2::new(0, 1)));
    commands.add(SpawnTerrain(IVec2::new(1, -1)));
    commands.add(SpawnTerrain(IVec2::new(1, 0)));
    commands.add(SpawnTerrain(IVec2::new(1, 1)));

    // directional 'sun' light
    commands.spawn(DirectionalLightBundle {
        directional_light: DirectionalLight {
            illuminance: light_consts::lux::OVERCAST_DAY,
            shadows_enabled: true,
            ..default()
        },
        transform: Transform {
            translation: Vec3::new(0.0, 2.0, 0.0),
            rotation: Quat::from_rotation_x(-PI / 4.),
            ..default()
        },
        ..default()
    });
}

#[derive(Resource)]
struct TerrainStore(HashMap<IVec2, Handle<Mesh>>);

struct SpawnTerrain(IVec2);

impl Command for SpawnTerrain {
    fn apply(self, world: &mut World) {
        if world
            .get_resource_mut::<TerrainStore>()
            .expect("TerrainStore to be available")
            .0
            .get(&self.0)
            .is_some()
        {
            // mesh already exists
            // do nothing for now
            warn!("mesh {} already exists", self.0);
            return;
        };

        let terrain_height = 70.;
        let noise = BasicMulti::<Perlin>::new(900);
        let mesh_size = 1000.;

        let mut terrain = Mesh::from(
            Plane3d::default()
                .mesh()
                .size(mesh_size, mesh_size)
                .subdivisions(50),
        );

        if let Some(VertexAttributeValues::Float32x3(positions)) =
            terrain.attribute_mut(Mesh::ATTRIBUTE_POSITION)
        {
            for pos in positions.iter_mut() {
                let val = noise.get([
                    (pos[0] as f64 + (mesh_size as f64 * self.0.x as f64)) / 300., // From 300. to 500.
                    (pos[2] as f64 + (mesh_size as f64 * self.0.y as f64)) / 300., // From 300. to 500.
                ]);

                pos[1] = val as f32 * terrain_height;
            }

            let colors: Vec<[f32; 4]> = positions
                .iter()
                .map(|[_, g, _]| {
                    let g = *g / terrain_height * 2.;

                    if g > 0.8 {
                        (Color::LinearRgba(LinearRgba {
                            red: 20.,
                            green: 20.,
                            blue: 20.,
                            alpha: 1.,
                        }))
                        .to_linear()
                        .to_f32_array()
                    } else if g > 0.3 {
                        Color::from(AMBER_800).to_linear().to_f32_array()
                        // Color::from(YELLOW_500).to_linear().to_f32_array()
                    } else if g < -0.8 {
                        Color::BLACK.to_linear().to_f32_array()
                        // Color::WHITE.to_linear().to_f32_array()
                    } else {
                        (Color::from(GREEN_400).to_linear()).to_f32_array()
                        // (Color::from(BLUE_400).to_linear()).to_f32_array()
                    }
                })
                .collect();
            terrain.insert_attribute(Mesh::ATTRIBUTE_COLOR, colors);
        }
        terrain.compute_normals();

        let mesh = world
            .get_resource_mut::<Assets<Mesh>>()
            .expect("meshes db to be available")
            .add(terrain.clone());
        let material = world
            .get_resource_mut::<Assets<StandardMaterial>>()
            .expect("StandardMaterial db to be available")
            .add(Color::WHITE);

        world
            .get_resource_mut::<TerrainStore>()
            .expect("TerrainStore to be available")
            .0
            .insert(self.0, mesh.clone());

        world.spawn((
            PbrBundle {
                mesh,
                material,
                transform: Transform::from_xyz(
                    self.0.x as f32 * mesh_size,
                    0.,
                    self.0.y as f32 * mesh_size,
                ),
                ..default()
            },
            Terrain,
            Name::new("Procedurally-generated terrain"),
            // RigidBody::Static,
            // Collider::convex_decomposition_from_mesh(&Mesh::from(terrain)).unwrap(),
        ));
        // eprintln!("The plane mesh isn't a real 3D mesh so there is no active collider...");
    }
}

fn manage_chunks(
    mut commands: Commands,
    mut current_chunk: Local<IVec2>,
    ship: Query<&Transform, With<Player>>,
    mut terrain_store: ResMut<TerrainStore>,
    terrain_entities: Query<(Entity, &Handle<Mesh>), With<Terrain>>,
) {
    // same as mesh_size for us
    let chunk_size = 1000.;

    let Ok(transform) = ship.get_single() else {
        warn!("no ship!");
        return;
    };

    let xz = (transform.translation.xz() / chunk_size).trunc().as_ivec2();

    if *current_chunk != xz {
        *current_chunk = xz;
        let chunks_to_render = [
            *current_chunk + IVec2::new(-1, -1),
            *current_chunk + IVec2::new(-1, 0),
            *current_chunk + IVec2::new(-1, 1),
            *current_chunk + IVec2::new(0, -1),
            *current_chunk + IVec2::new(0, 0),
            *current_chunk + IVec2::new(0, 1),
            *current_chunk + IVec2::new(1, -1),
            *current_chunk + IVec2::new(1, 0),
            *current_chunk + IVec2::new(1, 1),
        ];
        // extract_if is perfect here, but its nightly
        let chunks_to_despawn: Vec<(IVec2, Handle<Mesh>)> = terrain_store
            .0
            .clone()
            .into_iter()
            .filter(|(key, _)| !chunks_to_render.contains(key))
            .collect();

        for (chunk, mesh) in chunks_to_despawn {
            let Some((entity, _)) = terrain_entities.iter().find(|(_, handle)| handle == &&mesh)
            else {
                continue;
            };
            commands.entity(entity).despawn_recursive();
            terrain_store.0.remove(&chunk);
        }

        for chunk in chunks_to_render {
            commands.add(SpawnTerrain(chunk));
        }
    }
}

#[derive(Component)]
pub struct Terrain;
