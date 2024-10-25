use avian3d::math::{Scalar, Vector2};
use bevy::prelude::*;
use states::screens::{loading::VirtualJoystickAssets, Screen};
use virtual_joystick::*;

use crate::movement::MovementAction;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(VirtualJoystickPlugin::<String>::default());
    app.add_systems(OnEnter(Screen::Pipeline), create_scene);
    app.add_systems(Update, update_joystick.run_if(in_state(Screen::Playing)));
}

fn create_scene(mut commands: Commands, virtual_joystick_assets: Res<VirtualJoystickAssets>) {
    let joystick_entity_instance = create_joystick(
        &mut commands,
        "PlayerMovementJoystick".to_string(),
        virtual_joystick_assets.knob.clone(),
        virtual_joystick_assets.outline.clone(),
        None,
        None,
        None,
        Vec2::new(75., 75.),
        Vec2::new(150., 150.),
        Style {
            width: Val::Px(150.),
            height: Val::Px(150.),
            position_type: PositionType::Absolute,
            left: Val::Percent(50.),
            bottom: Val::Percent(15.),
            ..default()
        },
        (JoystickFloating, JoystickInvisible),
        NoAction,
    );

    commands.spawn(joystick_entity_instance);
}

/// Sends [`MovementAction`] events based on touch input (virtual joystick).
fn update_joystick(
    mut joystick: EventReader<VirtualJoystickEvent<String>>,
    mut movement_event_writer: EventWriter<MovementAction>,
) {
    for j in joystick.read() {
        let Vec2 { x, y } = j.axis();
        movement_event_writer.send(MovementAction::Move(
            Vector2::new(*x as Scalar, *y as Scalar).clamp_length_max(1.0),
        ));

        // TODO! Implement jump with a dedicated button
    }
}