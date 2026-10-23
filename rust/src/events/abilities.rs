use bevy::prelude::*;

use crate::{
    abilities::{
        definition::{AbilityEffect, AbilityId, calculate_effect_value},
        registry::AbilityRegistry,
    },
    characters::components::convictions::Convictions,
    combat::attack::AttackDefinition,
    events::{damage::DamageEvent, heal::HealEvent, stat_modifier::StatModifierEvent},
};

#[derive(Debug, Clone)]
pub enum AbilitySource {
    Player(AbilityId),
    Enemy(AttackDefinition),
}

#[derive(Event)]
pub struct AbilityEvent {
    pub ability: AbilitySource,
    pub source: Entity,
    pub target: Entity,
}

fn on_ability_event(
    trigger: On<AbilityEvent>,
    ability_registry: Res<AbilityRegistry>,
    query: Query<&Convictions>,
    mut commands: Commands,
) {
    // let Some(ability) = ability_registry.get(trigger.ability) else {
    //     return;
    // };

    let Ok(convictions) = query.get(trigger.source) else {
        return;
    };

    let Ok(target_convictions) = query.get(trigger.target) else {
        return;
    };

    let effect = match &trigger.ability {
        AbilitySource::Player(ability_id) => {
            let Some(ability) = ability_registry.get(*ability_id) else {
                return;
            };
            &ability.effect
        }
        AbilitySource::Enemy(attack) => &attack.effect,
    };

    match &effect {
        AbilityEffect::Damage {
            base: _,
            scalings: _,
        } => {
            let damage = calculate_effect_value(effect, convictions, target_convictions);

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
            let heal = calculate_effect_value(effect, convictions, target_convictions);
            commands.trigger(HealEvent {
                source: trigger.source,
                target: trigger.target,
                heal,
            });
        }
        AbilityEffect::StatModifier {
            conviction,
            stat,
            modifier,
        } => {
            commands.trigger(StatModifierEvent {
                target: trigger.target,
                conviction: *conviction,
                stat: *stat,
                modifier: modifier.clone(),
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
