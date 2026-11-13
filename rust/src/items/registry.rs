use std::collections::HashMap;

use bevy::prelude::Resource;

use crate::{
    abilities::definition::ConvictionType,
    items::definition::{
        ConsumableEffect, ConvictionsBonus, ItemDefinition, ItemId, ItemKind, ProstheticDefinition,
        ProstheticSlot, ProstheticUpgrade, UpgradeRequirements,
    },
};

#[derive(Resource)]
pub struct ItemRegistry {
    pub items: HashMap<ItemId, ItemDefinition>,
}

impl ItemRegistry {
    pub fn get(&self, id: ItemId) -> Option<&ItemDefinition> {
        self.items.get(&id)
    }
}

impl Default for ItemRegistry {
    fn default() -> Self {
        let definitions = [
            ItemDefinition {
                id: ItemId::RegenerativeSerum,
                name: "Regenerative Serum",
                description: "A bioengineered serum that accelerates tissue regeneration and restores health.",
                kind: ItemKind::Consumable(ConsumableEffect::Heal(5.0)),
            },
            ItemDefinition {
                id: ItemId::MyofiberArm,
                name: "Myofiber Arm",
                description: "A prosthetic arm woven from synthetic muscle fibers, enhancing physical strength through biological augmentation.",
                kind: ItemKind::Prosthetic(ProstheticDefinition {
                    slot: ProstheticSlot::LeftArm,
                    bonuses: vec![ConvictionsBonus {
                        conviction: ConvictionType::Sekhem,
                        attack: 3.0,
                        defense: 1.0,
                    }],
                    upgrades: vec![
                        ProstheticUpgrade {
                            requirements: vec![UpgradeRequirements {
                                material: ItemId::Biomass,
                                quantity: 3,
                            }],
                            bonuses: vec![ConvictionsBonus {
                                conviction: ConvictionType::Sekhem,
                                attack: 5.0,
                                defense: 2.0,
                            }],
                        },
                        ProstheticUpgrade {
                            requirements: vec![UpgradeRequirements {
                                material: ItemId::Biomass,
                                quantity: 6,
                            }],
                            bonuses: vec![ConvictionsBonus {
                                conviction: ConvictionType::Sekhem,
                                attack: 8.0,
                                defense: 3.0,
                            }],
                        },
                    ],
                    max_level: 3,
                }),
            },
            ItemDefinition {
                id: ItemId::Biomass,
                name: "Biomass",
                description: "Processed organic matter used to grow and enhance biological prosthetics.",
                kind: ItemKind::Materials,
            },
        ];

        Self {
            items: definitions
                .into_iter()
                .map(|item| (item.id, item))
                .collect(),
        }
    }
}
