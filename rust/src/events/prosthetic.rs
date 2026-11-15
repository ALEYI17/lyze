use bevy::prelude::*;
use godot::global::godot_print;

use crate::{
    characters::{
        components::convictions::Convictions, inventories::items::ItemInventory,
        player_3d::Player3DNode,
    },
    items::{definition::{ItemId, ItemKind, ProstheticSlot}, registry::ItemRegistry}, ui::hud::ReloadItemListEvent,
};

#[derive(Event, Debug, Clone)]
pub struct EquipProstheticEvent {
    pub item: ItemId,
}

#[derive(Event, Debug, Clone)]
pub struct UnEquipProstheticEvent {
    pub slot: ProstheticSlot,
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
        let Some(level) = item_inventory.get_prosthetic_level(item_definition.id, &item_registry) else {
            return;
        };

        let slot = match definition.slot {
            ProstheticSlot::LeftArm => &mut item_inventory.left_arm,
            ProstheticSlot::RightLeg => &mut item_inventory.right_leg,
        };

        if slot.is_some(){
            return;
        }

        *slot = Some(item_definition.id);
        
        for bonus in &level.bonuses{
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
    mut query: Query<&mut Convictions, With<Player3DNode>>
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

    let Some(level) = item_inventory.get_prosthetic_level(item_definition.id, &item_registry) else {
        return;
    };
    
    for bonus in &level.bonuses{
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

pub struct ProstheticPlugin;

impl Plugin for ProstheticPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_equip_prosthetic)
            .add_observer(on_unequip_prosthetic);
    }
}
