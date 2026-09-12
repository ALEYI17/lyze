use bevy::prelude::*;

pub mod damage;

pub struct EventPlugins;

impl Plugin for EventPlugins  {
    fn build(&self, app: &mut App) {
        app.add_plugins(damage::DamagePlugin);
    }
}
