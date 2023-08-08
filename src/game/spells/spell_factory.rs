use super::components::*;
use bevy::prelude::*;

use crate::game::utils::components::*;
use crate::game::utils::systems::*;

pub fn spawn_spell(
    spell_request: &SpellRequest,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    match spell_request.spell {
        SpellEnum::Hammer => spawn_spell_hammer(&spell_request, commands, &asset_server),
        SpellEnum::Sword => spawn_spell_sword(&spell_request, commands, &asset_server),
        SpellEnum::Orb => spawn_spell_orb(&spell_request, commands, &asset_server),
    }
}

pub fn spawn_spell_hammer(
    spell_request: &SpellRequest,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        create_sprite_bundle("sprites/hammer.png", spell_request.position, asset_server),
        Spell {
            life_time: 5.0,
            current_lifetime: 0.0,
        },
        SpiralSpellMovement {
            grow_dist_per_sec: 20.0,
            start_pos: Vec2::new(spell_request.position.x, spell_request.position.y),
            angle_deg_per_sec: 180.0,
        },
        RotateOverTime {
            speed_deg: 2.0 * 360.0,
            ..default()
        },
        DamageArea {
            area: 16.0,
            damage: 5,
            onwer_team_type: TeamType::Hero,
            position: Vec3::ZERO,
            owner: spell_request.owner,
        },
    ));
}

pub fn spawn_spell_sword(
    spell_request: &SpellRequest,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        create_sprite_bundle("sprites/sword.png", spell_request.position, asset_server),
        Spell {
            life_time: 0.5,
            ..default()
        },
        DamageArea {
            area: 16.0,
            damage: 1,
            onwer_team_type: TeamType::Hero,
            position: Vec3::ZERO,
            owner: spell_request.owner,
        },
        StraightLineMovement {
            speed: 200.0,
            ..default()
        },
        SetRandomDirectionOnSpawn {},
        LookAtDirection {},
    ));
}

pub fn spawn_spell_orb(
    spell_request: &SpellRequest,
    commands: &mut Commands,
    asset_server: &Res<AssetServer>,
) {
    commands.spawn((
        create_sprite_bundle("sprites/orb.png", spell_request.position, asset_server),
        Spell {
            life_time: 2.0,
            ..default()
        },
        DamageArea {
            area: 16.0,
            damage: 15,
            onwer_team_type: TeamType::Hero,
            position: Vec3::ZERO,
            owner: spell_request.owner,
        },
        StraightLineMovement {
            speed: 100.0,
            ..default()
        },
        SetDirectionTowardClosestEnemy {},
        RotateOverTime {
            speed_deg: 45.0,
            ..default()
        },
    ));
}

