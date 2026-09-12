use bevy::prelude::*;
use godot::prelude::*;
use crate::characters::components::stats::{Damage, Health};
#[derive(Event)]
pub struct DamageEvent{
    pub source: Entity,
    pub target: Option<Entity>,
}

fn on_damage_event(damage_event: On<DamageEvent>, mut query_health: Query<&mut Health>, query_damage: Query<&Damage>){
    godot_print!("Get event");

    godot_print!("Source entity: {:?}, Target entity: {:?}", damage_event.source, damage_event.target);

    let Ok(mut health) = query_health.get_mut(damage_event.target.unwrap()) else {
        return;
    };

    let Ok(damage) = query_damage.get(damage_event.source) else {
        return;
    };

    godot_print!("Target health: {}", health.0);
    health.0 = health.0 - damage.0;
    godot_print!("Target health after: {}", health.0);
}

pub struct DamagePlugin;

impl Plugin for DamagePlugin{
    fn build(&self, app: &mut App) {
            app.add_observer(on_damage_event);
    }
}
