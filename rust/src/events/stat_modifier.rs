use bevy::prelude::*;

use crate::{
    abilities::definition::{ConvictionType, ModifierType},
    characters::components::convictions::{ConvictionStats, Convictions},
};

#[derive(Event)]
pub struct StatModifierEvent {
    pub target: Entity,
    pub conviction: ConvictionType,
    pub stat: ConvictionStats,
    pub modifier: ModifierType,
}

fn on_stat_modifier_event(
    trigger: On<StatModifierEvent>,
    mut query_target: Query<&mut Convictions>,
) {
    let Ok(mut convictions) = query_target.get_mut(trigger.target) else {
        return;
    };

    let stats = match trigger.conviction {
        ConvictionType::Aeternum => &mut convictions.aeternum,
        ConvictionType::Alligatio => &mut convictions.alligatio,
        ConvictionType::Ananke => &mut convictions.ananke,
        ConvictionType::Sekhem => &mut convictions.sekhem,
        ConvictionType::Ataraxia => &mut convictions.ataraxia,
    };

    match trigger.modifier {
        ModifierType::Buff => {
            stats.attack += trigger.stat.attack;
            stats.defense += trigger.stat.defense;
        }
        ModifierType::Debuff => {
            stats.attack = (stats.attack - trigger.stat.attack).max(0.0);
            stats.defense = (stats.defense - trigger.stat.defense).max(0.0);
        }
    }
}

pub struct StatModifierPlugin;

impl Plugin for StatModifierPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_stat_modifier_event);
    }
}
