use bevy::prelude::*;

pub mod abilities;
pub mod combat;
pub mod damage;
pub mod effects;
pub mod heal;
pub mod stat_modifier;
pub mod upgrade_convictions;

pub struct EventPlugins;

impl Plugin for EventPlugins {
    fn build(&self, app: &mut App) {
        app.add_plugins(damage::DamagePlugin)
            .add_plugins(combat::CombatPlugin)
            .add_plugins(effects::EffectPlugin)
            .add_plugins(abilities::AbilitiesPlugin)
            .add_plugins(heal::HealPluign)
            .add_plugins(stat_modifier::StatModifierPlugin)
            .add_plugins(upgrade_convictions::UpgradeConvictionPlugin);
    }
}
