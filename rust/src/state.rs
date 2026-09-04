use bevy::prelude::*;

pub use game_state::GameState;
mod game_state;
mod scene_manager;
pub mod state_manager;

pub struct StatePlugin;

impl Plugin for StatePlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(game_state::GameStatePlugin)
            .add_plugins(scene_manager::SceneManagerPlugin)
            .add_plugins(state_manager::StateManagerPlugin);
    }
}
