#![allow(clippy::type_complexity)] // Allow complex types flagged by Clippy (optional).

use avian3d::PhysicsPlugins;
/// This project follows the structure of the `bevy_new_2d` template.
/// For reference: https://github.com/TheBevyFlock/bevy_new_2d.
///
/// # Code Organization:
///
/// - **screens/**: Handles different UI states (e.g., main menu, pause, inventory).
///   Manages window attributes, like cursor visibility and gameplay freezing during screens.
///   
/// - **game/**: Core gameplay logic when the game is in the "Playing" state (player controls, animations).
///
/// # Platform Targets:
///
/// - **main.rs**: Entry point for desktop (Windows, MacOS, Linux) and WebAssembly (WASM).
/// - **src/mobile/src/lib.rs**: Entry point for mobile (iOS and Android).
///
/// You can also extend the template for custom platforms, adding platform-specific directories
/// and functionalities as needed, such as mobile controllers or Android-specific features.
///
/// All code in **lib.rs** is shared across all platforms.
use bevy::prelude::*;
use bevy_transform_interpolation::TransformInterpolationPlugin; // Common Bevy imports for ease of development. // Debugging tools.

/// Core game plugin for Bevy.
/// This manages all game-related features and systems by bundling them into the Bevy app.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    /// The `build` function registers all necessary systems, states, and plugins for the game.
    fn build(&self, app: &mut App) {
        // Register core game plugins
        app.add_plugins((
            PhysicsPlugins::default(),
            audio::plugin, // Audio management (e.g., background music, sound effects).
            camera_system::plugin, // Camera system management.
            player::plugin, // Player controls, animations, and gameplay.
            states::plugin, // Screen state.
            world::plugin, // World spawn.
            ui::plugin,    // UI handling
            #[cfg(debug_assertions)]
            // This plugin isn't available at release to offer
            // cleaner game experience
            debug::plugin,
            TransformInterpolationPlugin {
                global_translation_interpolation: true,
                global_rotation_interpolation: true,
                global_scale_interpolation: true,
            },
        ));
    }
}
