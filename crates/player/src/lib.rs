#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use movement::CharacterControllerPlugin;

#[cfg(any(target_os = "ios", target_os = "android", target_arch = "wasm32"))]
pub mod mobile_controller;
pub mod movement;
pub mod spawn;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        CharacterControllerPlugin,
        spawn::plugin,
        #[cfg(any(target_os = "ios", target_os = "android", target_arch = "wasm32"))]
        mobile_controller::plugin,
    ));
    app.register_type::<Player>();
}

#[derive(Component, Reflect)]
#[reflect(Component)] // Ensure Player can be reflected as a component
pub struct Player {
    pub health: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub jump_intensity: f32,
}

pub const MINIMAL_PLAYER_Y: f32 = -15.0;
