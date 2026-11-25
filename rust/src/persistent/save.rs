use std::collections::BTreeSet;

use bevy::prelude::*;
use godot::global::godot_print;
use godot_bevy::interop::{GodotAccess, GodotNodeHandle};

use crate::{
    abilities::registry::AbilityRegistry,
    characters::{
        components::{
            convictions::Convictions,
            identity::Name,
            progression::{Experience, Level},
            stats::{Health, MaxHealth},
        },
        inventories::{
            abilities::AbilityInventory, convictions::ConvictionPoints, items::ItemInventory,
        },
        player_3d::Player3DNode,
    },
    godot_utils::nodes::get_custom_character_body_3d,
    loot::bag::LootBag,
    persistent::dto::{
        PlayerAbility, PlayerConvictions, PlayerItems, PlayerState, SaveData, SavedPosition,
        save_to_file, write_save_data,
    },
    state::GameState,
    world::world_manager::WorldAssets,
};

#[derive(Resource, Default)]
pub struct OpenedChests(pub BTreeSet<u32>);

#[derive(Event, Debug)]
pub struct SaveChestEvent {
    pub chest_id: u32,
}

#[allow(clippy::too_many_arguments)]
fn save_current_game(
    query_player_state: &Query<
        (
            &Health,
            &MaxHealth,
            &Level,
            &Experience,
            &Convictions,
            &Name,
            &GodotNodeHandle,
        ),
        With<Player3DNode>,
    >,
    conviction_inventory: &ConvictionPoints,
    item_inventory: &ItemInventory,
    ability_inventory: &AbilityInventory,
    ability_registry: &AbilityRegistry,
    bag: &LootBag,
    world_assets: &WorldAssets,
    opened_chests: &OpenedChests,
    godot: &mut GodotAccess,
) {
    let Ok((health, max_health, level, current_experience, convictions, name, handle)) =
        query_player_state.single()
    else {
        godot_print!("Save query failed: expected one player");
        return;
    };

    let learned_abilities = ability_registry
        .abilities
        .values()
        .filter(|ability| ability.learned)
        .map(|ability| ability.id)
        .collect();

    let granted_prosthetics = bag
        .remaining_prosthetic
        .iter()
        .filter_map(|(key, entry)| entry.granted.then_some(*key))
        .collect();

    let Some(body) = get_custom_character_body_3d(handle, godot) else {
        return;
    };

    let position = body.get_global_position();
    let save_data = SaveData {
        version: 1,
        player_state: PlayerState {
            health: health.0,
            max_health: max_health.0,
            level: level.0,
            current_experience: current_experience.0,
            name: name.0.clone(),
        },
        player_convictions: PlayerConvictions {
            convictions: convictions.clone(),
            avaible_points: conviction_inventory.avaible_points,
        },
        player_item: PlayerItems {
            items: item_inventory.items.clone(),
            prosthetics: item_inventory.prosthetics.clone(),
            left_arm: item_inventory.left_arm,
            right_leg: item_inventory.right_leg,
            granted_prosthetics,
        },
        player_ability: PlayerAbility {
            learned_abilities,
            equiped_abilities: ability_inventory.abilities.clone(),
            avaible_points: ability_inventory.avaible_points,
        },
        player_position: SavedPosition {
            x: f64::from(position.x),
            y: f64::from(position.y),
            z: f64::from(position.z),
            scene_path: world_assets.level_scene_path.clone(),
        },
        open_chest: opened_chests.0.iter().copied().collect(),
    };

    match write_save_data(&save_data) {
        Ok(json) => match save_to_file(&json) {
            Ok(()) => godot_print!("Game saved: {}", json),
            Err(error) => godot_print!("Failed to write save file: {:?}", error),
        },
        Err(error) => godot_print!("Failed to serialize save data: {:?}", error),
    }
}

#[allow(clippy::too_many_arguments)]
fn save_data(
    query_player_state: Query<
        (
            &Health,
            &MaxHealth,
            &Level,
            &Experience,
            &Convictions,
            &Name,
            &GodotNodeHandle,
        ),
        With<Player3DNode>,
    >,
    conviction_inventory: Res<ConvictionPoints>,
    item_inventory: Res<ItemInventory>,
    ability_inventory: Res<AbilityInventory>,
    ability_registry: Res<AbilityRegistry>,
    bag: Res<LootBag>,
    world_assets: Res<WorldAssets>,
    opened_chests: Res<OpenedChests>,
    mut godot: GodotAccess,
) {
    save_current_game(
        &query_player_state,
        &conviction_inventory,
        &item_inventory,
        &ability_inventory,
        &ability_registry,
        &bag,
        &world_assets,
        &opened_chests,
        &mut godot,
    );
}

#[allow(clippy::too_many_arguments)]
fn on_save_chest_event(
    trigger: On<SaveChestEvent>,
    mut opened_chests: ResMut<OpenedChests>,
    query_player_state: Query<
        (
            &Health,
            &MaxHealth,
            &Level,
            &Experience,
            &Convictions,
            &Name,
            &GodotNodeHandle,
        ),
        With<Player3DNode>,
    >,
    conviction_inventory: Res<ConvictionPoints>,
    item_inventory: Res<ItemInventory>,
    ability_inventory: Res<AbilityInventory>,
    ability_registry: Res<AbilityRegistry>,
    bag: Res<LootBag>,
    world_assets: Res<WorldAssets>,
    mut godot: GodotAccess,
) {
    opened_chests.0.insert(trigger.chest_id);

    save_current_game(
        &query_player_state,
        &conviction_inventory,
        &item_inventory,
        &ability_inventory,
        &ability_registry,
        &bag,
        &world_assets,
        &opened_chests,
        &mut godot,
    );
}

pub struct SaveDataPlugin;

impl Plugin for SaveDataPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<OpenedChests>()
            .add_systems(OnEnter(GameState::PauseMenu), save_data)
            .add_observer(on_save_chest_event);
    }
}
