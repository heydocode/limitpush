use bevy::prelude::*;

pub mod openworld;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((openworld::plugin,));
}
