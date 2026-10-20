use bevy::prelude::*;
use godot::global::godot_print;

use crate::characters::components::stats::{Health, MaxHealth};

#[derive(Event)]
pub struct HealEvent {
    pub source: Entity,
    pub target: Entity,
    pub heal: Option<f32>,
}

fn on_heal_event(heal_event: On<HealEvent>, mut query_health: Query<(&mut Health, &MaxHealth)>) {
    godot_print!(
        "Source entity: {:?}, Target entity: {:?}",
        heal_event.source,
        heal_event.target
    );

    let Ok((mut health, max_health)) = query_health.get_mut(heal_event.target) else {
        return;
    };

    let Some(heal) = heal_event.heal else {
        return;
    };

    let target_health = health.0 + heal;

    if target_health > max_health.0 {
        health.0 = max_health.0;
    } else {
        health.0 = target_health;
    }
}

pub struct HealPluign;

impl Plugin for HealPluign {
    fn build(&self, app: &mut App) {
        app.add_observer(on_heal_event);
    }
}
