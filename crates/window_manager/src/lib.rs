#![allow(clippy::type_complexity)]

pub mod window;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(window::plugin);
}
