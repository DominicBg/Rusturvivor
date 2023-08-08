use super::components::*;
use super::hero_factory::*;
use bevy::prelude::*;

pub fn spawn_first_hero(
    mut ev_spawn_hero: EventWriter<HeroEvent>,
) {
    ev_spawn_hero.send(HeroEvent{
        pos: Vec3::ZERO,
        hero_type: HeroType::HeroPaladin
    });
}

pub fn spawn_hero_event(
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    mut ev_spawn_hero: EventReader<HeroEvent>,
) {
    for hero_event in ev_spawn_hero.iter() {
        commands.spawn(create_hero_bundle(&hero_event, &asset_server));
    }
}



// TODO hero moving and dodging lol
