use bevy::prelude::*;

use crate::characters::inventories::{abilities::AbilityPlugin, convictions::ConvictionPlugin};
pub mod abilities;
pub mod convictions;
pub mod items;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AbilityPlugin)
            .add_plugins(ConvictionPlugin)
            .add_plugins(items::ItemPlugin);
    }
}
