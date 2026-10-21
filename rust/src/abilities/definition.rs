use crate::characters::components::convictions::Convictions;

#[derive(Debug, Clone)]
pub struct AbilityScaling {
    pub primary: ConvictionScaling,
    pub secondary: Vec<ConvictionScaling>,
}

#[derive(Debug, Clone)]
pub enum AbilityEffect {
    Damage { base: f32, scalings: AbilityScaling },
    Heal { base: f32, scalings: AbilityScaling },
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbilityId {
    Strike,
    Kick,
    Slash,
    Vigor,
    Heal,
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

    calculate_conviction(&scaling.primary)
        + scaling
            .secondary
            .iter()
            .map(calculate_conviction)
            .sum::<f32>()
}

pub fn calculate_ability_value(
    ability: &AbilityDefinition,
    source_convictions: &Convictions,
    target_convictions: &Convictions,
) -> Option<f32> {
    let k = 20.0;
    match &ability.effect {
        AbilityEffect::Damage { base, scalings } => {
            let raw_damage = base + calculate_scaling_bonus(source_convictions, scalings);
            let defense = calculate_defense(target_convictions, scalings);

            let final_damage = raw_damage * (k / (k + defense));
            Some(final_damage.max(0.0))
        }
        AbilityEffect::Heal { base, scalings } => {
            let extra_heal = calculate_scaling_bonus(source_convictions, scalings);

            Some(base + extra_heal)
        }
    }
}
