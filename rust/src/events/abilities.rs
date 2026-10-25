use bevy::prelude::*;
use godot::global::godot_print;

use crate::{
    abilities::{
        definition::{AbilityEffect, AbilityId, calculate_effect_value},
        registry::AbilityRegistry,
    },
    characters::{components::convictions::Convictions, inventories::abilities::AbilityInventory},
    combat::attack::AttackDefinition,
    events::{damage::DamageEvent, heal::HealEvent, stat_modifier::StatModifierEvent},
    ui::hud::ReloadAbilityListEvent,
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

#[derive(Event)]
pub struct LearnAbilityEvent {
    pub ability: AbilityId,
    pub avaible_points: u32,
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

fn on_learn_ability_event(
    trigger: On<LearnAbilityEvent>,
    mut ability_registry: ResMut<AbilityRegistry>,
    mut ability_inventory: ResMut<AbilityInventory>,
    mut commands: Commands,
) {
    godot_print!("get learn ability event");
    let Some(ability) = ability_registry.get_mut(trigger.ability) else {
        return;
    };

    if trigger.avaible_points >= ability.point_cost {
        ability_inventory.avaible_points -= ability.point_cost;
        ability.point_cost = 0;
        ability.learned = true;
        commands.trigger(ReloadAbilityListEvent);
    }
}

pub struct AbilitiesPlugin;

impl Plugin for AbilitiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_ability_event)
            .add_observer(on_learn_ability_event);
    }
}
