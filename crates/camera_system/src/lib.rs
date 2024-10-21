pub mod camera;
pub mod pan_orbit_camera;

use bevy::app::App;
use bevy::color::Color;
use bevy::prelude::ClearColor;
use bevy::prelude::Msaa;

pub fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin);

    // Disable anti-aliasing (multisampling).
    app.insert_resource(Msaa::Off);

    // Set the window background color (grey in this case).
    app.insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)));
}
