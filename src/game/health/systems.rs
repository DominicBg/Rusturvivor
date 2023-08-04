use crate::game::utils::components::*;

use super::components::*;
use bevy::prelude::*;

pub const INVINSIBILITY_TIMER: f32 = 0.5;
pub const INVINSIBILITY_FLICKER_TIMES: f32 = 5.0;

pub fn update_health(mut health_query: Query<&mut Health>, time: Res<Time>) {
    let delta_time = time.delta_seconds();
    for mut health in health_query.iter_mut() {
        health.invinsible_time -= delta_time;
        health.invinsible_time = health.invinsible_time.max(0.0);
    }
}

pub fn update_damage_hits(
    mut health_query: Query<(Entity, &mut Health, &Transform, &Team)>,
    damage_area_query: Query<&DamageArea>,
    mut commands: Commands,
    mut ev_damage: EventWriter<DamageEvent>,
    mut ev_death: EventWriter<KillEvent>,
) {
    for (entity, mut health, transform, team) in health_query.iter_mut() {
        for damage_area in damage_area_query.iter() {
            let area_sq = damage_area.area.powi(2);

            if team.team_type != damage_area.onwer_team_type
                && health.invinsible_time <= 0.0
                && transform.translation.distance_squared(damage_area.position) < area_sq
            {
                //health
                health.invinsible_time = INVINSIBILITY_TIMER;
                health.current_hp -= damage_area.damage;

                ev_damage.send(DamageEvent {
                    attacker: damage_area.owner,
                    receiver: entity,
                    damage: damage_area.damage,
                });

                if health.current_hp <= 0 {
                    //create events?

                    ev_death.send(KillEvent {
                        killer: damage_area.owner,
                        dead: entity,
                    });
                    commands.entity(entity).despawn();
                }
            }
        }
    }
}

pub fn sprite_damage_flash(mut sprite_query: Query<(&Health, &mut Sprite)>) {
    for (health, mut sprite) in sprite_query.iter_mut() {
        
        if health.invinsible_time <= 0.0 {
            //set alpha back to 0?
            continue;
        }

        let invincible_flicker_step = INVINSIBILITY_TIMER / INVINSIBILITY_FLICKER_TIMES;
        let invincible_flicker_threshold =
            INVINSIBILITY_TIMER / (INVINSIBILITY_FLICKER_TIMES * 2.0);

        let alpha: f32 =
            if (health.invinsible_time % invincible_flicker_step) > invincible_flicker_threshold {
                0.0
            } else {
                1.0
            };

        sprite.color.set_a(alpha);
    }
}
