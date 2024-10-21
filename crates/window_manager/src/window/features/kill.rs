use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::KILL_KEY;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        kill_window_on_escape.run_if(
            input_just_pressed(KILL_KEY).and_then(input_just_pressed(KeyCode::ControlLeft)),
        ),
    );
}

pub fn kill_window_on_escape(mut app_exit_events: ResMut<Events<bevy::app::AppExit>>) {
    app_exit_events.send(AppExit::Success);
}
