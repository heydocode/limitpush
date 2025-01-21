use bevy::app::AppExit;
use bevy::diagnostic::{DiagnosticsStore, FrameTimeDiagnosticsPlugin};
use bevy::prelude::*;
use bevy::utils::error;
use bevy_ratatui::event::KeyEvent;
use bevy_ratatui::kitty::KittyEnabled;
use bevy_ratatui::terminal::RatatuiContext;
use bevy_ratatui_camera::LuminanceConfig;
use bevy_ratatui_camera::RatatuiCamera;
use bevy_ratatui_camera::RatatuiCameraEdgeDetection;
use bevy_ratatui_camera::RatatuiCameraStrategy;
use bevy_ratatui_camera::RatatuiCameraWidget;
use crossterm::event::{KeyCode, KeyEventKind};
use log::LevelFilter;
use ratatui::layout::Alignment;
use ratatui::layout::Constraint;
use ratatui::layout::Direction;
use ratatui::layout::Layout;
use ratatui::style::Style;
use ratatui::style::Stylize;
use ratatui::widgets::Block;
use std::io;
use tui_logger::init_logger;
use tui_logger::set_default_level;
use tui_logger::TuiLoggerWidget;

pub struct TerminalRender;

impl Plugin for TerminalRender {
    fn build(&self, app: &mut App) {
        init_logger(LevelFilter::Info).unwrap();
        set_default_level(LevelFilter::Info);

        app.add_systems(Startup, camera_setup);
        app.add_systems(Update, draw_scene_system.map(error));
        app.add_systems(PreUpdate, handle_input_system);
        app.insert_resource(Flags { debug: false });
    }
}

#[derive(Component)]
pub struct Camera;

#[derive(Resource, Default)]
pub struct Flags {
    pub debug: bool,
}

fn camera_setup(mut commands: Commands) {
    commands.spawn((
        RatatuiCamera::autoresize(),
        RatatuiCameraStrategy::Luminance(LuminanceConfig {
            luminance_scale: 7.0,
            ..default()
        }),
        RatatuiCameraEdgeDetection {
            edge_color: Some(ratatui::style::Color::Black),
            thickness: 1.0,
            normal_enabled: false,
            color_enabled: false,
            ..default()
        },
        Camera,
        Camera3d::default(),
        Transform::from_xyz(2.5, 2.5, 2.5).looking_at(Vec3::ZERO, Vec3::Z),
    ));
}

fn draw_scene_system(
    mut ratatui: ResMut<RatatuiContext>,
    ratatui_camera_widget: Query<&RatatuiCameraWidget>,
    flags: Res<Flags>,
    diagnostics: Res<DiagnosticsStore>,
    kitty_enabled: Option<Res<KittyEnabled>>,
) -> io::Result<()> {
    ratatui.draw(|frame| {
        let layout = Layout::new(
            Direction::Vertical,
            [Constraint::Percentage(66), Constraint::Fill(1)],
        )
        .split(frame.area());

        let mut block = Block::bordered()
            .bg(ratatui::style::Color::Rgb(0, 0, 0))
            .border_style(Style::default().bg(ratatui::style::Color::Rgb(0, 0, 0)))
            .title_bottom("[q for quit]")
            .title_bottom("[d for debug]")
            .title_bottom("[p for panic]")
            .title_alignment(Alignment::Center);

        if flags.debug {
            block = block.title_top(format!(
                "[kitty protocol: {}]",
                if kitty_enabled.is_some() {
                    "enabled"
                } else {
                    "disabled"
                }
            ));

            if let Some(value) = diagnostics
                .get(&FrameTimeDiagnosticsPlugin::FPS)
                .and_then(|fps| fps.smoothed())
            {
                block = block.title_top(format!("[fps: {value:.0}]"));
            }

            let inner = block.inner(layout[0]);
            frame.render_widget(block, layout[0]);
            frame.render_widget(
                TuiLoggerWidget::default()
                    .block(Block::bordered())
                    .style(Style::default().bg(ratatui::style::Color::Reset)),
                layout[1],
            );
            if let Ok(camera_widget) = ratatui_camera_widget.get_single() {
                frame.render_widget(camera_widget, inner);
            }
        } else {
            let inner = block.inner(frame.area());
            frame.render_widget(block, frame.area());
            if let Ok(camera_widget) = ratatui_camera_widget.get_single() {
                frame.render_widget(camera_widget, inner);
            }
        }
    })?;

    Ok(())
}

pub fn handle_input_system(
    mut rat_events: EventReader<KeyEvent>,
    mut exit: EventWriter<AppExit>,
    mut flags: ResMut<Flags>,
) {
    for key_event in rat_events.read() {
        match key_event.kind {
            KeyEventKind::Press | KeyEventKind::Repeat => match key_event.code {
                KeyCode::Char('q') => {
                    exit.send_default();
                }
                KeyCode::Char('p') => {
                    panic!("Panic!");
                }
                KeyCode::Char('d') => {
                    flags.debug = !flags.debug;
                }
                _ => {}
            },
            _ => {}
        }
    }
}
