use bevy::prelude::*;

pub mod combat;
pub mod damage;

pub struct EventPlugins;

impl Plugin for EventPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(damage::DamagePlugin)
            .add_plugins(combat::CombatPlugin);
    }
}
