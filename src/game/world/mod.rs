use bevy::prelude::*;
// use avian3d::prelude::Collider;
// use avian3d::prelude::RigidBody;
#[cfg(feature = "dev")]
#[cfg(not(target_family = "wasm"))]
use bevy_inspector_egui::quick::WorldInspectorPlugin;
use levels::openworld::Terrain;

pub mod levels;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        levels::plugin,
        #[cfg(feature = "dev")]
        #[cfg(not(target_family = "wasm"))]
        WorldInspectorPlugin::new(),
    ));
}

#[derive(Bundle)]
pub struct TerrainBundle {
    pub terrain: Terrain,
    pub pbr: PbrBundle,
    pub name: Name,
    // pub body: RigidBody,
    // pub collider: Collider,
}
