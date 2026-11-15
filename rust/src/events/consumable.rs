use bevy::prelude::*;
use godot::global::godot_print;

use crate::{
    characters::{
        components::convictions::Convictions, inventories::items::ItemInventory,
        player_3d::Player3DNode,
    },
    events::{heal::HealEvent, upgrade_convictions::apply_conviction_upgrade},
    items::{
        definition::{ConsumableEffect, ItemId, ItemKind},
        registry::ItemRegistry,
    },
};

#[derive(Event, Debug, Clone)]
pub struct ConsumableEvent {
    pub item: ItemId,
}

fn on_consumable_event(
    trigger: On<ConsumableEvent>,
    item_registry: Res<ItemRegistry>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut Convictions), With<Player3DNode>>,
    mut item_inventory: ResMut<ItemInventory>,
) {
    let Some(definition) = item_registry.get(trigger.item) else {
        return;
    };

    let ItemKind::Consumable(effect) = definition.kind else {
        return;
    };

    let Ok((entity, mut convictions)) = query.single_mut() else {
        return;
    };

    if !item_inventory.decrease_one(trigger.item) {
        godot_print!("could not decrease item consumable quantity");
        return;
    }

    match effect {
        ConsumableEffect::Heal(value) => {
            commands.trigger(HealEvent {
                heal: Some(value),
                source: entity,
                target: entity,
            });
        }
        ConsumableEffect::UpgrdeConviction { conviction } => {
            apply_conviction_upgrade(&mut convictions, conviction);
        }
    }
}
pub struct ConsumablePlugin;

impl Plugin for ConsumablePlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_consumable_event);
    }
}
