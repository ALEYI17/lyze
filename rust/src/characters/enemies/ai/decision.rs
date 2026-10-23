use crate::abilities::definition::AbilityEffect;
use crate::abilities::definition::AbilityScaling;
use crate::abilities::definition::ConvictionScaling;
use crate::abilities::definition::ConvictionType;
use crate::characters::enemies::ai::context::CombatContext;
use crate::characters::enemies::ai::personality::EnemyPersonality;
use crate::characters::enemies::ai::personality::EnemyPersonalityStats;
use crate::combat::attack::AttackDefinition;
use crate::combat::attack::DodgeDirection;
use crate::combat::attack::ReactionType;

#[derive(Clone)]
pub struct EnemyCombo {
    pub attacks: Vec<AttackDefinition>,
}

struct ComboStatistics {
    num_of_attacks: usize,
    average_damage: f32,
    average_reaction_window: f32,
}

#[derive(Debug, Clone, Copy)]
pub struct NormalizedComboStatistics {
    pub num_of_attacks: f32,
    pub average_damage: f32,
    pub reaction_difficulty: f32,
    pub difficulty: f32,
}

pub fn enemy_combos() -> Vec<EnemyCombo> {
    vec![
        // Basic combos
        EnemyCombo {
            attacks: vec![
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 6.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Aeternum,
                                rate: 0.8,
                            },
                            secondary: vec![ConvictionScaling {
                                conviction: ConvictionType::Sekhem,
                                rate: 0.2,
                            }],
                        },
                    },
                    reaction_window: 1.1,
                    reaction: ReactionType::Parry,
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 3.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Alligatio,
                                rate: 0.6,
                            },
                            secondary: vec![],
                        },
                    },
                    reaction_window: 0.8,
                    reaction: ReactionType::Dodge(vec![DodgeDirection::Forward]),
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 3.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Ananke,
                                rate: 0.7,
                            },
                            secondary: vec![],
                        },
                    },
                    reaction_window: 0.7,
                    reaction: ReactionType::Dodge(vec![DodgeDirection::Backward]),
                },
            ],
        },
        EnemyCombo {
            attacks: vec![
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 4.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Sekhem,
                                rate: 0.7,
                            },
                            secondary: vec![],
                        },
                    },
                    reaction_window: 0.6,
                    reaction: ReactionType::Parry,
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 3.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Aeternum,
                                rate: 0.5,
                            },
                            secondary: vec![ConvictionScaling {
                                conviction: ConvictionType::Ataraxia,
                                rate: 0.3,
                            }],
                        },
                    },
                    reaction_window: 0.8,
                    reaction: ReactionType::Parry,
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 3.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Alligatio,
                                rate: 0.6,
                            },
                            secondary: vec![],
                        },
                    },
                    reaction_window: 0.5,
                    reaction: ReactionType::Dodge(vec![DodgeDirection::Right]),
                },
            ],
        },
        EnemyCombo {
            attacks: vec![
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 4.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Ananke,
                                rate: 0.8,
                            },
                            secondary: vec![],
                        },
                    },
                    reaction_window: 0.9,
                    reaction: ReactionType::Dodge(vec![DodgeDirection::Left]),
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 3.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Aeternum,
                                rate: 0.6,
                            },
                            secondary: vec![],
                        },
                    },
                    reaction_window: 0.8,
                    reaction: ReactionType::Parry,
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 3.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Ataraxia,
                                rate: 0.5,
                            },
                            secondary: vec![ConvictionScaling {
                                conviction: ConvictionType::Sekhem,
                                rate: 0.2,
                            }],
                        },
                    },
                    reaction_window: 1.0,
                    reaction: ReactionType::Dodge(vec![DodgeDirection::Backward]),
                },
            ],
        },
        // Heavy combos
        EnemyCombo {
            attacks: vec![
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 10.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Aeternum,
                                rate: 1.2,
                            },
                            secondary: vec![ConvictionScaling {
                                conviction: ConvictionType::Sekhem,
                                rate: 0.5,
                            }],
                        },
                    },
                    reaction_window: 1.0,
                    reaction: ReactionType::Parry,
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 7.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Ananke,
                                rate: 1.0,
                            },
                            secondary: vec![ConvictionScaling {
                                conviction: ConvictionType::Alligatio,
                                rate: 0.3,
                            }],
                        },
                    },
                    reaction_window: 0.7,
                    reaction: ReactionType::Dodge(vec![DodgeDirection::Left]),
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 8.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Sekhem,
                                rate: 1.1,
                            },
                            secondary: vec![ConvictionScaling {
                                conviction: ConvictionType::Aeternum,
                                rate: 0.4,
                            }],
                        },
                    },
                    reaction_window: 0.6,
                    reaction: ReactionType::Parry,
                },
            ],
        },
        EnemyCombo {
            attacks: vec![
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 8.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Ananke,
                                rate: 1.0,
                            },
                            secondary: vec![ConvictionScaling {
                                conviction: ConvictionType::Ataraxia,
                                rate: 0.4,
                            }],
                        },
                    },
                    reaction_window: 0.8,
                    reaction: ReactionType::Dodge(vec![DodgeDirection::Backward]),
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 10.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Aeternum,
                                rate: 1.3,
                            },
                            secondary: vec![ConvictionScaling {
                                conviction: ConvictionType::Sekhem,
                                rate: 0.5,
                            }],
                        },
                    },
                    reaction_window: 0.7,
                    reaction: ReactionType::Parry,
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 6.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Alligatio,
                                rate: 0.9,
                            },
                            secondary: vec![],
                        },
                    },
                    reaction_window: 0.5,
                    reaction: ReactionType::Dodge(vec![DodgeDirection::Right]),
                },
                AttackDefinition {
                    effect: AbilityEffect::Damage {
                        base: 12.0,
                        scalings: AbilityScaling {
                            primary: ConvictionScaling {
                                conviction: ConvictionType::Sekhem,
                                rate: 1.4,
                            },
                            secondary: vec![
                                ConvictionScaling {
                                    conviction: ConvictionType::Aeternum,
                                    rate: 0.6,
                                },
                                ConvictionScaling {
                                    conviction: ConvictionType::Ananke,
                                    rate: 0.3,
                                },
                            ],
                        },
                    },
                    reaction_window: 1.0,
                    reaction: ReactionType::Parry,
                },
            ],
        },
    ]
}

