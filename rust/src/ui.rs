mod main_menu;
mod pause_menu;
mod hud;

use bevy::prelude::*;

pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(main_menu::MainMenuPlugin)
            .add_plugins(pause_menu::PauseMenuPlugin)
            .add_plugins(hud::HudPlugin);
    }
}
