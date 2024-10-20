use bevy::prelude::*;

#[cfg(all(feature = "dev", not(target_family = "wasm")))]
pub mod debug;
pub mod openworld;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        openworld::plugin,
        #[cfg(all(feature = "dev", not(target_family = "wasm")))]
        debug::plugin,
    ));
}
