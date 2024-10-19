use bevy::prelude::*;

use crate::screens::{loading::PlayerAssets, Screen};

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Playing), spawn_player);
}

#[derive(Component)]
pub struct Player {
    pub health: f64,
    pub min_speed: f64,
    pub max_speed: f64,
    pub jump_intensity: f64,
}

fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    let player = (
        SceneBundle {
            scene: player_assets.base_character.clone(),
            transform: Transform::from_xyz(0.0, 20.0, 0.0),
            ..default()
        },
        Player {
            health: 1.0,
            min_speed: 1.5,
            max_speed: 8.0,
            jump_intensity: 10.0,
        },
    );

    commands.spawn(player);
}
