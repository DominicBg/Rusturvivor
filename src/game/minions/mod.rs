pub mod components;
mod systems;

use bevy::prelude::*;
use systems::*;

pub struct MinionPlugin;

impl Plugin for MinionPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_overlord)
            .add_systems(
                Update,
                (
                    spawn_minion,
                    update_overlord_controls,
                    update_minions_toward_closest_hero,
                    update_minions_circle,
                    update_minions_same_direction,
                    update_loop_over_map
                ),
            );
    }
}
