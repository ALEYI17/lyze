use bevy::prelude::*;
use godot::global::godot_print;

use crate::{
    characters::{
        components::progression::{Experience, Level},
        inventories::{abilities::AbilityInventory, convictions::ConvictionPoints},
    },
    progression::experience::process_level_up,
    ui::hud::ReloadAbilityListEvent,
};

const CONVICTION_POINTS_ADDED: u32 = 3;
const ABILITY_POINTS_ADDED: u32 = 1;

#[derive(Event)]
pub struct GainExperienceEvent {
    pub experience: u32,
    pub target: Entity,
}

fn on_gain_experience_event(
    trigger: On<GainExperienceEvent>,
    mut query: Query<(&mut Experience, &mut Level)>,
    mut conviction_inventory: ResMut<ConvictionPoints>,
    mut ability_inventory: ResMut<AbilityInventory>,
    mut commands: Commands,
) {
    let Ok((mut experience, mut level)) = query.get_mut(trigger.target) else {
        return;
    };

    experience.0 = experience.0.saturating_add(trigger.experience);

    godot_print!("Gain {} experience", experience.0);

    let level_gained = process_level_up(&mut level, &mut experience);

    if level_gained > 0 {
        godot_print!("Level up {} levels", level_gained);
        conviction_inventory.avaible_points += CONVICTION_POINTS_ADDED;
        ability_inventory.avaible_points += ABILITY_POINTS_ADDED;
        commands.trigger(ReloadAbilityListEvent)
    }
}

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_gain_experience_event);
    }
}
