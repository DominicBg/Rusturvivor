use super::components::*;
use bevy::prelude::*;
use std::time::Duration;

use crate::game::utils::components::*;

pub fn create_spell_auto_caster_default(mut commands: Commands) {
    commands.spawn(create_spell_auto_caster_bundle(AutoCasterRequest {
        spell: SpellEnum::Hammer,
    }));
    commands.spawn(create_spell_auto_caster_bundle(AutoCasterRequest {
        spell: SpellEnum::Sword,
    }));
    commands.spawn(create_spell_auto_caster_bundle(AutoCasterRequest {
        spell: SpellEnum::Orb,
    }));
}

pub fn create_spell_auto_caster_bundle(auto_caster_request: AutoCasterRequest) -> AutoCasterBundle {
    match auto_caster_request.spell {
        SpellEnum::Hammer => return spell_hammer_auto_caster_bundle(auto_caster_request),
        SpellEnum::Sword => return spell_sword_auto_caster_bundle(auto_caster_request),
        SpellEnum::Orb => return spell_orb_auto_caster_bundle(auto_caster_request),
    }
}



fn spell_hammer_auto_caster_bundle(auto_caster_request: AutoCasterRequest) -> AutoCasterBundle {
    return AutoCasterBundle {
        auto_caster: AutoCaster {
            cast_timer: Timer::new(Duration::from_secs_f32(2.25), TimerMode::Repeating),
            team_type: TeamType::Hero,
            spell: SpellEnum::Hammer,
        },

        transform: Transform {
            
            //link as a child to player
            translation: Vec3::ZERO,
            ..default()
        },
    };
}

fn spell_sword_auto_caster_bundle(auto_caster_request: AutoCasterRequest) -> AutoCasterBundle {
    return AutoCasterBundle {
        auto_caster: AutoCaster {
            cast_timer: Timer::new(Duration::from_secs_f32(0.25), TimerMode::Repeating),
            team_type: TeamType::Hero,
            spell: SpellEnum::Sword,
        },

        transform: Transform {
            translation: Vec3::ZERO,
            ..default()
        },
    };
}

fn spell_orb_auto_caster_bundle(auto_caster_request: AutoCasterRequest) -> AutoCasterBundle {
    return AutoCasterBundle {
        auto_caster: AutoCaster {
            cast_timer: Timer::new(Duration::from_secs_f32(0.5), TimerMode::Repeating),
            team_type: TeamType::Hero,
            spell: SpellEnum::Orb,
        },

        transform: Transform {
            translation: Vec3::ZERO,
            ..default()
        },
    };
}
