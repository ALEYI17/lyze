use std::collections::HashMap;

use bevy::prelude::*;

use crate::{
    abilities::definition::{AbilityScaling, ModifierType},
    characters::components::convictions::ConvictionStats,
};

use super::definition::{
    AbilityDefinition, AbilityEffect, AbilityId, ConvictionScaling, ConvictionType,
};

#[derive(Resource)]
pub struct AbilityRegistry {
    pub abilities: HashMap<AbilityId, AbilityDefinition>,
}

impl AbilityRegistry {
    pub fn get(&self, id: AbilityId) -> Option<&AbilityDefinition> {
        self.abilities.get(&id)
    }
    pub fn get_mut(&mut self, id: AbilityId) -> Option<&mut AbilityDefinition> {
        self.abilities.get_mut(&id)
    }
}

impl Default for AbilityRegistry {
    fn default() -> Self {
        let definitions = [
            AbilityDefinition {
                id: AbilityId::Strike,
                name: "Strike",
                learned: true,
                point_cost: 0,
                effect: AbilityEffect::Damage {
                    base: 10.0,
                    scalings: AbilityScaling {
                        primary: ConvictionScaling {
                            conviction: ConvictionType::Ataraxia,
                            rate: 1.2,
                        },
                        secondary: Vec::new(),
                    },
                },
            },
            AbilityDefinition {
                id: AbilityId::Kick,
                name: "Kick",
                learned: true,
                point_cost: 0,
                effect: AbilityEffect::Damage {
                    base: 12.0,
                    scalings: AbilityScaling {
                        primary: ConvictionScaling {
                            conviction: ConvictionType::Ananke,
                            rate: 1.3,
                        },
                        secondary: Vec::new(),
                    },
                },
            },
            AbilityDefinition {
                id: AbilityId::Slash,
                name: "Slash",
                learned: true,
                point_cost: 0,
                effect: AbilityEffect::Damage {
                    base: 15.0,
                    scalings: AbilityScaling {
                        primary: ConvictionScaling {
                            conviction: ConvictionType::Sekhem,
                            rate: 1.5,
                        },
                        secondary: Vec::new(),
                    },
                },
            },
            AbilityDefinition {
                id: AbilityId::Vigor,
                name: "Vigor",
                learned: false,
                point_cost: 2,
                effect: AbilityEffect::Heal {
                    base: 8.0,
                    scalings: AbilityScaling {
                        primary: ConvictionScaling {
                            conviction: ConvictionType::Aeternum,
                            rate: 1.7,
                        },
                        secondary: Vec::new(),
                    },
                },
            },
            AbilityDefinition {
                id: AbilityId::Heal,
                name: "Heal",
                learned: true,
                point_cost: 0,
                effect: AbilityEffect::Heal {
                    base: 8.0,
                    scalings: AbilityScaling {
                        primary: ConvictionScaling {
                            conviction: ConvictionType::Alligatio,
                            rate: 1.2,
                        },
                        secondary: Vec::new(),
                    },
                },
            },
            AbilityDefinition {
                id: AbilityId::Weaken,
                name: "Weaken",
                learned: true,
                point_cost: 0,
                effect: AbilityEffect::StatModifier {
                    conviction: ConvictionType::Sekhem,
                    stat: ConvictionStats {
                        attack: 4.0,
                        defense: 5.0,
                    },
                    modifier: ModifierType::Debuff,
                },
            },
            AbilityDefinition {
                id: AbilityId::Fortify,
                name: "Fortify",
                learned: true,
                point_cost: 0,
                effect: AbilityEffect::StatModifier {
                    conviction: ConvictionType::Aeternum,
                    stat: ConvictionStats {
                        attack: 3.0,
                        defense: 5.0,
                    },
                    modifier: ModifierType::Buff,
                },
            },
        ];

        Self {
            abilities: definitions
                .into_iter()
                .map(|ability| (ability.id, ability))
                .collect(),
        }
    }
}
