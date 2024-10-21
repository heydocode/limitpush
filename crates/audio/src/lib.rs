use states::screens::loading::AudioAssets;
use states::screens::Screen;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin);
    app.add_systems(OnEnter(Screen::Playing), start_audio);
}

#[derive(Resource)]
pub struct BackgroundAudio(pub Handle<AudioInstance>);

pub fn start_audio(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    state: Res<State<Screen>>,
) {
    let handle = match *state.get() {
        Screen::Playing => audio
            .play(audio_assets.flying.clone())
            .looped()
            .with_volume(0.01)
            .handle(),
        _ => return,
    };
    commands.insert_resource(BackgroundAudio(handle));
}

pub fn toggle_audio(
    audio: Res<BackgroundAudio>,
    mut audio_instances: ResMut<Assets<AudioInstance>>,
) {
    if let Some(instance) = audio_instances.get_mut(&audio.0) {
        match instance.state() {
            PlaybackState::Paused { .. } => {
                instance.resume(AudioTween::default());
            }
            PlaybackState::Playing { .. } => {
                instance.pause(AudioTween::default());
            }
            _ => {}
        }
    }
}