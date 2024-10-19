#![allow(clippy::type_complexity)]
// This directive disables a specific Clippy lint warning about complex types.
// Rust's Clippy tool helps enforce good coding practices, and here we
// allow complex types which might otherwise be flagged.
///
/// ------- DESIGN CHOICES (actually not really) ----------------
/// This template follows a similar structure to the `bevy_new_2d` project.
/// `bevy_new_2d` (previously known as `bevy_quickstart`) is a community template
/// designed for Bevy, a popular game engine in Rust.
///
/// For reference, you can find the `bevy_new_2d` template here:
/// https://github.com/TheBevyFlock/bevy_new_2d
///
/// ------- CODE ORGANIZATION ------------------------------------
///
/// - **screens folder**: This folder contains logic for different UI screens.
///   Each screen represents a different game state, such as the main menu,
///   inventory, or pause screen. The code manages window attributes, like
///   showing the cursor or freezing the game while a screen (e.g., inventory) is active.
///   Example: Implementing a screen to show an inventory where the player can't
///   control the character but can interact with the UI.
///
/// - **game folder**: This folder contains the core game logic when the game is in the "Playing" state.
///   It includes player controls, animations, and overall gameplay features.
///
/// ------- NATIVE DIRECTORIES ----------------------------------
///
/// - **main.rs**: This is the entry point for desktop platforms (Windows, MacOS, Linux) and WebAssembly (WASM).
///
/// - **src/mobile/src/lib.rs**: This is the entry point for mobile platforms (iOS and Android).
///
/// ------- CUSTOM BUILD TARGETS --------------------------------
/// This template allows you to add new platforms or extend the existing ones with platform-specific code.
///
/// - You can add a new native directory to handle platform-specific functionality.
///   For example, in the `src/mobile/src/controller.rs`, you could implement mobile controller support,
///   then integrate that plugin into the mobile build by registering it in `lib.rs`.
///
/// - You can also add Android-specific code by checking if the game is being compiled for Android,
///   and adding Android features accordingly. You can apply a similar pattern to desktop builds.
///
/// ------- UNIVERSAL TARGETS -----------------------------------
/// All code inside `src/lib.rs` is shared across all platforms, making it universal.
/// This means it works for all targets: desktop, mobile, and web platforms alike.
pub mod audio; // This module manages the game's audio (e.g., background music, sound effects).
pub mod game; // This module handles the game logic, including player controls and animations.
pub mod states; // This module contains the screen (UI) management logic.

use crate::audio::InternalAudioPlugin; // Import the custom audio plugin for managing sound.
use crate::states::screens::Screen; // Import the Screen state, which manages different game screens (e.g., menu, playing).

use bevy::app::App; // Bevy's core app structure that drives the game engine.
#[cfg(debug_assertions)]
use bevy::diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin};
// These plugins are used for debugging and performance tracking (e.g., monitoring frame rates).
use bevy::prelude::*; // A collection of common imports from Bevy to make development easier.

/// This struct defines the core game plugin for Bevy.
/// In Bevy, plugins are used to group related functionality (systems, resources, etc.)
/// and add them to the app. This `GamePlugin` will be responsible for managing all
/// game-related features and systems.
pub struct GamePlugin;

impl Plugin for GamePlugin {
    /// The `build` function is called when the plugin is added to the Bevy app.
    /// It's responsible for registering the necessary systems, states, and plugins
    /// for this game.
    ///
    /// The `app` parameter is a mutable reference to the `App` instance,
    /// allowing us to modify and extend it with new functionality.
    fn build(&self, app: &mut App) {
        // Add core plugins to the app:
        // 1. `screens::plugin`: Manages the different UI screens (e.g., menu, playing).
        // 2. `InternalAudioPlugin`: Manages the audio (e.g., background music, sound effects).
        // 3. `game::plugin`: Handles the core game logic, such as player movement and animations.
        app.add_plugins((
            states::plugin,     // Plugin for screen (UI) management.
            InternalAudioPlugin, // Plugin for handling audio in the game.
            game::plugin,        // Plugin for the core game mechanics.
        ));

        // Disable multisampling, which can be used for anti-aliasing.
        app.insert_resource(Msaa::Off);
        // Set the clear color of the window, which is the background color when nothing is rendered.
        app.insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4))); // A grey background color.

        // The following block only runs in debug mode (during development).
        // It adds diagnostic plugins to help developers monitor performance.
        #[cfg(debug_assertions)]
        {
            app.add_plugins((
                FrameTimeDiagnosticsPlugin, // Plugin to track frame times, useful for performance tuning.
                LogDiagnosticsPlugin::default(), // Logs performance data (like frame time) to the console.
            ));
        }
    }
}
