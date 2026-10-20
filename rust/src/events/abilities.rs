use bevy::prelude::*;

use crate::{
    abilities::{
        definition::{AbilityEffect, AbilityId, calculate_ability_value},
        registry::AbilityRegistry,
    },
    characters::components::convictions::Convictions,
    events::{damage::DamageEvent, heal::HealEvent},
};

#[derive(Event)]
pub struct AbilityEvent {
    pub ability: AbilityId,
    pub source: Entity,
    pub target: Entity,
}

fn on_ability_event(
    trigger: On<AbilityEvent>,
    ability_registry: Res<AbilityRegistry>,
    query: Query<&Convictions>,
    mut commands: Commands,
) {
    let Some(ability) = ability_registry.get(trigger.ability) else {
        return;
    };

    let Ok(convictions) = query.get(trigger.source) else {
        return;
    };

    let Ok(target_convictions) = query.get(trigger.target) else {
        return;
    };

    match &ability.effect {
        AbilityEffect::Damage {
            base: _,
            scalings: _,
        } => {
            let damage = calculate_ability_value(ability, convictions, target_convictions);

            commands.trigger(DamageEvent {
                source: trigger.source,
                target: trigger.target,
                damage,
            });
        }
        AbilityEffect::Heal {
            base: _,
            scalings: _,
        } => {
            let heal = calculate_ability_value(ability, convictions, target_convictions);
            commands.trigger(HealEvent {
                source: trigger.source,
                target: trigger.target,
                heal,
            });
        }
    }
}

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_ability_event);
    }
}
