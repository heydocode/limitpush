// This line disables the console window on Windows when compiling in release mode.
// It helps provide a cleaner experience for applications that do not require console output.
// The `cfg_attr` attribute conditionally applies the `windows_subsystem` attribute
// if the build is not in debug mode (i.e., for release builds).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use bevy::asset::AssetMetaCheck;
// Importing asset management checks from Bevy.
use bevy::prelude::*;
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
use bevy::render::RenderPlugin;
// Importing common Bevy types and traits for ease of use.
use bevy::window::PrimaryWindow; // Importing the PrimaryWindow component for window management.
use bevy::winit::WinitWindows; // Winit is a window management library that Bevy uses for window handling.
use bevy::DefaultPlugins; // Default plugins provided by Bevy for standard functionalities.
use limitpush::GamePlugin; // Importing the GamePlugin, which contains core game logic.
use std::io::Cursor; // Used to create an in-memory cursor for the image data.
use winit::window::Icon; // Importing the Icon type for window icon management.

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
                        visible: false, // The window module is handling everything related to the window!
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
        // Register the system to set the window icon on startup.
        .add_systems(Startup, set_window_icon)
        // Start the application and begin running the Bevy app loop.
        .run();
}

// This function sets the window icon for Windows and X11 (Linux).
// TODO: Investigate potential conflicts with WASM and ensure compatibility with Trunk.
// It's important to determine if Trunk is using this main.rs file for WASM builds.
fn set_window_icon(
    windows: NonSend<WinitWindows>, // NonSend allows this resource to be used without sending it between threads.
    primary_window: Query<Entity, With<PrimaryWindow>>, // Query to get the entity representing the primary window.
) {
    // Get the single primary window entity from the query.
    let primary_entity = primary_window.single();
    // Attempt to retrieve the window associated with the primary entity.
    let Some(primary) = windows.get_window(primary_entity) else {
        return; // If the window is not found, exit the function early.
    };

    // Load the window icon image from the specified path in the project.
    let icon_buf = Cursor::new(include_bytes!(
        "../build/macos/AppIcon.iconset/icon_256x256.png" // The icon image included in the binary.
    ));

    // Try to load the image in PNG format.
    if let Ok(image) = image::load(icon_buf, image::ImageFormat::Png) {
        let image = image.into_rgba8(); // Convert the image to RGBA format (8 bits per channel).
        let (width, height) = image.dimensions(); // Get the image dimensions.
        let rgba = image.into_raw(); // Convert the image into a raw RGBA byte array.
        let icon = Icon::from_rgba(rgba, width, height).unwrap(); // Create an Icon from the RGBA data.

        // Set the window icon using the created Icon instance.
        primary.set_window_icon(Some(icon));
    }
}
