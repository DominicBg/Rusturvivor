pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct UtilPlugin;

impl Plugin for UtilPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            (
                rotate_objects_system,
                update_mana_each_second,
            ),
        );
    }
}
