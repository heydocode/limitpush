use crate::bevy_ecs;
use dep_reexp::bevy::prelude::Resource;

#[derive(Resource, Default)]
pub struct Flags {
    pub debug: bool,
}
