pub mod components;
pub mod systems;

use bevy::prelude::*;
use systems::*;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_hero);
    }
}