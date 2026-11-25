use bevy::{asset::LoadState, prelude::*};
use godot::{classes::class_macros::private::virtuals::ZipReader::Vector3, global::godot_print};
use godot_bevy::{
    interop::{GodotAccess, GodotNodeHandle},
    plugins::assets::GodotResource,
};

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
    persistent::{
        dto::{SaveLoadError, load_from_file},
        save::OpenedChests,
    },
    state::GameState,
    world::world_manager::{WorldAssets, WorldOperationMessage},
};

#[derive(Resource, Default)]
struct LoadComplete(bool);

struct PendingLevelRestore {
    scene: Handle<GodotResource>,
    scene_path: String,
    destination: Vector3,
}

#[derive(Resource, Default)]
struct PendingLevelRestoreResource(Option<PendingLevelRestore>);

#[allow(clippy::too_many_arguments)]
fn load_save_data(
    prev_state: Res<PreviousState<GameState>>,
    mut query_player_state: Query<
        (
            &mut Health,
            &mut MaxHealth,
            &mut Level,
            &mut Experience,
            &mut Convictions,
            &mut Name,
        ),
        With<Player3DNode>,
    >,
    mut conviction_inventory: ResMut<ConvictionPoints>,
    mut item_inventory: ResMut<ItemInventory>,
    mut ability_inventory: ResMut<AbilityInventory>,
    mut ability_registry: ResMut<AbilityRegistry>,
    mut complete: ResMut<LoadComplete>,
    mut bag: ResMut<LootBag>,
    asset_server: Res<AssetServer>,
    mut pending_level: ResMut<PendingLevelRestoreResource>,
    mut opened_chests: ResMut<OpenedChests>,
) {
    if *prev_state.get() != GameState::MainMenu || complete.0 {
        return;
    }

    let Ok((
        mut health,
        mut max_health,
        mut level,
        mut current_experience,
        mut convictions,
        mut name,
    )) = query_player_state.single_mut()
    else {
        return;
    };

    let save_data = match load_from_file() {
        Ok(save_data) => save_data,
        Err(SaveLoadError::Io(error)) => {
            godot_print!("Failed to read save file: {}", error);
            complete.0 = true;
            return;
        }
        Err(SaveLoadError::Json(error)) => {
            godot_print!("Failed to parse save file: {}", error);
            complete.0 = true;
            return;
        }
    };

    let scene_path = save_data
        .player_position
        .scene_path
        .clone()
        .unwrap_or_else(|| "res://scenes/levels/level_1.tscn".to_string());

    health.0 = save_data.player_state.health;
    max_health.0 = save_data.player_state.max_health;
    level.0 = save_data.player_state.level;
    current_experience.0 = save_data.player_state.current_experience;
    name.0 = save_data.player_state.name;
    conviction_inventory.avaible_points = save_data.player_convictions.avaible_points;
    *convictions = save_data.player_convictions.convictions;

    item_inventory.right_leg = save_data.player_item.right_leg;
    item_inventory.left_arm = save_data.player_item.left_arm;
    item_inventory.prosthetics = save_data.player_item.prosthetics;
    item_inventory.items = save_data.player_item.items;

    ability_inventory.avaible_points = save_data.player_ability.avaible_points;
    ability_inventory.abilities = save_data.player_ability.equiped_abilities;

    ability_registry
        .abilities
        .iter_mut()
        .for_each(|(key, value)| {
            let learned = save_data.player_ability.learned_abilities.contains(key);
            value.learned = learned;
            if learned {
                value.point_cost = 0;
            }
        });

    bag.remaining_prosthetic
        .iter_mut()
        .for_each(|(key, value)| {
            value.granted = save_data.player_item.granted_prosthetics.contains(key);
        });

    opened_chests.0 = save_data.open_chest.iter().copied().collect();

    let destination = Vector3 {
        x: save_data.player_position.x as f32,
        y: save_data.player_position.y as f32,
        z: save_data.player_position.z as f32,
    };

    pending_level.0 = Some(PendingLevelRestore {
        scene: asset_server.load(scene_path.clone()),
        scene_path,
        destination,
    });

    complete.0 = true;
}

fn restore_level_when_loaded(
    mut pending_level: ResMut<PendingLevelRestoreResource>,
    asset_server: Res<AssetServer>,
    world_assets: Res<WorldAssets>,
    player_query: Query<&GodotNodeHandle, With<Player3DNode>>,
    mut operations: MessageWriter<WorldOperationMessage>,
    mut godot: GodotAccess,
) {
    let Some(pending) = pending_level.0.as_ref() else {
        return;
    };

    if !world_assets.initialized || world_assets.level.is_none() {
        return;
    }

    let Ok(player_handle) = player_query.single() else {
        return;
    };

    if world_assets.level_scene_path.as_deref() == Some(pending.scene_path.as_str()) {
        if let Some(mut player) = get_custom_character_body_3d(player_handle, &mut godot) {
            player.set_global_position(pending.destination);
            pending_level.0 = None;
        }
        return;
    }

    match asset_server.get_load_state(pending.scene.id()) {
        Some(LoadState::Loaded) => {
            operations.write(WorldOperationMessage::change_level(
                pending.scene.clone(),
                pending.scene_path.clone(),
                pending.destination,
            ));
            pending_level.0 = None;
        }
        Some(LoadState::Failed(error)) => {
            godot_print!("Failed to load saved level: {:?}", error);
            pending_level.0 = None;
        }
        Some(LoadState::Loading) | None => {}
        _ => {}
    }
}

fn reset_load_state(
    mut complete: ResMut<LoadComplete>,
    mut pending_level: ResMut<PendingLevelRestoreResource>,
) {
    complete.0 = false;
    pending_level.0 = None;
}

fn load_is_not_complete(complete: Res<LoadComplete>) -> bool {
    !complete.0
}

pub struct LoadPlugin;

impl Plugin for LoadPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<LoadComplete>()
            .init_resource::<PendingLevelRestoreResource>()
            .add_systems(
                Update,
                load_save_data.run_if(in_state(GameState::Ingame3D).and_then(load_is_not_complete)),
            )
            .add_systems(
                Update,
                restore_level_when_loaded.run_if(in_state(GameState::Ingame3D)),
            )
            .add_systems(OnEnter(GameState::MainMenu), reset_load_state);
    }
}
