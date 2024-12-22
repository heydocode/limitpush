use dep_reexp::bevy::prelude::*;
use dep_reexp::bevy::remote::{RemotePlugin, http::RemoteHttpPlugin};

pub struct DebugPlugin;

impl Plugin for DebugPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((RemotePlugin::default(), RemoteHttpPlugin::default()));
        println!("registered!");
    }
}