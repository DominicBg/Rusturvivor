use bevy::prelude::*;

use crate::game::{health::components::Health, utils::components::Movement};

#[derive(Component)]
pub struct Hero {
    pub hero_type: HeroType,
}

#[derive(PartialEq, Clone, Copy)]
pub enum HeroType {
    HeroPaladin = 1,
    HeroMage = 2,
}

#[derive(Event)]
pub struct HeroEvent {
    pub hero_type: HeroType,
    pub pos: Vec3,
}

#[derive(Bundle)]
pub struct HeroBundle {
    pub sprite: SpriteBundle,
    pub health_component: Health,
    pub movement: Movement
}
