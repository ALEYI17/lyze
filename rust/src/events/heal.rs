use bevy::prelude::*;
use godot::global::godot_print;

use crate::characters::components::stats::Health;

#[derive(Event)]
pub struct HealEvent {
    pub source: Entity,
    pub target: Entity,
    pub heal: Option<f32>,
}

fn on_heal_event(damage_event: On<HealEvent>, mut query_health: Query<&mut Health>) {
    godot_print!(
        "Source entity: {:?}, Target entity: {:?}",
        damage_event.source,
        damage_event.target
    );

    let Ok(mut health) = query_health.get_mut(damage_event.target) else {
        return;
    };

    let Some(heal) = damage_event.heal else {
        return;
    };

    godot_print!("Target health: {}", health.0);
    health.0 += heal;
    godot_print!("Target health after: {}", health.0);
}

pub struct HealPluign;

impl Plugin for HealPluign {
    fn build(&self, app: &mut App) {
        app.add_observer(on_heal_event);
    }
}
