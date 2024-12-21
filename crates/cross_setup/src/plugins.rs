#[cfg(any(feature = "desktop", feature = "mobile", target_family = "wasm"))]
use crate::std::StdPlugin;
#[cfg(feature = "terminal")]
use crate::terminal::TerminalRender;
#[cfg(feature = "terminal")]
use std::time::Duration;

use dep_reexp::bevy::asset::AssetMetaCheck;
#[cfg(feature = "terminal")]
use dep_reexp::bevy::app::ScheduleRunnerPlugin;
#[cfg(feature = "terminal")]
use dep_reexp::bevy::diagnostic::FrameTimeDiagnosticsPlugin;
#[cfg(feature = "terminal")]
use dep_reexp::bevy::log::LogPlugin;
use dep_reexp::bevy::prelude::*;
#[cfg(feature = "terminal")]
use dep_reexp::bevy::winit::WinitPlugin;
#[cfg(feature = "terminal")]
use dep_reexp::bevy_ratatui::RatatuiPlugins;
#[cfg(feature = "terminal")]
use dep_reexp::bevy_ratatui_render::RatatuiCameraPlugin;

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
            TerminalRender
        ));

        #[cfg(feature = "embedded")]
        app.add_plugins(());
    }
}
