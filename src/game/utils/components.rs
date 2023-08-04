use bevy::prelude::*;

#[derive(Component)]
pub struct Movement{
    pub movement_speed: f32,
}

#[derive(Component, Default)]
pub struct RotateOverTime {
    pub angle_degree: f32,
    pub speed_deg: f32,
}

#[derive(Component)]
pub struct Mana {
    pub current_mana: i32,
    pub max_mana :i32,
    pub mana_regen_per_sec: i32
}

#[derive(PartialEq)]
pub enum TeamType {
    Hero = 0,
    Minion = 1,
}

#[derive(Component)]
pub struct Team{
    pub team_type:TeamType
}

#[derive(Component)]
pub struct DamageArea{
    pub onwer_team_type:TeamType,
    pub damage:i32,
    pub position:Vec3,
    pub area:f32,
    pub owner: Entity
}