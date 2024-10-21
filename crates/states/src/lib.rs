#![allow(clippy::type_complexity)]

pub mod screens;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(screens::plugin);
}
