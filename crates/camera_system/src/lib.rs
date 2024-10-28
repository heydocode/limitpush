pub mod camera;

use bevy::app::App;
use bevy::color::Color;
use bevy::prelude::ClearColor;
use bevy::prelude::Msaa;

pub fn plugin(app: &mut App) {
    app.add_plugins(camera::plugin);

    app.insert_resource(Msaa::Off);

    app.insert_resource(ClearColor(Color::linear_rgb(0.4, 0.4, 0.4)));
}
