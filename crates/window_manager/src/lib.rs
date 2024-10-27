#![allow(clippy::type_complexity)]

pub mod window;

use bevy::prelude::*;
use std::io::Cursor; // Used to create an in-memory cursor for the image data.
use winit::window::Icon; // Importing the Icon type for window icon management.
use bevy::window::PrimaryWindow; // Importing the PrimaryWindow component for window management.
use bevy::winit::WinitWindows; // Winit is a window management library that Bevy uses for window handling.

pub fn plugin(app: &mut App) {
    app.add_plugins(window::plugin);
    // Register the system to set the window icon on startup.
    app.add_systems(Startup, set_window_icon);
}

// This function sets the window icon for Windows and X11 (Linux).
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
        "../../../build/macos/AppIcon.iconset/icon_256x256.png" // The icon image included in the binary.
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
