use bevy::prelude::*;

use crate::audio::start_audio;

pub mod loading;
pub mod menu;
pub mod playing;
pub mod pipeline;

pub(super) fn plugin(app: &mut App) {
    // Initializes the `Screen` state. This state tracks which game screen is currently active.
    app.init_state::<Screen>();

    // For the best readability, please order the plugins in the Screen order
    // (in the call order, so: ...Loading->Menu->Playing...)
    app.add_plugins((loading::plugin, menu::plugin, playing::plugin, pipeline::plugin));
    app.add_systems(OnEnter(Screen::Playing), start_audio);
}

// This example game uses States to separate logic
// See https://bevy-cheatbook.github.io/programming/states.html
// Or https://github.com/bevyengine/bevy/blob/main/examples/ecs/state.rs
#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
pub enum Screen {
    // TODO! Please implement the lightweight splash screen using bevy_lunex
    // and then implement UI and make this Screen default
    // #[default]
    // Splash

    // During the loading State the LoadingPlugin will load our assets
    #[default]
    Loading,
    Pipeline,
    // Here the menu is drawn and waiting for player interaction
    Menu,
    // During this State the actual game logic is executed
    Playing,
}

pub fn cleanup<T: Component>(mut commands: Commands, query: Query<Entity, With<T>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn_recursive();
    }
}

/// These objects should despawn on exit of loading state
#[derive(Component)]
pub struct LoadingObject;

/// All objects with this component should be invisible until switching
/// screen from `Loading` to `Playing`
#[derive(Component)]
pub struct GameObject;