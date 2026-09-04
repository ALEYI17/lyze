mod main_menu;
mod pause_menu;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(main_menu::MainMenuPlugin)
            .add_plugins(pause_menu::PauseMenuPlugin);
    }
}
