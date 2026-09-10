mod components;
mod enemies;
mod players;
use bevy::prelude::*;

use players::PlayerPlugin;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(enemies::EnemiesPlugin);
    }
}
