use avian3d::prelude::*;
use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(PhysicsDebugPlugin::default());
    #[cfg(not(target_family = "wasm"))]
    bevy_mod_debugdump::print_schedule_graph(app, PhysicsSchedule);
}
