pub mod components;
pub mod resources;
pub mod states;

// I don't know why, but it fixes an error
pub(crate) use dep_reexp::bevy::ecs as bevy_ecs;
