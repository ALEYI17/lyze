use bevy::prelude::*;
use std::collections::BTreeMap;

use crate::items::{definition::{ItemId, ItemKind, ProstheticLevels}, registry::ItemRegistry};

#[derive(Resource, Debug)]
pub struct ItemInventory {
    pub items: BTreeMap<ItemId, u32>,
    pub prosthetics: BTreeMap<ItemId, u32>,
    pub left_arm: Option<ItemId>,
    pub right_leg: Option<ItemId>,
}

impl Default for ItemInventory {
    fn default() -> Self {
        let items: BTreeMap<ItemId, u32> = BTreeMap::from([
            (ItemId::SekhemSerum, 1),
            (ItemId::RegenerativeSerum, 3),
            (ItemId::Biomass, 1),
        ]);
        let prosthetics: BTreeMap<ItemId, u32> = BTreeMap::from([
            (ItemId::MyofiberArm, 1),
            (ItemId::NeurostrideLeg, 1),
        ]);

        Self {
            items,
            left_arm: None,
            right_leg: None,
            prosthetics,
        }
    }
}

impl ItemInventory {
    pub fn decrease_one(&mut self, item: ItemId) -> bool {
        let Some(item_quantity) = self.items.get_mut(&item) else {
            return false;
        };

        if *item_quantity == 0 {
            return false;
        }

        *item_quantity -= 1;

        if *item_quantity == 0 {
            self.items.remove(&item);
        }

        true
    }
    pub fn get_prosthetic_level(&self, item: ItemId, item_registry: &ItemRegistry) -> Option<ProstheticLevels>{
        let current_level = self.prosthetics.get(&item)?; 

        let item_def = item_registry.get(item)?;

        let ItemKind::Prosthetic(definition) = &item_def.kind else {
            return None;
        };

        definition.levels.iter().find(|entry| entry.level == *current_level).cloned()
    }
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemInventory>()
            .init_resource::<ItemRegistry>();
    }
}
