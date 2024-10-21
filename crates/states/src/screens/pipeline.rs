use bevy::prelude::*;
use bevy_pipelines_ready::{PipelinesReady, PipelinesReadyPlugin};

use super::{cleanup, Screen};

#[derive(Component)]
struct PipelineOnly;

const EXPECTED_PIPELINES: usize = 9;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PipelinesReadyPlugin);
    app.add_systems(OnEnter(Screen::Pipeline), setup_loading_screen);
    app.add_systems(Update, print.run_if(resource_changed::<PipelinesReady>));
    app.add_systems(
        Update,
        transition
            .run_if(in_state(Screen::Pipeline))
            .run_if(resource_changed::<PipelinesReady>),
    );
    app.add_systems(OnExit(Screen::Pipeline), cleanup::<PipelineOnly>);
}

// Your loading screen should include all of the cameras, lights, and other elements that cause
// pipelines to be built in your app.
fn setup_loading_screen(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section("Pipelines loading...".to_string(), TextStyle::default()),
        PipelineOnly,
    ));
}

fn print(ready: Res<PipelinesReady>) {
    info!("Pipelines Ready: {}/{}", ready.get(), EXPECTED_PIPELINES);
}

fn transition(ready: Res<PipelinesReady>, mut next_state: ResMut<NextState<Screen>>) {
    if ready.get() >= EXPECTED_PIPELINES {
        next_state.set(Screen::Menu);
    }
}
