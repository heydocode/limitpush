// This line disables the console window on Windows when compiling in release mode.
// It helps provide a cleaner experience for applications that do not require console output.
// The `cfg_attr` attribute conditionally applies the `windows_subsystem` attribute
// if the build is not in debug mode (i.e., for release builds).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::asset::AssetMetaCheck;
// Importing asset management checks from Bevy.
use bevy::prelude::*;
#[cfg(not(target_family = "wasm"))]
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
#[cfg(not(target_family = "wasm"))]
use bevy::render::RenderPlugin;
// Importing common Bevy types and traits for ease of use.
use bevy::DefaultPlugins; // Default plugins provided by Bevy for standard functionalities.
use limitpush::GamePlugin; // Importing the GamePlugin, which contains core game logic.

fn main() {
    // Create a new Bevy application instance.
    App::new()
        .add_plugins((
            #[cfg(not(target_family = "wasm"))]
            DefaultPlugins // Add default plugins for basic functionality (input handling, rendering, etc.).
                // Configure the WindowPlugin with specific settings.
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "LimitPush".to_string(), // Set the window title for the application.
                        // Bind to a canvas specified in the HTML file for WASM builds.
                        canvas: Some("#bevy".to_owned()), // The HTML canvas where Bevy will render.
                        fit_canvas_to_parent: true, // Adjust the canvas to fit its parent element.
                        // Prevent default event handling for WASM (like F5 and Ctrl+R).
                        prevent_default_event_handling: false,
                        visible: false, // The window module is handling everything related to the window!
                        ..default()     // Use default settings for the rest of the Window struct.
                    }),
                    ..default() // Use default settings for the rest of the WindowPlugin.
                })
                // Configure the AssetPlugin to never perform asset metadata checks (improving performance).
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default() // Use default settings for the rest of the AssetPlugin.
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        // WARN this is a native only feature. It will not work with webgl or webgpu
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                }),
            #[cfg(target_family = "wasm")]
            DefaultPlugins // Add default plugins for basic functionality (input handling, rendering, etc.).
                // Configure the WindowPlugin with specific settings.
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "LimitPush".to_string(), // Set the window title for the application.
                        // Bind to a canvas specified in the HTML file for WASM builds.
                        canvas: Some("#bevy".to_owned()), // The HTML canvas where Bevy will render.
                        fit_canvas_to_parent: true, // Adjust the canvas to fit its parent element.
                        // Prevent default event handling for WASM (like F5 and Ctrl+R).
                        prevent_default_event_handling: false,
                        visible: true, // The window module is handling everything related to the window!
                        ..default()     // Use default settings for the rest of the Window struct.
                    }),
                    ..default() // Use default settings for the rest of the WindowPlugin.
                })
                // Configure the AssetPlugin to never perform asset metadata checks (improving performance).
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default() // Use default settings for the rest of the AssetPlugin.
                }),
        ))
        .add_plugins((
            // Just make sure the plugin isn't running
            // on wasm builds (trunk should ignore this
            // file but anyway)
            #[cfg(not(target_family = "wasm"))]
            window_manager::plugin,
            GamePlugin, // Add the GamePlugin, which contains the main game logic.
        ))
        // Start the application and begin running the Bevy app loop.
        .run();
}
