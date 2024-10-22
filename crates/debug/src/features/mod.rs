//! This crate implements debug to help develop the limitpush game
//!
//! Wireframe isn't supported by web & mobile builds, well web is rejected
//! in lib.rs because it doesn't even support the debug menu

pub mod debug_menu;
// DON'T IMPORT WIREFRAME TO MOBILE BUILDS!
#[cfg(not(any(target_os = "android", target_os = "ios")))]
pub mod wireframe;

use bevy::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins((
        debug_menu::plugin,
        // DON'T IMPORT WIREFRAME TO MOBILE BUILDS!
        #[cfg(not(any(target_os = "android", target_os = "ios")))]
        wireframe::plugin,
    ));
}
