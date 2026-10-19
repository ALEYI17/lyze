use bevy::prelude::*;

use crate::characters::inventories::abilities::AbilityPlugin;
pub mod abilities;

pub struct InventoryPlugin;

impl Plugin for InventoryPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(AbilityPlugin);
    }
}
