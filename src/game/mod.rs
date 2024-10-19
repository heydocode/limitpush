use bevy::prelude::*;

pub mod camera;
pub mod player;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((camera::plugin, player::plugin));
}
