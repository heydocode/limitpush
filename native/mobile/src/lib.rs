use bevy::asset::AssetMetaCheck;
use bevy::prelude::*;
use bevy::window::WindowMode;
use limitpush::GamePlugin; // ToDo: Replace bevy_game with your new crate name.

#[bevy_main]
fn main() {
    App::new()
        .add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        resizable: false,
                        mode: WindowMode::BorderlessFullscreen,
                        ..default()
                    }),
                    ..default()
                })
                // Configure the AssetPlugin to never perform asset metadata checks (improving performance).
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default() // Use default settings for the rest of the AssetPlugin.
                }),
            GamePlugin,
        ))
        .run();
}
