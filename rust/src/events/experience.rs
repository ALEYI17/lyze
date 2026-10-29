use bevy::prelude::*;
use godot::global::godot_print;

use crate::{
    characters::components::progression::{Experience, Level},
    progression::experience::process_level_up,
};

#[derive(Event)]
pub struct GainExperienceEvent {
    pub experience: u32,
    pub target: Entity,
}

fn on_gain_experience_event(
    trigger: On<GainExperienceEvent>,
    mut query: Query<(&mut Experience, &mut Level)>,
) {
    let Ok((mut experience, mut level)) = query.get_mut(trigger.target) else {
        return;
    };

    experience.0 = experience.0.saturating_add(trigger.experience);

    godot_print!("Gain {} experience", experience.0);

    let level_gained = process_level_up(&mut level, &mut experience);

    if level_gained > 0 {
        godot_print!("Level up {} levels", level_gained);
    }
}

pub struct ExperiencePlugin;

impl Plugin for ExperiencePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_gain_experience_event);
    }
}
