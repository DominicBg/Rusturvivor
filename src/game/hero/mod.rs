pub mod components;
pub mod hero_factory;
pub mod systems;

use bevy::prelude::*;
use systems::*;

use self::components::HeroEvent;

pub struct HeroPlugin;

impl Plugin for HeroPlugin {
    fn build(&self, app: &mut App) {
        app.add_event::<HeroEvent>()
            .add_systems(Startup, spawn_first_hero)
            .add_systems(Update, spawn_hero_event);
    }
}
