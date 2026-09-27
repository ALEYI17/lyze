use bevy::prelude::*;
mod girl_npc;

use girl_npc::GirlNpcPlugin;

pub struct NpcPlugin;

impl Plugin for NpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(GirlNpcPlugin);
    }
}
