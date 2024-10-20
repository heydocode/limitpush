use bevy::pbr::wireframe::WireframePlugin;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(Update, toggle_wireframe.run_if(in_state(Screen::Playing)));
    app.add_plugins(WireframePlugin);
}

fn toggle_wireframe(
    mut commands: Commands,
    landscapes_wireframes: Query<Entity, (With<Terrain>, With<Wireframe>)>,
    landscapes: Query<Entity, (With<Terrain>, Without<Wireframe>)>,
    input: Res<ButtonInput<KeyCode>>,
) {
    if input.just_pressed(KeyCode::Space) {
        for terrain in &landscapes {
            commands.entity(terrain).insert(Wireframe);
        }
        for terrain in &landscapes_wireframes {
            commands.entity(terrain).remove::<Wireframe>();
        }
    }
}
