#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use objects_queue::QueuedObjects;

pub mod objects_queue;
pub mod scene_builder;

pub fn plugin(app: &mut App) {
    app.add_plugins(scene_builder::plugin);
    app.init_resource::<QueuedObjects>();
}
