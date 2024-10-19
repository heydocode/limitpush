use bevy::prelude::*;

pub mod levels;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((levels::plugin,));
}
