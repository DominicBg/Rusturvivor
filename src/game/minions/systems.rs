use super::components::*;
use crate::game::health::components::*;
use crate::game::hero::components::*;
use crate::game::utils::components::*;

use bevy::prelude::*;
use rand::prelude::*;

pub const MAP_SIZE: f32 = 150.0;
pub const MAP_SIZE_SPAWN_OFFSET: f32 = 5.0;

pub fn spawn_overlord(mut commands: Commands) {
    commands.spawn((
        Overlord {
            is_spawning_enemies: true,
            spawning_enemy_index: 0,
            minion_control: MinionControl::TowardHero,
        },
        Mana {
            mana_regen_per_sec: 5,
            current_mana: 45,
            max_mana: 45,
        },
    ));

    println!("I AM AWAKEN");
}

pub fn update_overlord_controls(
    mut overlord_query: Query<&mut Overlord>,
    keyboard_input: Res<Input<KeyCode>>,
) {
    let mut overlord = overlord_query
        .get_single_mut()
        .expect("Overload doesn't exists");

    if keyboard_input.just_pressed(KeyCode::Key1) {
        overlord.minion_control = MinionControl::TowardHero;
    } else if keyboard_input.just_pressed(KeyCode::Key2) {
        overlord.minion_control = MinionControl::Circle;
    } else if keyboard_input.just_pressed(KeyCode::Key3) {
        overlord.minion_control = MinionControl::SameDirection;
    }
}

pub fn lerp(a: Vec3, b: Vec3, t: f32) -> Vec3 {
    return a.lerp(b, t);
}

pub fn spawn_minion(
    mut overlord_query: Query<(&mut Overlord, &mut Mana)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
) {
    let (overlord, mut mana) = overlord_query
        .get_single_mut()
        .expect("Overload doesn't exists");

    if overlord.is_spawning_enemies && mana.current_mana > MANA_COST_SPAWN_MINION {
        mana.current_mana -= MANA_COST_SPAWN_MINION;

        let corners: [Vec3; 4] = [
            Vec3::new(-MAP_SIZE, MAP_SIZE, 0.0),
            Vec3::new(MAP_SIZE, MAP_SIZE, 0.0),
            Vec3::new(MAP_SIZE, -MAP_SIZE, 0.0),
            Vec3::new(-MAP_SIZE, -MAP_SIZE, 0.0),
        ];
        let spawn_corner_index1: usize = random::<usize>() % corners.len();
        let spawn_corner_index2: usize = (spawn_corner_index1 + 1) % corners.len();

        let spawn_corner1: Vec3 = corners[spawn_corner_index1];
        let spawn_corner2: Vec3 = corners[spawn_corner_index2];

        let spawn_pos: Vec3 = lerp(spawn_corner1, spawn_corner2, random::<f32>());

        //check minion index to spawn good archetype
        let entity = commands
            .spawn((
                SpriteBundle {
                    transform: Transform {
                        translation: spawn_pos,
                        ..default()
                    },
                    texture: asset_server.load("sprites/minion1.png"),
                    ..default()
                },
                Minion {},
                Health {
                    current_hp: 35,
                    max_hp: 35,
                    invinsible_time: 0.0,
                },
                Movement {
                    movement_speed: 25.0,
                },
                Team {
                    team_type: TeamType::Minion,
                },
            ))
            .id();

        commands.entity(entity).insert(DamageArea {
            area: 16.0,
            onwer_team_type: TeamType::Minion,
            damage: 1,
            owner: entity,
            position: Vec3::ZERO,
        });
    }
}

pub fn update_minions_toward_closest_hero(
    mut minion_query: Query<(&mut Transform, &Movement), (With<Minion>, Without<Hero>)>,
    hero_query: Query<&Transform, (With<Hero>, Without<Minion>)>,
    overlord_query: Query<&Overlord>,
    time: Res<Time>,
) {
    let minion_control: MinionControl = overlord_query.single().minion_control;
    if minion_control != MinionControl::TowardHero {
        return;
    }

    let delta_time = time.delta_seconds();
    for (mut minion_transform, movement) in minion_query.iter_mut() {
        let closest_hero_pos: Vec3 =
            get_closest_hero_pos(minion_transform.translation, &hero_query);

        let direction: Vec3 = (closest_hero_pos - minion_transform.translation).normalize();
        minion_transform.translation += direction * delta_time * movement.movement_speed;
    }
}

