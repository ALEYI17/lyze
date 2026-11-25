use bevy::prelude::*;
use godot::global::godot_print;

use crate::{
    characters::inventories::items::ItemInventory,
    loot::bag::LootBag,
    persistent::save::{OpenedChests, SaveChestEvent},
    ui::hud::{LootReceivedEvent, ReloadItemListEvent},
    world::chest::{ChestID, ChestInteractionNode, Opened, ProstheticChest},
};

#[derive(Event)]
pub struct ChestLootEvent {
    pub chest_entity: Entity,
}

fn on_chest_loot_event(
    trigger: On<ChestLootEvent>,
    mut bag: ResMut<LootBag>,
    mut item_inventory: ResMut<ItemInventory>,
    mut query: Query<(&mut Opened, &ProstheticChest, &ChestID), With<ChestInteractionNode>>,
    mut commands: Commands,
    opened_chests: Res<OpenedChests>,
) {
    let Ok((mut opened, is_prosthetic, chest_id)) = query.get_mut(trigger.chest_entity) else {
        return;
    };

    if opened.0 || opened_chests.0.contains(&chest_id.0) {
        opened.0 = true;
        return;
    }

    let reward_opt = if is_prosthetic.0 {
        bag.draw_one_prosthetic()
    } else {
        bag.draw_one()
    };

    let Some(reward) = reward_opt else {
        return;
    };

    godot_print!(
        "Chest reward : ammount:{}, item: {:?}",
        reward.ammount,
        reward.item
    );

    opened.0 = true;

    if is_prosthetic.0 {
        item_inventory.add_prosthetic(reward.item);
    } else {
        item_inventory.add_item(reward.item, reward.ammount);
    }

    commands.trigger(SaveChestEvent {
        chest_id: chest_id.0,
    });

    commands.trigger(LootReceivedEvent {
        item: reward.item,
        quantity: reward.ammount,
    });
    commands.trigger(ReloadItemListEvent);
}

pub struct ChestLootPlugin;

impl Plugin for ChestLootPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_chest_loot_event);
    }
}
