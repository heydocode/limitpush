use crate::Screen;
use bevy::color::palettes::css::*;
use bevy::prelude::*;
use bevy_asset_loader::prelude::*;
use bevy_kira_audio::AudioSource;
use bevy_vector_shapes::prelude::*;
use std::f32::consts::TAU;

use super::cleanup;

/// This plugin loads all assets using [`AssetLoader`] from a third party bevy plugin
/// Alternatively you can write the logic to load assets yourself
/// If interested, take a look at <https://bevy-cheatbook.github.io/features/assets.html>
pub(super) fn plugin(app: &mut App) {
    app.add_plugins(ShapePlugin::default());
    app.add_systems(OnExit(Screen::Loading), cleanup::<LoadingOnly>);
    app.add_systems(OnEnter(Screen::Loading), setup_loading_screen);
    app.add_systems(
        Update,
        draw_loading_screen.run_if(in_state(Screen::Loading)),
    );

    // Linking Screen::Loading with the loading assets logic
    app.add_loading_state(
        LoadingState::new(Screen::Loading)
            .continue_to_state(Screen::Pipeline)
            .load_collection::<AudioAssets>()
            .load_collection::<TextureAssets>()
            .load_collection::<PlayerAssets>(),
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

#[derive(AssetCollection, Resource)]
pub struct PlayerAssets {
    #[asset(path = "models/animated/player/BaseCharacter.gltf#Scene0")]
    pub base_character: Handle<Scene>,
    #[asset(path = "models/animated/player/Cow.gltf#Scene0")]
    pub cow: Handle<Scene>,
}

// ---------------------------------------------------------------------------------------------------------------------------------
// ------------- LOADING SCREEN ------------- LOADING SCREEN ------------- LOADING SCREEN ------------- LOADING SCREEN -------------
// ---------------------------------------------------------------------------------------------------------------------------------

#[derive(Component)]
pub struct LoadingOnly;

pub trait Pastel {
    fn pastel(&self) -> Srgba;
}

impl Pastel for Srgba {
    fn pastel(&self) -> Srgba {
        let mut out = *self + Srgba::WHITE * 0.25;
        out.set_alpha(1.0);
        out
    }
}

/// This function draws the animated loading screen,
/// to avoid providing a lag experience.
fn draw_loading_screen(time: Res<Time>, painter: ShapePainter) {
    gallery(painter, time.elapsed_seconds());
}

/// This function contains the loading screen UI components and animates them.
pub fn gallery(mut painter: ShapePainter, seconds: f32) {
    let seconds = seconds % (2.0 * TAU);
    let start_pos = painter.transform;
    let (x, y) = (0.0, 0.0);
    let _diag_vec = Vec3::X + Vec3::Y;
    painter.transform = start_pos;
    painter.translate((Vec3::X * x - Vec3::Y * y) * 4.0);

    let circle_fill = ((seconds * 2.).sin() + 1.0) / 2.0;

    painter.hollow = true;
    painter.set_color(ORANGE.pastel());
    painter.thickness = 0.5;
    painter.circle(1.5 * circle_fill);
}

fn setup_loading_screen(
    mut commands: Commands,
) {
    commands.spawn((
        TextBundle::from_section("Assets loading...".to_string(), TextStyle::default()),
        LoadingOnly,
    ));
}