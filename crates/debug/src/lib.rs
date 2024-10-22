pub mod features;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(features::plugin);
}
