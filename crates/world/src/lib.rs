#![allow(clippy::type_complexity)]

use bevy::prelude::*;

pub mod levels;

pub fn plugin(app: &mut App) {
    app.add_plugins(levels::plugin);
}