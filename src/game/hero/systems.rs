use bevy::prelude::*;
use super::components::*;

pub fn spawn_hero(mut commands: Commands, asset_server: Res<AssetServer>) {
    commands.spawn((
        SpriteBundle {
            transform: Transform::from_xyz(0.0, 0.0, 0.0),
            texture: asset_server.load("sprites/hero.png"),
            ..default()
        },
        Hero {},
    ));
}

// TODO hero moving and dodging lol