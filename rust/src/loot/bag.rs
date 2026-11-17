use std::collections::BTreeMap;

use bevy::prelude::*;
use godot::global::godot_print;
use rand::random_range;

use crate::{
    items::definition::ItemId,
    loot::definition::{LootEntry, LootReward},
    state::GameState,
};

#[derive(Resource)]
pub struct LootBag {
    pub remaining: BTreeMap<ItemId, LootEntry>,
    pub reset_timer: Timer,
}

fn initial_entries() -> BTreeMap<ItemId, LootEntry> {
    BTreeMap::from([
        (
            ItemId::Biomass,
            LootEntry {
                min_quantity: 2,
                max_quantity: 5,
                copies: 4,
            },
        ),
        (
            ItemId::RegenerativeSerum,
            LootEntry {
                min_quantity: 1,
                max_quantity: 2,
                copies: 3,
            },
        ),
        (
            ItemId::SekhemSerum,
            LootEntry {
                min_quantity: 1,
                max_quantity: 1,
                copies: 1,
            },
        ),
    ])
}

impl Default for LootBag {
    fn default() -> Self {
        Self {
            remaining: initial_entries(),
            reset_timer: Timer::from_seconds(300.0, TimerMode::Once),
        }
    }
}

impl LootBag {
    pub fn draw_one(&mut self) -> Option<LootReward> {
        let mut total = self.remaining.values().map(|entry| entry.copies).sum();
        if total == 0 {
            self.refill_bag();
            godot_print!("Refill on draw one");
            total = self.remaining.values().map(|entry| entry.copies).sum();
        }

        let mut roll = rand::random_range(0..total);

        godot_print!("Loot random: {}", roll);
        for (item_id, entry) in &mut self.remaining {
            if roll < entry.copies {
                let quantity = random_range(entry.min_quantity..=entry.max_quantity);

                entry.copies -= 1;

                return Some(LootReward {
                    item: *item_id,
                    ammount: quantity,
                });
            }

            roll -= entry.copies;
        }

        None
    }

    pub fn refill_bag(&mut self) {
        self.remaining = initial_entries();
        self.reset_timer.reset();
    }
}

fn reset_bag(mut bag: ResMut<LootBag>, time: Res<Time>) {
    bag.reset_timer.tick(time.delta());

    if bag.reset_timer.just_finished() {
        bag.refill_bag();
    }
}

pub struct LootBagPlugin;

impl Plugin for LootBagPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LootBag>().add_systems(
            Update,
            reset_bag.run_if(in_state(GameState::Ingame3D).or_eager(in_state(GameState::InCombat))),
        );
    }
}
