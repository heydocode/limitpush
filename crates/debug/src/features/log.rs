use bevy::{
    app::App,
    diagnostic::{FrameTimeDiagnosticsPlugin, LogDiagnosticsPlugin},
};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins((
        FrameTimeDiagnosticsPlugin, // Tracks frame time for performance tuning.
        LogDiagnosticsPlugin::default(), // Logs performance data to the console.
    ));
}
