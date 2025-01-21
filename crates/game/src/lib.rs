use bevy::prelude::*;

mod cube;
#[cfg(feature = "debug")]
mod debug;
mod light;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup);
        app.add_plugins((cube::plugin, light::plugin));
        #[cfg(feature = "debug")]
        app.add_plugins(debug::DebugPlugin);
    }
}

// TODO
#[allow(dead_code, unused_variables, unused_mut)]
fn setup(mut commands: Commands) {}
