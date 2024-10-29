//! This crate implements debug to help develop the limitpush game
//!
//! Wireframe isn't supported by web & mobile builds, that' why
//! we have established conditional compilation to avoid crashes
//! (and infinite lag in wasm when try to render vertices in Wireframe mode)

#[cfg(not(target_family = "wasm"))]
pub mod debug_menu;

#[cfg(not(target_family = "wasm"))]
pub mod adapter_debug;

#[cfg(not(any(target_os = "ios", target_os = "android", target_family = "wasm")))]
pub mod panic_catcher;

#[cfg(not(any(target_os = "ios", target_os = "android", target_family = "wasm")))]
pub mod avian_debug;

#[cfg(feature = "diagnostics-logs")]
mod log;

#[cfg(not(any(target_os = "ios", target_os = "android", target_family = "wasm")))]
pub mod wireframe;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        #[cfg(not(target_family = "wasm"))]
        debug_menu::plugin,
        #[cfg(not(any(target_os = "android", target_os = "ios", target_family = "wasm")))]
        wireframe::plugin,
        #[cfg(not(any(target_os = "ios", target_os = "android", target_arch = "wasm32")))]
        avian_debug::plugin,
        #[cfg(not(any(target_os = "ios", target_os = "android", target_arch = "wasm32")))]
        panic_catcher::plugin,
        #[cfg(feature = "diagnostics-logs")]
        log::plugin,
        #[cfg(not(target_family = "wasm"))]
        adapter_debug::plugin,
    ));
}
