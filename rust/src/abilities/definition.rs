use godot::global::godot_print;

use crate::characters::components::convictions::{ConvictionStats, Convictions};

#[derive(Debug, Clone)]
pub struct AbilityScaling {
    pub primary: ConvictionScaling,
    pub secondary: Vec<ConvictionScaling>,
}

#[derive(Debug, Clone)]
pub enum ModifierType {
    Buff,
    Debuff,
}

#[derive(Debug, Clone)]
pub enum AbilityEffect {
    Damage {
        base: f32,
        scalings: AbilityScaling,
    },
    Heal {
        base: f32,
        scalings: AbilityScaling,
    },
    StatModifier {
        conviction: ConvictionType,
        stat: ConvictionStats,
        modifier: ModifierType,
    },
}

#[derive(Debug, Copy, Clone, PartialEq, Eq)]
pub enum ConvictionType {
    Aeternum,
    Alligatio,
    Ananke,
    Sekhem,
    Ataraxia,
}

#[derive(Debug, Copy, Clone)]
pub struct ConvictionScaling {
    pub conviction: ConvictionType,
    pub rate: f32,
}

pub struct AbilityDefinition {
    pub id: AbilityId,
    pub name: &'static str,
    pub effect: AbilityEffect,
    pub learned: bool,
    pub point_cost: u32,
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbilityId {
    Strike,
    Kick,
    Slash,
    Vigor,
    Heal,
    Fortify,
    Weaken,
}

impl AbilityEffect {
    pub fn base(&self) -> Option<f32> {
        match self {
            Self::Damage { base, .. } | Self::Heal { base, .. } => Some(*base),
            Self::StatModifier { .. } => None,
        }
    }
}

fn format_conviction_scaling(scaling: &ConvictionScaling) -> String {
    let conviction = match scaling.conviction {
        ConvictionType::Aeternum => "Aeternum",
        ConvictionType::Alligatio => "Alligatio",
        ConvictionType::Ananke => "Ananke",
        ConvictionType::Sekhem => "Sekhem",
        ConvictionType::Ataraxia => "Ataraxia",
    };

    format!("{conviction} × {:.1}", scaling.rate)
}

pub fn format_ability_scaling(effect: &AbilityEffect) -> String {
    let (base, scalings, effect_type) = match effect {
        AbilityEffect::Damage { base, scalings } => (base, scalings, "Damage"),
        AbilityEffect::Heal { base, scalings } => (base, scalings, "Healing"),

        AbilityEffect::StatModifier {
            conviction,
            stat,
            modifier,
        } => {
            let effect_type = match modifier {
                ModifierType::Buff => "Buff",
                ModifierType::Debuff => "Debuf",
            };

            let sign = match modifier {
                ModifierType::Buff => "+",
                ModifierType::Debuff => "-",
            };

            let mut changes = Vec::new();

            if stat.attack != 0.0 {
                changes.push(format!("ATK {sign}{:.1}", stat.attack));
            }

            if stat.defense != 0.0 {
                changes.push(format!("DEF {sign}{:.1}", stat.defense));
            }

            return format!("{effect_type} ({conviction:?}): {}", changes.join(" · "));
        }
    };

    let primary = format_conviction_scaling(&scalings.primary);

    let secondary = scalings
        .secondary
        .iter()
        .map(format_conviction_scaling)
        .collect::<Vec<_>>()
        .join(" · ");

    let scaling_text = if secondary.is_empty() {
        primary
    } else {
        format!("{primary} · {secondary}")
    };

    format!("{effect_type}: {base:.1} · {scaling_text}")
}

fn calculate_scaling_bonus(convictions: &Convictions, scaling: &AbilityScaling) -> f32 {
    let calculate_conviction = |scaling: &ConvictionScaling| {
        let conviction = match scaling.conviction {
            ConvictionType::Ananke => &convictions.ananke,
            ConvictionType::Sekhem => &convictions.sekhem,
            ConvictionType::Aeternum => &convictions.aeternum,
            ConvictionType::Ataraxia => &convictions.ataraxia,
            ConvictionType::Alligatio => &convictions.alligatio,
        };

        conviction.attack * scaling.rate
    };

    calculate_conviction(&scaling.primary)
        + scaling
            .secondary
            .iter()
            .map(calculate_conviction)
            .sum::<f32>()
}

fn calculate_defense(convictions: &Convictions, scaling: &AbilityScaling) -> f32 {
    let calculate_conviction = |scaling: &ConvictionScaling| {
        let conviction = match scaling.conviction {
            ConvictionType::Ananke => &convictions.ananke,
            ConvictionType::Sekhem => &convictions.sekhem,
            ConvictionType::Aeternum => &convictions.aeternum,
            ConvictionType::Ataraxia => &convictions.ataraxia,
            ConvictionType::Alligatio => &convictions.alligatio,
        };

        conviction.defense * scaling.rate
    };

    let defense = calculate_conviction(&scaling.primary)
        + scaling
            .secondary
            .iter()
            .map(calculate_conviction)
            .sum::<f32>();

    defense.max(0.0)
}

pub fn calculate_effect_value(
    effect: &AbilityEffect,
    source_convictions: &Convictions,
    target_convictions: &Convictions,
) -> Option<f32> {
    let k = 20.0;
    match &effect {
        AbilityEffect::Damage { base, scalings } => {
            let scaling_bonus = calculate_scaling_bonus(source_convictions, scalings);
            let raw_damage = base + calculate_scaling_bonus(source_convictions, scalings);
            let defense = calculate_defense(target_convictions, scalings);
            let multiplier = k / (k + defense);

            let final_damage = (raw_damage * multiplier).max(0.0);
            godot_print!(
                "[Damage] Base: {base:.2}, Scaling: {scaling_bonus:.2}, \
                 Raw: {raw_damage:.2}, Defense: {defense:.2}, \
                 Multiplier: {multiplier:.4}, Final: {final_damage:.2}"
            );
            Some(final_damage.max(0.0))
        }
        AbilityEffect::Heal { base, scalings } => {
            let scaling_bonus = calculate_scaling_bonus(source_convictions, scalings);
            let final_heal = base + scaling_bonus;

            godot_print!(
                "[Healing] Base: {base:.2}, Scaling: {scaling_bonus:.2}, \
                 Final: {final_heal:.2}"
            );
            Some(final_heal)
        }
        AbilityEffect::StatModifier { .. } => None,
    }
}
