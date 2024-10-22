#![allow(clippy::type_complexity)]

use bevy::prelude::*;

pub mod menu;

pub fn plugin(app: &mut App) {
    app.add_plugins(menu::plugin);
}
