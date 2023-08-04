use crate::game::utils::components::TeamType;
use bevy::prelude::*;

#[derive(PartialEq, Clone, Copy)]
pub enum SpellEnum {
    Hammer = 0,
    Sword = 1,
    Orb = 2,
}

#[derive(Component, Default)]
pub struct Spell {
    pub current_lifetime: f32,
    pub life_time: f32,
}

pub struct SpellRequest {
    pub position: Vec3,
    pub direction: Vec3,
    pub owner: Entity,
    pub spell: SpellEnum,
}

#[derive(Component)]
pub struct AutoCaster {
    pub cast_timer: Timer, //add architype
    pub team_type: TeamType,
    pub spell: SpellEnum,
}

pub struct AutoCasterRequest {
    pub spell: SpellEnum,
}

#[derive(Component)]
pub struct SpiralSpellMovement {
    pub start_pos: Vec2,
    pub angle_deg_per_sec: f32,
    pub grow_dist_per_sec: f32,
}


#[derive(Component)]
pub struct SetRandomDirectionOnSpawn{}

#[derive(Component)]
pub struct SetDirectionTowardClosestEnemy{}

#[derive(Component, Default)]
pub struct StraightLineMovement {
    pub speed: f32,
    pub direction: Vec2
}

#[derive(Component)]
pub struct LookAtDirection {
}

