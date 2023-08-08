use bevy::prelude::*;

use crate::game::{utils::{systems::create_sprite_bundle, components::Movement}, health::components::Health};

use super::components::*;

pub fn create_hero_bundle(hero_event: &HeroEvent, asset_server: &Res<AssetServer>) -> HeroBundle {
    match hero_event.hero_type {
        HeroType::HeroPaladin => return spawn_hero_paladin(hero_event, asset_server),
        HeroType::HeroMage => return spawn_hero_mage(hero_event, asset_server),
    }
}

pub fn spawn_hero_paladin(hero_event: &HeroEvent, asset_server: &Res<AssetServer>) -> HeroBundle {
    return HeroBundle {
        sprite: create_sprite_bundle("sprites/hero_paladin.png", hero_event.pos, &asset_server),
        health_component: Health {
            current_hp: 200,
            max_hp: 200,
            invinsible_time: 0.0,
        },
        movement: Movement {
            movement_speed: 15.0
        },
    };
}

pub fn spawn_hero_mage(hero_event: &HeroEvent, asset_server: &Res<AssetServer>) -> HeroBundle {
    return HeroBundle {
        sprite: create_sprite_bundle("sprites/hero_mage.png", hero_event.pos, &asset_server),
        health_component: Health {
            current_hp: 200,
            max_hp: 200,
            invinsible_time: 0.0,
        },
        movement: Movement {
            movement_speed: 15.0
        },
    };
}
