pub mod components;
mod enemies;
mod npc;
mod player_3d;
pub mod players;
use bevy::prelude::*;

use npc::NpcPlugin;
use player_3d::Player3DPlugin;
use players::PlayerPlugin;

pub struct CharactersPlugin;

impl Plugin for CharactersPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(PlayerPlugin)
            .add_plugins(enemies::EnemiesPlugin)
            .add_plugins(Player3DPlugin)
            .add_plugins(NpcPlugin);
    }
}