fn calculate_stadistics(combos: &EnemyCombo) -> Option<ComboStatistics> {
    let num_of_attacks = combos.attacks.len();

    if num_of_attacks == 0 {
        return None;
    }

    let (total_damage, total_reaction_window) =
        combos
            .attacks
            .iter()
            .fold((0.0_f32, 0.0_f32), |(damage, reaction_window), attack| {
                (
                    damage + attack.effect.base().unwrap_or(0.0),
                    reaction_window + attack.reaction_window,
                )
            });

    let attack_count = num_of_attacks as f32;

    Some(ComboStatistics {
        num_of_attacks,
        average_damage: total_damage / attack_count,
        average_reaction_window: total_reaction_window / attack_count,
    })
}

fn normalize_value(value: f32, min: f32, max: f32) -> f32 {
    if max <= min {
        return 0.5;
    }

    (value - min) / (max - min)
}

fn calculate_difficulty(stats: &NormalizedComboStatistics) -> f32 {
    (stats.num_of_attacks * 0.40)
        + (stats.reaction_difficulty * 0.45)
        + (stats.average_damage * 0.15)
}

fn normalize_statistics(statistics: &[ComboStatistics]) -> Vec<NormalizedComboStatistics> {
    let Some(first) = statistics.first() else {
        return Vec::new();
    };

    let (min_attacks, max_attacks) = statistics.iter().fold(
        (first.num_of_attacks as f32, first.num_of_attacks as f32),
        |(min, max), stats| {
            let attacks = stats.num_of_attacks as f32;
            (min.min(attacks), max.max(attacks))
        },
    );

    let (min_damage, max_damage) = statistics.iter().fold(
        (first.average_damage, first.average_damage),
        |(min, max), stats| (min.min(stats.average_damage), max.max(stats.average_damage)),
    );

    let (min_reaction, max_reaction) = statistics.iter().fold(
        (first.average_reaction_window, first.average_reaction_window),
        |(min, max), stats| {
            (
                min.min(stats.average_reaction_window),
                max.max(stats.average_reaction_window),
            )
        },
    );

    statistics
        .iter()
        .map(|stats| {
            let num_of_attacks =
                normalize_value(stats.num_of_attacks as f32, min_attacks, max_attacks);

            let average_damage = normalize_value(stats.average_damage, min_damage, max_damage);

            let reaction_difficulty =
                1.0 - normalize_value(stats.average_reaction_window, min_reaction, max_reaction);

            let mut normalized = NormalizedComboStatistics {
                num_of_attacks,
                average_damage,
                reaction_difficulty,
                difficulty: 0.0,
            };

            normalized.difficulty = calculate_difficulty(&normalized);

            normalized
        })
        .collect()
}

pub fn calculate_difficulty_preference(
    stats: EnemyPersonalityStats,
    context: CombatContext,
) -> f32 {
    let hp_factor = 1.0 - context.hp_ratio;
    let aggression = stats.aggression / 5.0;
    let defense = stats.defense / 5.0;
    let preference = (aggression * hp_factor) + (defense * context.hp_ratio);

    preference.clamp(0.0, 1.0)
}

pub fn calculate_utility(difficulty: f32, difficulty_preference: f32) -> f32 {
    (difficulty_preference * difficulty) + ((1.0 - difficulty_preference) * (1.0 - difficulty))
}

pub fn attack_decision(
    combos: &[EnemyCombo],
    personality: EnemyPersonality,
    context: CombatContext,
    random_value: f32,
) -> Option<usize> {
    if combos.is_empty() || !(0.0..1.0).contains(&random_value) {
        return None;
    }

    if matches!(personality, EnemyPersonality::Crazy) {
        let selected = random_value * combos.len() as f32;

        return combos
            .iter()
            .enumerate()
            .find(|(index, _)| (*index as f32) <= selected)
            .map(|(index, _)| index);
    }

    let statistics: Vec<ComboStatistics> = combos
        .iter()
        .map(calculate_stadistics)
        .collect::<Option<Vec<_>>>()?;

    let normalized = normalize_statistics(&statistics);

    let personality_stats = personality.into();

    let preference = calculate_difficulty_preference(personality_stats, context);

    let weights: Vec<f32> = normalized
        .iter()
        .map(|stats| calculate_utility(stats.difficulty, preference).max(0.05))
        .collect();

    let total_weight: f32 = weights.iter().sum();

    if total_weight <= 0.0 || !total_weight.is_finite() {
        return None;
    }

    let target = random_value * total_weight;

    let mut cumulative_weight = 0.0;

    weights
        .iter()
        .enumerate()
        .find_map(|(index, weight)| {
            cumulative_weight += weight;

            (target < cumulative_weight).then_some(index)
        })
        .or_else(|| weights.len().checked_sub(1))
}

pub fn get_attack(
    personality: EnemyPersonality,
    context: CombatContext,
    random_value: f32,
) -> Option<Vec<AttackDefinition>> {
    let combos = enemy_combos();

    let combo_index = attack_decision(&combos, personality, context, random_value)?;

    let combo = combos.get(combo_index)?;

    Some(combo.attacks.clone())
}
