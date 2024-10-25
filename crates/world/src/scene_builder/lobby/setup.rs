use bevy::prelude::*;

use crate::objects_queue::{ObjectData, QueuedObjects};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_level);
}

fn spawn_level(queued_objects: ResMut<QueuedObjects>) {
    queued_objects.value.enqueue(ObjectData {
        form: "cube".to_string(),
        variant: 1,
        size: 2.0,
        position: (0.0, 1.0, 0.0),
        name: "Cube".to_string(),
    });

    queued_objects.value.enqueue(ObjectData {
        form: "torus".to_string(),
        variant: 3,
        size: 10.0,
        position: (0.0, 0.0, 3.0),
        name: "Torus".to_string(),
    });
}
