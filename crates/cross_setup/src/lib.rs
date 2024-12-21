#![cfg_attr(feature = "embedded", no_std)]

#[cfg(any(feature = "desktop", feature = "mobile", target_family = "wasm"))]
pub(crate) mod std;
#[cfg(feature = "terminal")]
pub(crate) mod terminal;
mod plugins;
use plugins::PluginsSetup;
use dep_reexp::bevy::prelude::*;

pub struct CrossSetupPlugin;

impl Plugin for CrossSetupPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PluginsSetup);
    }
}