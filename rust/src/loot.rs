use bevy::prelude::*;
pub mod bag;
pub mod definition;

pub struct LootPlugin;

impl Plugin for LootPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(bag::LootBagPlugin);
    }
}
