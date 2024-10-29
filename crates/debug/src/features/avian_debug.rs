use avian3d::prelude::PhysicsDebugPlugin;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsDebugPlugin::default());
}
