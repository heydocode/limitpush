use bevy::{input::common_conditions::input_just_pressed, prelude::*};

use super::Screen;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(
        Update,
        return_to_menu
            .run_if(input_just_pressed(KeyCode::Escape).and_then(in_state(Screen::Playing))),
    );
}

fn return_to_menu(mut next_state: ResMut<NextState<Screen>>) {
    next_state.set(Screen::Menu);
}
