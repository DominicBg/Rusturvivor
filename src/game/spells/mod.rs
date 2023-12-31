pub mod components;
mod systems;
mod spell_factory;
mod spell_autocaster_factory;

use bevy::prelude::*;
use systems::*;
use spell_autocaster_factory::*;

pub struct SpellPlugin;

impl Plugin for SpellPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, create_spell_auto_caster_default)
            .add_systems(Update,(update_spell_duration))
            .add_systems(Update,update_auto_spell_casting)
            .add_systems(Update,(setup_random_direction_on_spawn, setup_direction_toward_closest_enemy))
            .add_systems(Update,(update_spiral_spell, update_straight_line_spell, update_look_at_direction_spell).after(update_spell_duration));
    }
}
