#![cfg_attr(feature = "embedded", no_std)]

#[cfg(any(feature = "desktop", feature = "mobile", target_family = "wasm"))]
pub(crate) mod std;
#[cfg(feature = "terminal")]
pub(crate) mod terminal;
mod plugins;
use plugins::PluginsSetup;
#[cfg(all(feature = "debug", feature = "desktop"))]
mod debug;
#[cfg(all(feature = "debug", feature = "desktop"))]
use debug::DebugPlugin;
use dep_reexp::bevy::prelude::*;

// I don't know why, but it fixes an error
pub(crate) use dep_reexp::bevy::ecs as bevy_ecs;

pub struct CrossSetupPlugin;

impl Plugin for CrossSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PluginsSetup);
        #[cfg(all(feature = "debug", feature = "desktop"))]
        app.add_plugins(DebugPlugin);
    }
}