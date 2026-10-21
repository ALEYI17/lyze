use std::collections::HashMap;

use bevy::prelude::*;

use crate::abilities::definition::AbilityScaling;

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
    pub fn is_learned(&self, id: AbilityId) -> Option<bool> {
        let ability = self.abilities.get(&id)?;

        Some(ability.learned)
    }
}

impl Default for AbilityRegistry {
    fn default() -> Self {
        let definitions = [
            AbilityDefinition {
                id: AbilityId::Strike,
                name: "Strike",
                learned: true,
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
                learned: true,
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
                effect: AbilityEffect::Heal {
                    base: 15.0,
                    scalings: AbilityScaling {
                        primary: ConvictionScaling {
                            conviction: ConvictionType::Alligatio,
                            rate: 1.2,
                        },
                        secondary: Vec::new(),
                    },
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
