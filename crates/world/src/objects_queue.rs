use std::collections::VecDeque;

use bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct QueuedObjects {
    queue: VecDeque<ObjectData>, // Store object data in a queue
}

impl QueuedObjects {
    pub fn new() -> Self {
        Self {
            queue: VecDeque::new(),
        }
    }

    pub fn enqueue(&mut self, object: ObjectData) {
        self.queue.push_back(object);
    }

    pub fn dequeue(&mut self) -> Option<ObjectData> {
        self.queue.pop_front()
    }

    pub fn is_empty(&self) -> bool {
        self.queue.is_empty()
    }
}

// Struct to store object data (could be extended with more fields)
pub struct ObjectData {
    pub form: String,
    pub variant: u32,
    pub size: f32,
    pub position: (f32, f32, f32),
    pub name: String,
}
