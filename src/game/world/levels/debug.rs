use bevy::{
    pbr::wireframe::{Wireframe, WireframePlugin},
    prelude::*,
    window::PrimaryWindow,
};
use bevy_egui::{egui, EguiContext, EguiPlugin};
use bevy_inspector_egui::{bevy_inspector, DefaultInspectorConfigPlugin};

use super::openworld::Terrain;

pub(super) fn plugin(app: &mut App) {
    #[cfg(not(target_os = "android"))]
    app.add_systems(
        Update,
        toggle_wireframe.run_if(resource_changed::<Configuration>),
    );
    #[cfg(not(target_os = "android"))]
    app.add_plugins(WireframePlugin);
    app.add_plugins(EguiPlugin);
    app.add_plugins(DefaultInspectorConfigPlugin);
    // insert and register resource
    #[cfg(not(target_os = "android"))]
    app.init_resource::<Configuration>();
    #[cfg(not(target_os = "android"))]
    app.register_type::<Configuration>();
    app.add_systems(Update, inspector_ui);
}

#[cfg(not(target_os = "android"))]
#[derive(Reflect, Resource)]
#[reflect(Resource)]
struct Configuration {
    name: String,
    toggle: bool,
}

#[cfg(not(target_os = "android"))]
impl Default for Configuration {
    fn default() -> Self {
        Self {
            name: "WireFrame".to_string(),
            toggle: false,
        }
    }
}

fn toggle_wireframe(
    mut commands: Commands,
    landscapes_wireframes: Query<Entity, (With<Terrain>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<Terrain>, Without<Wireframe>)>,
    #[cfg(not(target_os = "android"))] input: Res<Configuration>,
) {
    #[cfg(not(target_os = "android"))]
    {
        if input.toggle {
            for terrain in &landscapes {
                commands.entity(terrain).insert(Wireframe);
            }
            for terrain in &landscapes_wireframes {
                commands.entity(terrain).remove::<Wireframe>();
            }
        }
    }
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

    let egui_context = world
        .query_filtered::<&mut EguiContext, With<PrimaryWindow>>()
        .get_single(world);

    let Ok(egui_context) = egui_context else {
        return;
    };
    let mut egui_context = egui_context.clone();

    egui::Window::new("LimitPush DEBUG")
        .default_size((320., 160.))
        .show(egui_context.get_mut(), |ui| {
            egui::ScrollArea::both().show(ui, |ui| {
                bevy_inspector::ui_for_world(world, ui);
                #[cfg(not(target_os = "android"))]
                bevy_inspector_egui::bevy_inspector::ui_for_resource::<Configuration>(world, ui);
                ui.allocate_space(ui.available_size());
            });
        });
}
