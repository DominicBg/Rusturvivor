use bevy::prelude::*;

pub const MANA_COST_SPAWN_MINION: i32 = 5;

#[derive(PartialEq, Clone, Copy)]
pub enum MinionControl {
    TowardHero = 0,
    SameDirection = 1,
    Circle = 2,
}

#[derive(Component)]
pub struct Minion{

}

#[derive(Component)]
pub struct Overlord{
    pub is_spawning_enemies: bool,
    pub spawning_enemy_index: i32,
    pub minion_control: MinionControl
}

//#[derive(Event)]
//struct SpawnMinionEvent(Entity);
