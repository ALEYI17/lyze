use bevy::prelude::*;
pub mod chest;
pub mod save_point;
mod transport_node;
pub mod world_manager;

pub struct WorldPlugin;

impl Plugin for WorldPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(transport_node::LevelTransitionPlugin)
            .add_plugins(world_manager::WorldNodesPlugin)
            .add_plugins(chest::ChestNodePlugin)
            .add_plugins(save_point::SavePointPlugin);
    }
}
