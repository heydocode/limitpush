use dep_reexp::bevy::prelude::*;

mod spawn;
use spawn::SpawnStaticObjects;

pub struct StaticObjectsPlugin;

impl Plugin for StaticObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(SpawnStaticObjects);
    }
}