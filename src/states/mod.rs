use bevy::prelude::*;

pub mod screens;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((screens::plugin,));
}
