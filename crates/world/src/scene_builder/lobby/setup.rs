use bevy::prelude::*;

use crate::objects_queue::{ObjectData, QueuedObjects};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Startup, spawn_level);
}

fn spawn_level(mut queued_objects: ResMut<QueuedObjects>) {
    queued_objects.enqueue(ObjectData {
        form: "cube".to_string(),
        variant: 1,
        size: 2.0,
        position: (0.0, 5.0, 0.0),
        name: "Cube".to_string(),
    });

    queued_objects.enqueue(ObjectData {
        form: "torus".to_string(),
        variant: 3,
        size: 10.0,
        position: (0.0, 10.0, 3.0),
        name: "Torus".to_string(),
    });

    queued_objects.enqueue(ObjectData {
        form: "plane".to_string(),
        variant: 2,
        size: 1000.0,
        position: (0.0, 0.0, 3.0),
        name: "Plane3D".to_string(),
    });
}
