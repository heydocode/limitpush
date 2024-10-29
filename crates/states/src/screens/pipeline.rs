use bevy::prelude::*;
use bevy_pipelines_ready::{PipelinesReady, PipelinesReadyPlugin};

use super::{cleanup, Screen};

#[derive(Component)]
struct PipelineOnly;

const EXPECTED_PIPELINES: usize = 5;

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(PipelinesReadyPlugin);
    app.add_systems(OnEnter(Screen::Pipeline), setup_pipeline_screen);
    app.add_systems(Update, print.run_if(resource_changed::<PipelinesReady>));
    app.add_systems(
        Update,
        transition
            .run_if(in_state(Screen::Pipeline))
            .run_if(resource_changed::<PipelinesReady>),
    );
    app.add_systems(OnExit(Screen::Pipeline), cleanup::<PipelineOnly>);
}

// Your pipeline screen should include all of the cameras, lights, and other elements that cause
// pipelines to be built in your app.
fn setup_pipeline_screen(mut commands: Commands) {
    commands.spawn((
        TextBundle::from_section(
            "Pipelines loading...".to_string(),
            TextStyle {
                font_size: 30.0,
                color: Color::srgb(0.0, 0.0, 0.0),
                ..default()
            },
        ),
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
