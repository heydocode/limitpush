#![no_std]
#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use cross_setup::CrossSetupPlugin;
use game::GamePlugin;

pub struct AssemblerPlugin;

impl Plugin for AssemblerPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((CrossSetupPlugin, GamePlugin));

        #[cfg(feature = "debug")]
        info!("Dear Developer! The LimitPush template offers astonishing cross-platform abilities (WebGL2 (WebGPU soon), Windows, MacOS, Graphics based Linux + Terminal based Linux, Android, iOS and most important: embedded! (soon)) while keeping easy debugging and extending it. But to extend its cross-platform abilities, please open an issue on github instead of modifying the code yourself (cross_setup and dep_reexp crates are very unstable on edits)!");
    }
}
