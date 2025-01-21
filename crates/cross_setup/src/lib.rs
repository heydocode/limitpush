#![cfg_attr(feature = "embedded", no_std)]

mod plugins;
#[cfg(any(feature = "desktop", feature = "mobile", target_family = "wasm"))]
pub(crate) mod std;
#[cfg(feature = "terminal")]
pub(crate) mod terminal;
#[cfg(feature = "xr")]
// [ ] Implement
pub(crate) mod xr;
use bevy::prelude::*;
use plugins::PluginsSetup;

pub struct CrossSetupPlugin;

impl Plugin for CrossSetupPlugin {
    fn build(&self, app: &mut App) {
        // Ensure only one platform-specific feature is enabled at a time
        #[cfg(all(
    any(
        all(feature = "desktop", feature = "mobile"),
        all(feature = "desktop", target_family = "wasm"),
        all(feature = "desktop", feature = "terminal"),
        all(feature = "desktop", feature = "embedded"),
        all(feature = "mobile", target_family = "wasm"),
        all(feature = "mobile", feature = "terminal"),
        all(feature = "mobile", feature = "embedded"),
        all(target_family = "wasm", feature = "terminal"),
        all(target_family = "wasm", feature = "embedded"),
        all(feature = "terminal", feature = "embedded"),
        all(feature = "xr", feature = "desktop"),
        all(feature = "xr", feature = "mobile"),
        all(feature = "xr", feature = "terminal"),
        all(feature = "xr", feature = "embedded"),
        all(feature = "xr", target_family = "wasm"),
    ),
    not(clippy) // Skip this check when running Clippy
))]
        compile_error!(
            "cross_setup: Please enable only one platform-specific feature at a time!\n\
            Valid features: 'desktop', 'mobile', 'terminal', 'embedded', 'xr'.\n\
            Note: 'wasm' is automatically considered a platform-specific feature when targeting WebAssembly.\n\
            Example: `--features desktop` or `--target wasm32-unknown-unknown`."
);

        // Ensure at least one platform-specific feature is enabled, unless targeting wasm
        #[cfg(all(
    not(any(
        feature = "desktop",
        feature = "mobile",
        target_family = "wasm",
        feature = "terminal",
        feature = "embedded",
        feature = "xr",
    )),
    not(clippy) // Skip this check when running Clippy
))]
        compile_error!(
            "cross_setup: Please enable one platform-specific feature!\n\
            Valid features: 'desktop', 'mobile', 'terminal', 'embedded', 'xr'.\n\
            Note: If compiling to WebAssembly, no additional feature needs to be enabled.\n\
            Example: `--features mobile` or `--target wasm32-unknown-unknown`."
        );

        app.add_plugins(PluginsSetup);
    }
}
