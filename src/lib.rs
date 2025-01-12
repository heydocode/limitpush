#![no_std]
#![allow(clippy::type_complexity)]

use dep_reexp::bevy::prelude::*;
use cross_setup::CrossSetupPlugin;
use r#static_obj::StaticObjectsPlugin;
use dynamic_obj::DynamicObjectsPlugin;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
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

                all(feature = "terminal", feature = "embedded")
            ),
            not(clippy)
        ))]
        compile_error!("Please enable only one platform-specific feature! Mind about: when compiling to wasm, wasm becomes automatically a platform-specific feature.");

        #[cfg(all(not(any(
            feature = "desktop", feature = "mobile", target_family = "wasm", feature = "terminal", feature = "embedded"
        )), not(clippy)))]
        compile_error!("Please enable one platform-specific feature! (not needed if the compilation target is wasm)");

        #[cfg(all(not(all(feature = "debug", feature = "desktop")), feature = "debug", not(clippy)))]
        compile_error!("Feature \"debug\" is intended to work with \"desktop\" platform-specific feature!");

        app.add_plugins((CrossSetupPlugin, StaticObjectsPlugin, DynamicObjectsPlugin));

        #[cfg(feature = "debug")]
        info!("Dear Developer! The LimitPush template offers astonishing cross-platform abilities (WebGL2 (WebGPU soon), Windows, MacOS, Graphics based Linux + Terminal based Linux, Android, iOS and most important: embedded! (soon)) while keeping easy debugging and extending it. But to extend its cross-platform abilities, please open an issue on github instead of modifying the code yourself (cross_setup and dep_reexp crates are very unstable on edits)!");
    }
}
