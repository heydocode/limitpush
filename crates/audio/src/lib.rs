use bevy::prelude::*;
use bevy_kira_audio::prelude::*;
use states::screens::loading::AudioAssets;
use states::screens::Screen;

pub fn plugin(app: &mut App) {
    app.add_plugins(AudioPlugin);
    app.add_systems(OnEnter(Screen::Playing), start_audio);
}

// TODO!: implement a struct of background music with
// handle (can be multiple), current handle, volume
// and maybe more params. Also implement a system which
// rerun the background music with updated changes when
// the BackgroundAudio ressource is changed so the BackgroundAudio 
// will be this struct with all params

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
