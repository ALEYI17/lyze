use bevy::prelude::*;
pub mod dto;
pub mod load;
pub mod save;

pub struct PersistentPlugin;

impl Plugin for PersistentPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(save::SaveDataPlugin)
            .add_plugins(load::LoadPlugin);
    }
}
