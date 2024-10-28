use avian3d::debug_render::PhysicsDebugPlugin;
use avian3d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsDebugPlugin::default());
    bevy_mod_debugdump::print_schedule_graph(app, PhysicsSchedule);
}
