use bevy::prelude::*;

pub mod setup;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(setup::plugin);
}
