use bevy::prelude::*;

use crate::abilities::{definition::AbilityId, registry::AbilityRegistry};

#[derive(Resource, Debug)]
pub struct AbilityInventory {
    pub abilities: Vec<AbilityId>,
    pub avaible_points: u32,
}

impl Default for AbilityInventory {
    fn default() -> Self {
        let abilities: [AbilityId; 5] = [
            AbilityId::Slash,
            AbilityId::Kick,
            AbilityId::Heal,
            AbilityId::Fortify,
            AbilityId::Weaken,
        ];
        Self {
            abilities: abilities.to_vec(),
            avaible_points: 4,
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
