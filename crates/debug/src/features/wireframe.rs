use bevy::{pbr::wireframe::WireframePlugin, prelude::*};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(WireframePlugin);
}