use bevy::{prelude::*, window::PrimaryWindow};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{bevy_inspector, DefaultInspectorConfigPlugin};

pub(super) fn plugin(app: &mut App) {
    app.add_plugins(EguiPlugin);
    app.add_plugins(DefaultInspectorConfigPlugin);
    app.add_systems(Update, inspector_ui);
}



fn inspector_ui(world: &mut World, mut disabled: Local<bool>) {
    let key_c_pressed = world
        .resource::<ButtonInput<KeyCode>>()
        .just_pressed(KeyCode::KeyC);
    if key_c_pressed {
        *disabled = !*disabled;
    }
    if *disabled {
        return;
    }

    // Mobile platforms (Android, iOS)
    #[cfg(any(target_os = "android", target_os = "ios"))]
    let window_size: (f32, f32) = (160.0, 80.0);

    // Desktop platforms (Windows, macOS, Linux)
    #[cfg(any(target_os = "windows", target_os = "macos", target_os = "linux"))]
    let window_size: (f32, f32) = (320.0, 160.0);

    // All other platforms (not mobile or desktop)
    #[cfg(not(any(target_os = "android", target_os = "ios", target_os = "windows", target_os = "macos", target_os = "linux")))]
    let window_size: (f32, f32) = (100.0, 100.0);

    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("LimitPush DEBUG")
        .default_size(window_size)
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_world(world, ui);
                ui.allocate_space(ui.available_size());
            });
        });
}
