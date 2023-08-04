pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_startup_system(spawn_overlord)
            .add_system(spawn_minion)
            .add_systems(
                (
                    update_overlord_controls,
                    update_minions_toward_closest_hero,
                    update_minions_circle,
                    update_minions_same_direction,
                    update_loop_over_map
                ),
            );
    }
}
