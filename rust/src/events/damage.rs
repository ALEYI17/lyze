use crate::{characters::components::{state::Alive, stats::{Damage, Health}}, events::combat::DiedInCombat, state::GameState};
use bevy::prelude::*;
use godot::prelude::*;

#[derive(Event)]
pub struct DamageEvent {
    pub source: Entity,
    pub target: Entity,
    pub damage: Option<f32>,
}

#[derive(Clone, Copy, PartialEq, Eq)]
pub enum DeathCause{
    Combat,
    Enviroment,
}
#[derive(Event)]
pub struct Die{
    pub entity: Entity,
    pub cause: DeathCause,
}

fn on_damage_event(
    damage_event: On<DamageEvent>,
    mut query_health: Query<(&mut Health, &mut Alive)>,
    query_damage: Query<&Damage>,
    mut commands: Commands,
    state: Res<State<GameState>>
) {
    godot_print!("Get event");

    godot_print!(
        "Source entity: {:?}, Target entity: {:?}",
        damage_event.source,
        damage_event.target
    );

    let Ok((mut health, mut alive)) = query_health.get_mut(damage_event.target) else {
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

    if health.0 <= 0.0{
        alive.0 = false;
        if *state == GameState::InCombat{
            commands.trigger(Die{entity: damage_event.target,cause: DeathCause::Combat});
        }else if *state == GameState::Ingame3D{
            commands.trigger(Die{entity: damage_event.target,cause: DeathCause::Enviroment});
        }
        
    }

}

fn on_die(trigger: On<Die>, mut commands: Commands){

    match trigger.cause {
        DeathCause::Combat => {
            commands.trigger(DiedInCombat{entity: trigger.entity});
        }
        DeathCause::Enviroment => {
            godot_print!("Die by enviroment");
        }
    }

}

pub struct DamagePlugin;

impl Plugin for DamagePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_damage_event)
            .add_observer(on_die);
    }
}
