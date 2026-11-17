use bevy::prelude::*;
use godot::global::godot_print;

use crate::{
    characters::{
        components::convictions::Convictions, inventories::items::ItemInventory,
        player_3d::Player3DNode,
    },
    items::{
        definition::{ItemId, ItemKind, ProstheticSlot},
        registry::ItemRegistry,
    },
    ui::hud::ReloadItemListEvent,
};

#[derive(Event, Debug, Clone)]
pub struct EquipProstheticEvent {
    pub item: ItemId,
}

#[derive(Event, Debug, Clone)]
pub struct UnEquipProstheticEvent {
    pub slot: ProstheticSlot,
}

#[derive(Event, Debug, Clone)]
pub struct UpgradeProsthetic {
    pub item: ItemId,
}

fn on_equip_prosthetic(
    trigger: On<EquipProstheticEvent>,
    mut item_inventory: ResMut<ItemInventory>,
    item_registry: Res<ItemRegistry>,
    mut commands: Commands,
    mut query: Query<&mut Convictions, With<Player3DNode>>,
) {
    godot_print!("Get equip item event");
    let Some(item_definition) = item_registry.get(trigger.item) else {
        return;
    };

    let Ok(mut convictions) = query.single_mut() else {
        return;
    };

    if let ItemKind::Prosthetic(definition) = &item_definition.kind {
        let Some(level) = item_inventory.get_prosthetic_level(item_definition.id, &item_registry)
        else {
            return;
        };

        let slot = match definition.slot {
            ProstheticSlot::LeftArm => &mut item_inventory.left_arm,
            ProstheticSlot::RightLeg => &mut item_inventory.right_leg,
        };

        if slot.is_some() {
            return;
        }

        *slot = Some(item_definition.id);

        for bonus in &level.bonuses {
            let stats = convictions.get_mut(bonus.conviction);
            stats.attack += bonus.attack;
            stats.defense += bonus.defense;
        }

        commands.trigger(ReloadItemListEvent);
    }
}

fn on_unequip_prosthetic(
    trigger: On<UnEquipProstheticEvent>,
    mut item_inventory: ResMut<ItemInventory>,
    item_registry: Res<ItemRegistry>,
    mut commands: Commands,
    mut query: Query<&mut Convictions, With<Player3DNode>>,
) {
    godot_print!("Get unequip item event");

    let equipped_item = match trigger.slot {
        ProstheticSlot::LeftArm => item_inventory.left_arm,
        ProstheticSlot::RightLeg => item_inventory.right_leg,
    };

    let Some(item_id) = equipped_item else {
        return;
    };
    let Ok(mut convictions) = query.single_mut() else {
        return;
    };

    let Some(item_definition) = item_registry.get(item_id) else {
        return;
    };

    let Some(level) = item_inventory.get_prosthetic_level(item_definition.id, &item_registry)
    else {
        return;
    };

    for bonus in &level.bonuses {
        let stats = convictions.get_mut(bonus.conviction);
        stats.attack -= bonus.attack;
        stats.defense -= bonus.defense;
    }

    match trigger.slot {
        ProstheticSlot::LeftArm => item_inventory.left_arm = None,
        ProstheticSlot::RightLeg => item_inventory.right_leg = None,
    }

    commands.trigger(ReloadItemListEvent);
}

fn on_upgrade_prosthetic(
    trigger: On<UpgradeProsthetic>,
    mut item_inventory: ResMut<ItemInventory>,
    item_registry: Res<ItemRegistry>,
    mut commands: Commands,
    mut query: Query<&mut Convictions, With<Player3DNode>>,
) {
    godot_print!("Get event UpgradeProsthetic");
    if !item_inventory.check_prosthetic_upgrade(trigger.item, &item_registry) {
        return;
    }

    let Some(current_level) = item_inventory.prosthetics.get(&trigger.item) else {
        return;
    };

    let Some(next_level) = current_level.checked_add(1) else {
        return;
    };

    let Some(item) = item_registry.get(trigger.item) else {
        return;
    };
    let ItemKind::Prosthetic(definition) = &item.kind else {
        return;
    };
    let Some(current) = definition
        .levels
        .iter()
        .find(|level| level.level == *current_level)
    else {
        return;
    };
    let Some(next) = definition
        .levels
        .iter()
        .find(|level| level.level == next_level)
    else {
        return;
    };

    let equipped = item_inventory.left_arm == Some(trigger.item)
        || item_inventory.right_leg == Some(trigger.item);
    let mut convictions = if equipped {
        let Ok(convictions) = query.single_mut() else {
            return;
        };
        Some(convictions)
    } else {
        None
    };

    for upgrade in &next.upgrades {
        for requirement in &upgrade.requirements {
            if requirement.quantity == 0 {
                continue;
            }
            if let Some(quantity) = item_inventory.items.get_mut(&requirement.material) {
                *quantity -= requirement.quantity;
                if *quantity == 0 {
                    item_inventory.items.remove(&requirement.material);
                }
            }
        }
    }

    if let Some(convictions) = convictions.as_mut() {
        for bonus in &current.bonuses {
            let stats = convictions.get_mut(bonus.conviction);
            stats.attack -= bonus.attack;
            stats.defense -= bonus.defense;
        }
        for bonus in &next.bonuses {
            let stats = convictions.get_mut(bonus.conviction);
            stats.attack += bonus.attack;
            stats.defense += bonus.defense;
        }
    }

    item_inventory.prosthetics.insert(trigger.item, next_level);

    commands.trigger(ReloadItemListEvent);
}

pub struct ProstheticPlugin;

impl Plugin for ProstheticPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_equip_prosthetic)
            .add_observer(on_unequip_prosthetic)
            .add_observer(on_upgrade_prosthetic);
    }
}
