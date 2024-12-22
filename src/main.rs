// This line disables the console window on Windows when compiling in release mode.
// It helps provide a cleaner experience for applications that do not require console output.
// The `cfg_attr` attribute conditionally applies the `windows_subsystem` attribute
// if the build is not in debug mode (i.e., for release builds).
#![cfg_attr(not(debug_assertions), windows_subsystem = "windows")]

use dep_reexp::bevy::app::{App, AppExit};
use limitpush::GamePlugin;

fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}