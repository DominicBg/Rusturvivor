use std::f32::consts::PI;

use super::components::*;
use super::spell_factory::*;
use crate::game::minions::components::Minion;
use crate::game::utils::components::*;
use bevy::prelude::*;
use rand::random;

pub fn update_spell_duration(
    mut spell_query: Query<(Entity, &mut Spell)>,
    mut commands: Commands,
    time: Res<Time>,
) {
    let dt = time.delta_seconds();
    for (entity, mut spell) in spell_query.iter_mut() {
        spell.current_lifetime += dt;

        if spell.current_lifetime > spell.life_time {
            commands.entity(entity).despawn();
        }
    }
}

pub fn update_auto_spell_casting(
    mut auto_cast_query: Query<(Entity, &mut AutoCaster, &Transform)>,
    mut commands: Commands,
    asset_server: Res<AssetServer>,
    time: Res<Time>,
) {
    for (entity, mut auto_caster, transform) in auto_cast_query.iter_mut() {
        auto_caster.cast_timer.tick(time.delta());
        if auto_caster.cast_timer.finished() {
            let spell_request = SpellRequest {
                position: transform.translation,
                direction: Vec3::ZERO,
                owner: entity,
                spell: auto_caster.spell,
            };

            spawn_spell(&spell_request, &mut commands, &asset_server);
        }
    }
}

pub fn update_straight_line_spell(
    mut spell_query: Query<(&mut Transform, &StraightLineMovement), With<Spell>>,
    time: Res<Time>,
) {
    let delta_time = time.delta_seconds();
    for (mut transform, straight_line) in spell_query.iter_mut() {
        let velocity = Vec3::new(straight_line.direction.x, straight_line.direction.y, 0.0)
            * straight_line.speed;
        let pos: Vec3 = transform.translation + velocity * delta_time;
        transform.translation = pos;
    }
}

pub fn setup_random_direction_on_spawn(
    mut spell_query: Query<&mut StraightLineMovement, Changed<SetRandomDirectionOnSpawn>>,
) {
    for mut straight_line in spell_query.iter_mut() {
        straight_line.direction = Vec2::from_angle(random::<f32>() * 2.0 * PI);
    }
}

pub fn setup_direction_toward_closest_enemy(
    mut spell_query: Query<
        (&mut StraightLineMovement, &Transform),
        Changed<SetDirectionTowardClosestEnemy>,
    >,
    minion_query: Query<&Transform, With<Minion>>,
) {
    for (mut straight_line, transform) in spell_query.iter_mut() {
        let closest_minion_pos = get_closest_minion_pos(transform.translation, &minion_query);
        println!("closest enemy {}", closest_minion_pos);
        let diff = closest_minion_pos - transform.translation;
        let angle:f32 = diff.y.atan2(diff.x);
        straight_line.direction = Vec2::from_angle(angle);
    }
}

pub fn update_look_at_direction_spell(
    mut spell_query: Query<
        (&mut Transform, &StraightLineMovement),
        (With<Spell>, With<LookAtDirection>),
    >,
) {
    for (mut transform, straight_line) in spell_query.iter_mut() {
        let direction = straight_line.direction;
        let angle = direction.y.atan2(direction.x);
        transform.rotation = Quat::from_rotation_z(angle);
    }
}

pub fn update_spiral_spell(mut spell_query: Query<(&mut Transform, &SpiralSpellMovement, &Spell)>) {
    for (mut transform, spiral, spell) in spell_query.iter_mut() {
        let dist: f32 = spell.current_lifetime * spiral.grow_dist_per_sec;
        let angle: f32 = spell.current_lifetime * spiral.angle_deg_per_sec;

        let angle_vec: Vec2 = Vec2::from_angle(angle.to_radians());
        let pos: Vec2 = spiral.start_pos + angle_vec * dist;

        transform.translation = Vec3 {
            x: pos.x,
            y: pos.y,
            z: 0.0,
        };
    }
}

pub fn synch_damage_area_to_position(mut damage_area_query: Query<(&Transform, &mut DamageArea)>) {
    for (transform, mut damage_area) in damage_area_query.iter_mut() {
        damage_area.position = transform.translation;
    }
}

pub fn get_closest_minion_pos(
    current_pos: Vec3,
    minion_query: &Query<&Transform, With<Minion>>,
) -> Vec3 {
    let mut closest_minion_dist_sq: f32 = 10000.;
    let mut closest_minion_pos: Vec3 = Vec3::ZERO;

    for minion_transform in minion_query.iter() {
        let current_dist_sq: f32 = current_pos.distance_squared(minion_transform.translation);

        if current_dist_sq < closest_minion_dist_sq {
            closest_minion_dist_sq = current_dist_sq;
            closest_minion_pos = minion_transform.translation;
        }
    }
    return closest_minion_pos;
}
