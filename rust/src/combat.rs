use bevy::prelude::*;
pub mod attack;
pub mod reaction;

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(attack::AttackPlugin)
            .add_plugins(reaction::ReactionPlugin);
    }
}
