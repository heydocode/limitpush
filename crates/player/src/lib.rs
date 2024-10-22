#![allow(clippy::type_complexity)]

use bevy::prelude::*;
use bevy_rapier3d::prelude::*; // Import Rapier3D components

pub mod movement;

use states::screens::{loading::PlayerAssets, Screen};

pub fn plugin(app: &mut App) {
    app.add_systems(OnEnter(Screen::Pipeline), spawn_player);
    app.register_type::<Player>();
}

#[derive(Component, Reflect)]
#[reflect(Component)] // Ensure Player can be reflected as a component
pub struct Player {
    pub health: f32,
    pub min_speed: f32,
    pub max_speed: f32,
    pub jump_intensity: f32,
}

fn spawn_player(mut commands: Commands, player_assets: Res<PlayerAssets>) {
    let player = (
        SceneBundle {
            scene: player_assets.base_character.clone(),
            transform: Transform::from_xyz(0.0, 0.5, 0.0), // Start slightly above ground
            ..default()
        },
        Name::new("Player"),
        Player {
            health: 1.0,
            min_speed: 1.5,
            max_speed: 8.0,
            jump_intensity: 10.0,
        },
        RigidBody::KinematicPositionBased, // Set player movement to be kinematic
        Collider::capsule_y(1.0, 0.6),     // Add a capsule collider for the player shape
        KinematicCharacterController {
            offset: CharacterLength::Absolute(0.05), // Offset to avoid ground clipping
            autostep: Some(CharacterAutostep {
                max_height: CharacterLength::Absolute(0.5), // Maximum height the player can step over
                min_width: CharacterLength::Absolute(0.2),  // Minimum width required to step
                include_dynamic_bodies: true,               // Ability to step over dynamic objects
            }),
            ..default()
        },
    );

    commands.spawn(player);
}
