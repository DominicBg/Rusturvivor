use bevy::prelude::*;

#[derive(Component)]
pub struct Health{
    pub current_hp: i32,
    pub max_hp: i32,
    pub invinsible_time: f32
}

//#[derive(Event)]
pub struct DamageEvent{
    pub attacker: Entity,
    pub receiver: Entity,
    pub damage: i32
}

//#[derive(Event)]
pub struct KillEvent{
    pub killer: Entity,
    pub dead: Entity,
}
