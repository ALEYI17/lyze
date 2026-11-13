use bevy::prelude::*;

use crate::items::{definition::ItemId, registry::ItemRegistry};

#[derive(Resource, Debug)]
pub struct ItemInventory {
    pub items: Vec<ItemId>,
    pub left_arm: Option<ItemId>,
    pub right_leg: Option<ItemId>,
}

impl Default for ItemInventory {
    fn default() -> Self {
        let items: Vec<ItemId> = vec![
            ItemId::RegenerativeSerum,
            ItemId::MyofiberArm,
            ItemId::Biomass,
        ];

        Self {
            items,
            left_arm: None,
            right_leg: None,
        }
    }
}

pub struct ItemPlugin;

impl Plugin for ItemPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ItemInventory>()
            .init_resource::<ItemRegistry>();
    }
}
