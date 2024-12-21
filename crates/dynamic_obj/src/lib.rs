use dep_reexp::bevy::prelude::*;

mod spawn;
use spawn::SpawnDynamicObjects;
mod update;
use update::UpdateDynamicObjects;

pub struct DynamicObjectsPlugin;

impl Plugin for DynamicObjectsPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((SpawnDynamicObjects, UpdateDynamicObjects));
    }
}