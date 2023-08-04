use bevy::prelude::*;

use hero::HeroPlugin;
use health::HealthPlugin;
use minions::MinionPlugin;
use spells::SpellPlugin;
use utils::UtilPlugin;

use self::health::components::*;


pub mod hero;
pub mod health;
pub mod minions;
pub mod spells;
pub mod utils;

pub struct GamePlugin;

impl Plugin for GamePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, spawn_camera)
            .add_event::<DamageEvent>()
            .add_event::<KillEvent>()
            .add_plugins((SpellPlugin, HealthPlugin, UtilPlugin, HeroPlugin, MinionPlugin));
    }
}

pub fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2dBundle {
        transform: Transform::from_xyz(0.0, 0.0, 0.1),
        projection: OrthographicProjection {
            scale: 0.2,
            ..default()
        },

        ..default()
    });
}
