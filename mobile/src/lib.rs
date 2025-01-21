use bevy::{
    app::{App, AppExit},
    prelude::bevy_main,
};
use limitpush::AssemblerPlugin; // BRANDING: Replace bevy_game with your new crate name.

#[bevy_main]
fn main() -> AppExit {
    App::new().add_plugins(AssemblerPlugin).run()
}
