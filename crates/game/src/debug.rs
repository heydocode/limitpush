use bevy::prelude::*;
use bevy::remote::{http::RemoteHttpPlugin, RemotePlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()));
    }
}
