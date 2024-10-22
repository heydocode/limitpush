use bevy::prelude::*;
use bevy_rapier3d::prelude::*;
use states::screens::Screen;

use crate::Player;

pub fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        (player_movement_system, read_movement_output_system).run_if(in_state(Screen::Playing)),
    );
}

fn player_movement_system(
    input: Res<ButtonInput<KeyCode>>,
    time: Res<Time>,
    mut controllers: Query<&mut KinematicCharacterController>,
    player: Query<&Player, With<Player>>,
) {
    let player = player.single(); // Get &Player from player parameter query and put it on player variable

    let speed = if input.pressed(KeyCode::ShiftLeft) {
        player.max_speed // Sprint
    } else if input.pressed(KeyCode::ControlLeft) {
        player.min_speed * 0.5 // Crouch speed
    } else {
        player.min_speed // Walk
    };

    for mut controller in controllers.iter_mut() {
        let mut translation = Vec3::ZERO;

        if input.pressed(KeyCode::KeyW) {
            translation.z += 1.0;
        }
        if input.pressed(KeyCode::KeyS) {
            translation.z -= 1.0;
        }
        if input.pressed(KeyCode::KeyA) {
            translation.x -= 1.0;
        }
        if input.pressed(KeyCode::KeyD) {
            translation.x += 1.0;
        }

        // Normalize direction and apply speed
        if translation.length_squared() > 0.0 {
            translation = translation.normalize() * speed * time.delta_seconds();
        }

        // Apply movement to the controller
        controller.translation = Some(translation);
    }
}

fn read_movement_output_system(outputs: Query<(Entity, &KinematicCharacterControllerOutput)>) {
    for (entity, output) in outputs.iter() {
        println!(
            "Entity {:?} moved by {:?} and is grounded: {:?}",
            entity, output.effective_translation, output.grounded
        );
    }
}
