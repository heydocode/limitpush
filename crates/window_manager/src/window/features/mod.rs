use bevy::prelude::*;

pub mod kill;

pub const KILL_KEY: KeyCode = KeyCode::Escape;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        kill::plugin, // Kills the window on desktop builds when the player taps a key
    ));
}
