//! This crate implements debug to help develop the limitpush game
//!
//! Wireframe isn't supported by web & mobile builds, that' why
//! we have established conditional compilation to avoid crashes
//! (and infinite lag in wasm when try to render vertices in Wireframe mode)

pub mod debug_menu;
// DON'T IMPORT WIREFRAME TO MOBILE BUILDS!
#[cfg(not(any(target_os = "android", target_os = "ios", target_family = "wasm")))]
pub mod wireframe;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        debug_menu::plugin,
        // DON'T IMPORT WIREFRAME TO MOBILE BUILDS!
        #[cfg(not(any(target_os = "android", target_os = "ios", target_family = "wasm")))]
        wireframe::plugin,
    ));
}
