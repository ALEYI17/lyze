use crate::characters::components::convictions::Convictions;

#[derive(Debug, Clone)]
pub enum AbilityEffect {
    Damage {
        base: f32,
        scalings: Vec<ConvictionScaling>,
    },
    Heal {
        base: f32,
        scalings: Vec<ConvictionScaling>,
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
}

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash)]
pub enum AbilityId {
    Strike,
    Kick,
    Slash,
    Vigor,
    Heal,
}

fn calculate_scaling_bonus(convictions: &Convictions, scaling: &[ConvictionScaling]) -> f32 {
    scaling
        .iter()
        .map(|scaling| {
            let conviction = match scaling.conviction {
                ConvictionType::Ananke => &convictions.ananke,
                ConvictionType::Sekhem => &convictions.sekhem,
                ConvictionType::Aeternum => &convictions.aeternum,
                ConvictionType::Ataraxia => &convictions.ataraxia,
                ConvictionType::Alligatio => &convictions.alligatio,
            };
            conviction.attack * scaling.rate
        })
        .sum()
}

pub fn calculate_ability_value(
    ability: &AbilityDefinition,
    convictions: &Convictions,
) -> Option<f32> {
    match &ability.effect {
        AbilityEffect::Damage { base, scalings } => {
            let extra_damage = calculate_scaling_bonus(convictions, scalings);

            Some(base + extra_damage)
        }
        AbilityEffect::Heal { base, scalings } => {
            let extra_heal = calculate_scaling_bonus(convictions, scalings);

            Some(base + extra_heal)
        }
    }
}
