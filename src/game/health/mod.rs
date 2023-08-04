pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct HealthPlugin;

impl Plugin for HealthPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Update,(update_health, update_damage_hits, sprite_damage_flash));
    }
}
