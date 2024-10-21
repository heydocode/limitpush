use bevy::{core::FrameCount, prelude::*};

mod features;

#[derive(States, Default, Clone, Eq, PartialEq, Debug, Hash)]
enum WindowState {
    #[default]
    ShowingTask,
    Shown,
}

pub(super) fn plugin(app: &mut App) {
    // Initializes the `Screen` state. This state tracks which game screen is currently active.
    app.init_state::<WindowState>();

    app.add_systems(
        Update,
        make_visible.run_if(in_state(WindowState::ShowingTask)),
    );

    app.add_plugins((features::plugin,));
}

fn make_visible(
    mut window: Query<&mut Window>,
    frames: Res<FrameCount>,
    mut next_state: ResMut<NextState<WindowState>>,
    state: Res<State<WindowState>>,
) {
    if window.single_mut().visible {
        eprintln!(
            "The window is already shown! WindowState: {:?}",
            state.get()
        )
    }

    // The delay may be different for your app or system.
    if frames.0 > 5 {
        // At this point, the GPU is ready to show the app so we can make the window visible.
        window.single_mut().visible = true;

        // Transition to the "Done" state, which will stop the system from running
        next_state.set(WindowState::Shown);
    }
}
