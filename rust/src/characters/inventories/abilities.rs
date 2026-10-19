use bevy::prelude::*;

use crate::abilities::{definition::AbilityId, registry::AbilityRegistry};

#[derive(Resource, Debug)]
pub struct AbilityInventory {
    pub abilities: Vec<AbilityId>,
}

impl Default for AbilityInventory {
    fn default() -> Self {
        let abilities: [AbilityId; 5] = [
            AbilityId::Slash,
            AbilityId::Kick,
            AbilityId::Heal,
            AbilityId::Vigor,
            AbilityId::Strike,
        ];
        Self {
            abilities: abilities.to_vec(),
        }
    }
}

pub struct AbilityPlugin;

impl Plugin for AbilityPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<AbilityInventory>()
            .init_resource::<AbilityRegistry>();
    }
}
