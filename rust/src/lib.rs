use bevy::{prelude::*, state::app::StatesPlugin};
use godot::prelude::*;
use godot_bevy::prelude::*;

mod characters;
mod state;
mod ui;

use crate::characters::CharactersPlugin;

#[bevy_app]
fn build_app(app: &mut App) {
    app.add_plugins(GodotAssetsPlugin)
        .add_plugins(StatesPlugin)
        .add_plugins(state::StatePlugin)
        .add_plugins(CharactersPlugin)
        .add_plugins(ui::UiPlugin);
}
