use super::Screen;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;

use super::cleanup;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnExit(Screen::Loading), cleanup::<LoadingOnly>);
    app.add_systems(OnEnter(Screen::Loading), setup_loading_screen);

    // Linking Screen::Loading with the loading assets logic
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Pipeline)
            .load_collection::<AudioAssets>()
            .load_collection::<TextureAssets>()
            // .load_collection::<PlayerAssets>()
            .load_collection::<VirtualJoystickAssets>(),
    );
}

// the following asset collections will be loaded during the State `Screen::Loading`
// when done loading, they will be inserted as resources (see <https://github.com/NiklasEi/bevy_asset_loader>)

#[derive(AssetCollection, Resource)]
pub struct AudioAssets {
    #[asset(path = "audio/flying.ogg")]
    pub flying: Handle<AudioSource>,
}

#[derive(AssetCollection, Resource)]
pub struct TextureAssets {
    #[asset(path = "textures/bevy.png")]
    pub bevy: Handle<Image>,
    #[asset(path = "textures/github.png")]
    pub github: Handle<Image>,
}

// #[derive(AssetCollection, Resource)]
// pub struct PlayerAssets {
//     #[asset(path = "models/animated/player/BaseCharacter.gltf#Scene0")]
//     pub base_character: Handle<Scene>,
//     #[asset(path = "models/animated/player/Cow.gltf#Scene0")]
//     pub cow: Handle<Scene>,
// }

#[derive(AssetCollection, Resource)]
pub struct VirtualJoystickAssets {
    #[asset(path = "textures/Knob.png")]
    pub knob: Handle<Image>,
    #[asset(path = "textures/Outline.png")]
    pub outline: Handle<Image>,
}

// ---------------------------------------------------------------------------------------------------------------------------------
// ------------- LOADING SCREEN ------------- LOADING SCREEN ------------- LOADING SCREEN ------------- LOADING SCREEN -------------
// ---------------------------------------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct LoadingOnly;

// Your loading screen.
fn setup_loading_screen(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Pipelines loading...".to_string(),
            TextStyle {
                font_size: 30.0,
                color: Color::srgb(0.0, 0.0, 0.0),
                ..default()
            },
        ),
        LoadingOnly,
    ));
}
