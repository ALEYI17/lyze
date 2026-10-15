use bevy::prelude::*;

pub mod combat;
pub mod damage;
pub mod effects;

pub struct EventPlugins;

impl Plugin for EventPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(damage::DamagePlugin)
            .add_plugins(combat::CombatPlugin)
            .add_plugins(effects::EffectPlugin);
    }
}