pub fn update_minions_circle(
    mut minion_query: Query<(&mut Transform, &Movement), (With<Minion>, Without<Hero>)>,
    hero_query: Query<&Transform, (With<Hero>, Without<Minion>)>,
    overlord_query: Query<&Overlord>,
    time: Res<Time>,
) {
    let minion_control: MinionControl = overlord_query.single().minion_control;
    if minion_control != MinionControl::Circle {
        return;
    }

    let delta_time = time.delta_seconds();
    for (mut minion_transform, movement) in minion_query.iter_mut() {
        let closest_hero_pos: Vec3 =
            get_closest_hero_pos(minion_transform.translation, &hero_query);

        let direction_toward_hero: Vec3 =
            (closest_hero_pos - minion_transform.translation).normalize();

        let direction: Vec3 = Vec3::new(
            direction_toward_hero.y,
            -direction_toward_hero.x,
            direction_toward_hero.z,
        );
        minion_transform.translation += direction * delta_time * movement.movement_speed;
    }
}

pub fn update_minions_same_direction(
    mut minion_query: Query<(&mut Transform, &Movement), (With<Minion>, Without<Hero>)>,
    overlord_query: Query<&Overlord>,
    keyboard_input: Res<Input<KeyCode>>,
    time: Res<Time>,
) {
    let minion_control: MinionControl = overlord_query.single().minion_control;
    if minion_control != MinionControl::SameDirection {
        return;
    }

    let mut direction = Vec3::ZERO;
    if keyboard_input.pressed(KeyCode::Left) || keyboard_input.pressed(KeyCode::A) {
        direction += Vec3::new(-1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Right) || keyboard_input.pressed(KeyCode::D) {
        direction += Vec3::new(1.0, 0.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Up) || keyboard_input.pressed(KeyCode::W) {
        direction += Vec3::new(0.0, 1.0, 0.0);
    }
    if keyboard_input.pressed(KeyCode::Down) || keyboard_input.pressed(KeyCode::S) {
        direction += Vec3::new(0.0, -1.0, 0.0);
    }

    if direction.length() > 1.0 {
        direction = direction.normalize();
    }

    if direction == Vec3::ZERO {
        return;
    }

    let delta_time = time.delta_seconds();
    for (mut minion_transform, movement) in minion_query.iter_mut() {
        minion_transform.translation += direction * delta_time * movement.movement_speed;
    }
}

pub fn update_loop_over_map(
    mut minion_query: Query<&mut Transform, (With<Minion>, Without<Camera2d>)>,
    camera_query: Query<&Transform, With<Camera2d>>,
) {
    let camera = camera_query.single();
    let camera_pos = camera.translation;

    for mut minion_transform in minion_query.iter_mut() {
        let camera_minion_diff = minion_transform.translation - camera_pos;

        if camera_minion_diff.length_squared() > MAP_SIZE.powi(2) {
            minion_transform.translation =
                camera_pos - camera_minion_diff.normalize() * (MAP_SIZE - MAP_SIZE_SPAWN_OFFSET);
        }
    }
}

pub fn get_closest_hero_pos(
    current_pos: Vec3,
    hero_query: &Query<&Transform, (With<Hero>, Without<Minion>)>,
) -> Vec3 {
    let mut closest_hero_dist_sq: f32 = 10000.;
    let mut closest_hero_pos: Vec3 = Vec3::ZERO;

    for hero_transform in hero_query.iter() {
        let current_dist_sq: f32 = current_pos.distance_squared(hero_transform.translation);

        if current_dist_sq < closest_hero_dist_sq {
            closest_hero_dist_sq = current_dist_sq;
            closest_hero_pos = hero_transform.translation;
        }
    }
    return closest_hero_pos;
}
