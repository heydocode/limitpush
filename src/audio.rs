use crate::states::screens::loading::AudioAssets;
use crate::Screen;
use bevy::prelude::*;
use bevy_kira_audio::prelude::*;

pub struct InternalAudioPlugin;

// This plugin is responsible to control the game audio
impl Plugin for InternalAudioPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AudioPlugin);
        app.add_systems(OnEnter(Screen::Playing), start_audio);
    }
}

#[derive(Resource)]
pub struct BackgroundAudio(pub Handle<AudioInstance>);

pub fn start_audio(
    mut commands: Commands,
    audio_assets: Res<AudioAssets>,
    audio: Res<Audio>,
    state: Res<State<Screen>>,
) {
    // audio.pause();
    if state.get() == &Screen::Playing {
        let handle = audio
            .play(audio_assets.flying.clone())
            .looped()
            .with_volume(0.01)
            .handle();
        commands.insert_resource(BackgroundAudio(handle));
    } else {
        eprintln!("An error occured: tried to start background music not on Playing state!");
    }
}

pub fn control_flying_sound(
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
