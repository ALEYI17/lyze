use crate::characters::components::stats::{Damage, Health};
use bevy::prelude::*;
use godot::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
    pub damage: Option<f32>,
}

fn on_damage_event(
    damage_event: On<DamageEvent>,
    mut query_health: Query<&mut Health>,
    query_damage: Query<&Damage>,
) {
    godot_print!("Get event");

    godot_print!(
        "Source entity: {:?}, Target entity: {:?}",
        damage_event.source,
        damage_event.target
    );

    let Ok(mut health) = query_health.get_mut(damage_event.target) else {
        return;
    };

    let damage = damage_event.damage.unwrap_or_else(|| {
        query_damage
            .get(damage_event.source)
            .map_or(0.0, |damage| damage.0)
    });

    godot_print!("Target health: {}", health.0);
    health.0 -= damage;
    godot_print!("Target health after: {}", health.0);
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_damage_event);
    }
}
