#[cfg(any(feature = "desktop", feature = "mobile", target_family = "wasm"))]
use crate::std::StdPlugin;
#[cfg(feature = "terminal")]
use crate::terminal::TerminalRender;
#[cfg(feature = "terminal")]
use std::time::Duration;

#[cfg(feature = "terminal")]
use bevy::app::ScheduleRunnerPlugin;
#[cfg(not(feature = "embedded"))]
#[allow(unused_imports)]
// This import is ALWAYS used, but rust-analyzer is unable to understand that
use bevy::asset::AssetMetaCheck;
#[cfg(feature = "terminal")]
use bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(feature = "terminal")]
use bevy::log::LogPlugin;
use bevy::prelude::*;
#[cfg(feature = "desktop")]
use bevy::render::settings::{RenderCreation, WgpuFeatures, WgpuSettings};
#[cfg(feature = "desktop")]
use bevy::render::RenderPlugin;
#[cfg(feature = "terminal")]
use bevy::winit::WinitPlugin;
#[cfg(feature = "terminal")]
use bevy_ratatui::RatatuiPlugins;
#[cfg(feature = "terminal")]
use bevy_ratatui_camera::RatatuiCameraPlugin;

pub struct PluginsSetup;

impl Plugin for PluginsSetup {
    #[allow(unused_variables)]
    fn build(&self, app: &mut App) {
        #[cfg(feature = "desktop")]
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .set(RenderPlugin {
                    render_creation: RenderCreation::Automatic(WgpuSettings {
                        features: WgpuFeatures::POLYGON_MODE_LINE,
                        ..default()
                    }),
                    ..default()
                }),
            StdPlugin,
        ));

        #[cfg(feature = "mobile")]
        app.add_plugins((
            DefaultPlugins.set(AssetPlugin {
                meta_check: AssetMetaCheck::Never,
                ..default()
            }),
            StdPlugin,
        ));

        #[cfg(target_family = "wasm")]
        app.add_plugins((
            DefaultPlugins
                .set(WindowPlugin {
                    primary_window: Some(Window {
                        title: "LimitPush".to_string(),
                        canvas: Some("#bevy".to_owned()),
                        fit_canvas_to_parent: true,
                        prevent_default_event_handling: false,
                        visible: true,
                        ..default()
                    }),
                    ..default()
                })
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                }),
            StdPlugin,
        ));

        #[cfg(feature = "terminal")]
        app.add_plugins((
            DefaultPlugins
                .set(AssetPlugin {
                    meta_check: AssetMetaCheck::Never,
                    ..default()
                })
                .build()
                .disable::<WinitPlugin>()
                .disable::<LogPlugin>(),
            ScheduleRunnerPlugin::run_loop(Duration::from_secs_f64(1. / 90.)),
            FrameTimeDiagnosticsPlugin,
            RatatuiPlugins::default(),
            RatatuiCameraPlugin,
            TerminalRender,
        ));

        #[cfg(feature = "embedded")]
        app.add_plugins(());
    }
}
