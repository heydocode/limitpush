use bevy::{
    app::{App, AppExit},
    prelude::bevy_main,
};
use limitpush::GamePlugin; // ToDo: Replace bevy_game with your new crate name.

#[bevy_main]
fn main() -> AppExit {
    App::new().add_plugins(GamePlugin).run()
}
