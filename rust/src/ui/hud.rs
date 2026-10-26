use crate::abilities::definition::{AbilityId, ConvictionType, format_ability_scaling};
use crate::abilities::registry::AbilityRegistry;
use crate::characters::components::convictions::Convictions;
use crate::characters::components::identity::Name;
use crate::characters::inventories::abilities::AbilityInventory;
use crate::characters::inventories::convictions::ConvictionPoints;
use crate::characters::player_3d::Player3DNode;
use crate::events::abilities::LearnAbilityEvent;
use crate::events::upgrade_convictions::UpgradeConvictionEvent;
use crate::godot_utils::nodes::get_custom_canvas_layer;
use bevy::prelude::*;
use godot::classes::{
    Button, CanvasLayer, GridContainer, Label, PanelContainer, ProgressBar, ResourceLoader,
    VBoxContainer,
};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::characters::components::stats::{Health, MaxHealth};
use crate::state::GameState;

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = CanvasLayer, class_name = MainHud)]
pub struct HudNode;

#[derive(Resource, Default)]
struct HudAssets {
    pub hud: Option<GodotNodeHandle>,
    pub menu_button: Option<GodotNodeHandle>,
    pub close_button: Option<GodotNodeHandle>,
    pub initialized: bool,
    pub signal_connected: bool,
    pub ability_list_initialized: bool,
    pub equiped_abilities_grid_initialized: bool,
    pub avaible_abilities_grid_initialized: bool,
    pub convictions_upgrade_buttons_connected: bool,
}

#[derive(NodeTreeView)]
struct HudUi {
    #[node("/root/main_isometric/Hud")]
    pub hud: GodotNodeHandle,
    #[node("/root/main_isometric/Hud/Control/MenuButton")]
    pub menu_button: GodotNodeHandle,
    #[node("/root/main_isometric/Hud/Control/CharacterMenu/VBoxContainer/Header/CloseButton")]
    pub close_button: GodotNodeHandle,
}

#[derive(Event, Debug, Clone)]
struct OpenChatacterMenuEvent;

#[derive(Event, Debug, Clone)]
struct LearnAbilitySelect {
    ability: AbilityId,
    avaible_points: u32,
}

#[derive(Event, Debug, Clone)]
struct UnequipAbilitySelect {
    ability: AbilityId,
}

#[derive(Event, Debug, Clone)]
pub struct EquipAbilitySelect {
    ability: AbilityId,
}

#[derive(Event, Debug, Clone)]
pub struct UpgradeConvictionButtonEvent {
    conviction: ConvictionType,
}

#[derive(Event)]
pub struct ReloadAbilityListEvent;

fn get_health_bar(canvas: &Gd<CanvasLayer>) -> Option<Gd<ProgressBar>> {
    let node = canvas.get_node_or_null("Control/PlayerStatus/VBoxContainer/HealthRow/HealthBar")?;

    let Ok(health_bar) = node.try_cast::<ProgressBar>() else {
        return None;
    };

    Some(health_bar)
}

fn get_player_health(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let node =
        canvas.get_node_or_null("Control/PlayerStatus/VBoxContainer/HealthRow/player_healt")?;

    let Ok(label) = node.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_player_name(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let node = canvas.get_node_or_null("Control/PlayerStatus/VBoxContainer/PlayerName")?;

    let Ok(label) = node.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_character_menu(canvas: &Gd<CanvasLayer>) -> Option<Gd<PanelContainer>> {
    let node = canvas.get_node_or_null("Control/CharacterMenu")?;

    let Ok(panel) = node.try_cast::<PanelContainer>() else {
        return None;
    };

    Some(panel)
}

fn get_conviction_labels(canvas: &Gd<CanvasLayer>) -> Option<[(Gd<Label>, Gd<Label>); 5]> {
    let grid_path = "Control/CharacterMenu/VBoxContainer/Tabs/Convictions/VBoxContainer/StatsGrid/";

    let aeternum_attack_handle = canvas.get_node_or_null(&format!("{grid_path}AeternumAttack"))?;
    let Ok(aeternum_attack) = aeternum_attack_handle.try_cast::<Label>() else {
        return None;
    };

    let aeternum_defense_handle =
        canvas.get_node_or_null(&format!("{grid_path}AeternumDefense"))?;
    let Ok(aeternum_defense) = aeternum_defense_handle.try_cast::<Label>() else {
        return None;
    };

    let alligatio_attack_handle =
        canvas.get_node_or_null(&format!("{grid_path}AlligatioAttack"))?;
    let Ok(alligatio_attack) = alligatio_attack_handle.try_cast::<Label>() else {
        return None;
    };

    let alligatio_defense_handle =
        canvas.get_node_or_null(&format!("{grid_path}AlligatioDefense"))?;
    let Ok(alligatio_defense) = alligatio_defense_handle.try_cast::<Label>() else {
        return None;
    };

    let ananke_attack_handle = canvas.get_node_or_null(&format!("{grid_path}AnankeAttack"))?;
    let Ok(ananke_attack) = ananke_attack_handle.try_cast::<Label>() else {
        return None;
    };

    let ananke_defense_handle = canvas.get_node_or_null(&format!("{grid_path}AnankeDefense"))?;
    let Ok(ananke_defense) = ananke_defense_handle.try_cast::<Label>() else {
        return None;
    };

    let sekhem_attack_handle = canvas.get_node_or_null(&format!("{grid_path}SekhemAttack"))?;
    let Ok(sekhem_attack) = sekhem_attack_handle.try_cast::<Label>() else {
        return None;
    };

    let sekhem_defense_handle = canvas.get_node_or_null(&format!("{grid_path}SekhemDefense"))?;
    let Ok(sekhem_defense) = sekhem_defense_handle.try_cast::<Label>() else {
        return None;
    };

    let ataraxia_attack_handle = canvas.get_node_or_null(&format!("{grid_path}AtaraxiaAttack"))?;
    let Ok(ataraxia_attack) = ataraxia_attack_handle.try_cast::<Label>() else {
        return None;
    };

    let ataraxia_defense_handle =
        canvas.get_node_or_null(&format!("{grid_path}AtaraxiaDefense"))?;
    let Ok(ataraxia_defense) = ataraxia_defense_handle.try_cast::<Label>() else {
        return None;
    };

    Some([
        (aeternum_attack, aeternum_defense),
        (alligatio_attack, alligatio_defense),
        (ananke_attack, ananke_defense),
        (sekhem_attack, sekhem_defense),
        (ataraxia_attack, ataraxia_defense),
    ])
}

fn get_available_conviction_points(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let path = concat!(
        "Control/CharacterMenu/VBoxContainer/Tabs/",
        "Convictions/VBoxContainer/Heading/AvailablePoints"
    );

    let node = canvas.get_node_or_null(path)?;
    let Ok(label) = node.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_conviction_upgrade_buttons(canvas: &Gd<CanvasLayer>) -> Option<[Gd<Button>; 5]> {
    let grid_path = "Control/CharacterMenu/VBoxContainer/Tabs/Convictions/VBoxContainer/StatsGrid/";

    let aeternum_handle = canvas.get_node_or_null(&format!("{grid_path}AeternumUpgrade"))?;
    let Ok(aeternum) = aeternum_handle.try_cast::<Button>() else {
        return None;
    };

    let alligatio_handle = canvas.get_node_or_null(&format!("{grid_path}AlligatioUpgrade"))?;
    let Ok(alligatio) = alligatio_handle.try_cast::<Button>() else {
        return None;
    };

    let ananke_handle = canvas.get_node_or_null(&format!("{grid_path}AnankeUpgrade"))?;
    let Ok(ananke) = ananke_handle.try_cast::<Button>() else {
        return None;
    };

    let sekhem_handle = canvas.get_node_or_null(&format!("{grid_path}SekhemUpgrade"))?;
    let Ok(sekhem) = sekhem_handle.try_cast::<Button>() else {
        return None;
    };

    let ataraxia_handle = canvas.get_node_or_null(&format!("{grid_path}AtaraxiaUpgrade"))?;
    let Ok(ataraxia) = ataraxia_handle.try_cast::<Button>() else {
        return None;
    };

    Some([aeternum, alligatio, ananke, sekhem, ataraxia])
}

fn get_ability_list(canvas: &Gd<CanvasLayer>) -> Option<Gd<VBoxContainer>> {
    let path = concat!(
        "Control/CharacterMenu/VBoxContainer/Tabs/",
        "Abilities/VBoxContainer/AbilityScroll/AbilityList"
    );

    let node = canvas.get_node_or_null(path)?;

    let Ok(container) = node.try_cast::<VBoxContainer>() else {
        return None;
    };

    Some(container)
}

fn get_equipped_grid(canvas: &Gd<CanvasLayer>) -> Option<Gd<GridContainer>> {
    let path = concat!(
        "Control/CharacterMenu/VBoxContainer/Tabs/",
        "Loadout/VBoxContainer/EquippedGrid"
    );

    let node = canvas.get_node_or_null(path)?;

    let Ok(container) = node.try_cast::<GridContainer>() else {
        return None;
    };

    Some(container)
}

fn get_available_grid(canvas: &Gd<CanvasLayer>) -> Option<Gd<GridContainer>> {
    let path = concat!(
        "Control/CharacterMenu/VBoxContainer/Tabs/",
        "Loadout/VBoxContainer/AvailableScroll/AvailableGrid"
    );

    let node = canvas.get_node_or_null(path)?;

    let Ok(container) = node.try_cast::<GridContainer>() else {
        return None;
    };

    Some(container)
}

fn populate_ability_list(
    ability_list: &mut Gd<VBoxContainer>,
    ability_registry: &AbilityRegistry,
    ability_inventory: &AbilityInventory,
    signal_learn: &GodotSignals<LearnAbilitySelect>,
) {
    let Some(mut points_label) = ability_list
        .get_parent()
        .and_then(|node| node.get_parent())
        .and_then(|node| node.get_node_or_null("Header/AvailablePoints"))
        .and_then(|node| node.try_cast::<Label>().ok())
    else {
        return;
    };

    let avaible_points = ability_inventory.avaible_points;

    points_label.set_text(&format!("Points: {}", avaible_points,));

    let mut loader = ResourceLoader::singleton();

    let Ok(scene) = loader
        .load("res://scenes/ui/components/AbilityRow.tscn")
        .ok_or(())
        .and_then(|resource| resource.try_cast::<PackedScene>().map_err(|_| ()))
    else {
        return;
    };

    for ability in ability_registry.abilities.values() {
        let Some(row) = scene.instantiate() else {
            continue;
        };

        let Some(mut name_label) = row
            .get_node_or_null("MarginContainer/VBoxContainer/Header/AbilityName")
            .and_then(|node| node.try_cast::<Label>().ok())
        else {
            continue;
        };

        name_label.set_text(ability.name);

        let Some(mut scaling_label) = row
            .get_node_or_null("MarginContainer/VBoxContainer/ScalingInfo")
            .and_then(|node| node.try_cast::<Label>().ok())
        else {
            continue;
        };

        scaling_label.set_text(&format_ability_scaling(&ability.effect));

        let Some(mut cost_label) = row
            .get_node_or_null("MarginContainer/VBoxContainer/AbilityCost")
            .and_then(|node| node.try_cast::<Label>().ok())
        else {
            continue;
        };

        cost_label.set_text(&format!("Cost: {} points", ability.point_cost));

        let Some(mut status_label) = row
            .get_node_or_null("MarginContainer/VBoxContainer/Header/LearnedStatus")
            .and_then(|node| node.try_cast::<Label>().ok())
        else {
            continue;
        };

        let Some(mut learn_button) = row
            .get_node_or_null("MarginContainer/VBoxContainer/Actions/LearnButton")
            .and_then(|node| node.try_cast::<Button>().ok())
        else {
            continue;
        };

        let ability_id = ability.id;

        if ability.learned {
            status_label.set_text("Learned");
            learn_button.set_text("Learned");
            learn_button.set_disabled(true);
        } else {
            status_label.set_text("Not learned");
            learn_button.set_text("Learn");
            learn_button.set_disabled(false);

            signal_learn.connect(
                learn_button.into(),
                BaseButtonSignals::PRESSED,
                None,
                move |_args, _node_handle, _ent| {
                    Some(LearnAbilitySelect {
                        ability: ability_id,
                        avaible_points,
                    })
                },
            );
        }

        ability_list.add_child(&row);
    }
}

fn populate_equipped_abilities(
    equipped_grid: &mut Gd<GridContainer>,
    ability_registry: &AbilityRegistry,
    ability_inventory: &AbilityInventory,
    signal_unequip: &GodotSignals<UnequipAbilitySelect>,
) {
    let mut loader = ResourceLoader::singleton();

    let Ok(scene) = loader
        .load("res://scenes/ui/components/AbilitySlot.tscn")
        .ok_or(())
        .and_then(|resource| resource.try_cast::<PackedScene>().map_err(|_| ()))
    else {
        return;
    };

    for ability_id in ability_inventory.abilities.iter().take(5) {
        let Some(ability) = ability_registry.get(*ability_id) else {
            continue;
        };

        let Some(slot) = scene.instantiate() else {
            continue;
        };

        let Some(mut name_label) = slot
            .get_node_or_null("MarginContainer/VBoxContainer/AbilityName")
            .and_then(|node| node.try_cast::<Label>().ok())
        else {
            continue;
        };

        name_label.set_text(ability.name);

        let Some(mut action_button) = slot
            .get_node_or_null("MarginContainer/VBoxContainer/ActionButton")
            .and_then(|node| node.try_cast::<Button>().ok())
        else {
            continue;
        };

        action_button.set_text("Unequip");

        let ability_id = *ability_id;

        signal_unequip.connect(
            action_button.into(),
            BaseButtonSignals::PRESSED,
            None,
            move |_args, _node_handle, _ent| {
                Some(UnequipAbilitySelect {
                    ability: ability_id,
                })
            },
        );

        equipped_grid.add_child(&slot);
    }
}

fn populate_available_abilities(
    available_grid: &mut Gd<GridContainer>,
    ability_registry: &AbilityRegistry,
    ability_inventory: &AbilityInventory,
    signal_equip: &GodotSignals<EquipAbilitySelect>,
) {
    let mut loader = ResourceLoader::singleton();

    let Ok(scene) = loader
        .load("res://scenes/ui/components/AbilitySlot.tscn")
        .ok_or(())
        .and_then(|resource| resource.try_cast::<PackedScene>().map_err(|_| ()))
    else {
        return;
    };

    for ability in ability_registry
        .abilities
        .values()
        .filter(|ability| ability.learned)
        .filter(|ability| !ability_inventory.abilities.contains(&ability.id))
    {
        godot_print!("find ability not equiped");
        let Some(slot) = scene.instantiate() else {
            godot_print!("dont find 1");
            continue;
        };

        let Some(mut name_label) = slot
            .get_node_or_null("MarginContainer/VBoxContainer/AbilityName")
            .and_then(|node| node.try_cast::<Label>().ok())
        else {
            godot_print!("dont find 2");
            continue;
        };

        name_label.set_text(ability.name);

        let Some(mut action_button) = slot
            .get_node_or_null("MarginContainer/VBoxContainer/ActionButton")
            .and_then(|node| node.try_cast::<Button>().ok())
        else {
            godot_print!("dont find 3");
            continue;
        };

        action_button.set_text("Equip");

        let ability_id = ability.id;

        signal_equip.connect(
            action_button.into(),
            BaseButtonSignals::PRESSED,
            None,
            move |_args, _node_handle, _ent| {
                Some(EquipAbilitySelect {
                    ability: ability_id,
                })
            },
        );

        available_grid.add_child(&slot);
    }
}

fn reset_hud_assets(mut hud_assets: ResMut<HudAssets>) {
    hud_assets.hud = None;
    hud_assets.menu_button = None;
    hud_assets.close_button = None;
    hud_assets.initialized = false;
    hud_assets.equiped_abilities_grid_initialized = false;
    hud_assets.ability_list_initialized = false;
    hud_assets.avaible_abilities_grid_initialized = false;
    hud_assets.convictions_upgrade_buttons_connected = false;
}

fn initialized_hud(mut hud_assets: ResMut<HudAssets>, mut scene_tree: SceneTreeRef) {
    if let Some(root) = scene_tree.get().get_root() {
        if let Ok(hud_ui) = HudUi::from_node(root) {
            godot_print!("Found hud");
            hud_assets.hud = Some(hud_ui.hud);
            hud_assets.menu_button = Some(hud_ui.menu_button);
            hud_assets.close_button = Some(hud_ui.close_button);
            hud_assets.initialized = true;
        }
    } else {
        godot_print!("Not found hud");
    }
}

fn connect_button(
    mut hud_assets: ResMut<HudAssets>,
    signal_open: GodotSignals<OpenChatacterMenuEvent>,
) {
    if hud_assets.menu_button.is_some()
        && !hud_assets.signal_connected
        && let Some(menu_button) = hud_assets.menu_button
        && let Some(close_button) = hud_assets.close_button
    {
        signal_open.connect(
            menu_button,
            BaseButtonSignals::PRESSED,
            None,
            |_args, _node_handle, _ent| Some(OpenChatacterMenuEvent),
        );

        signal_open.connect(
            close_button,
            BaseButtonSignals::PRESSED,
            None,
            |_args, _node_handle, _ent| Some(OpenChatacterMenuEvent),
        );
        godot_print!("Main hud connected");

        hud_assets.signal_connected = true;
    }
}

fn on_open_character_menu_event(
    _trigger: On<OpenChatacterMenuEvent>,
    mut godot: GodotAccess,
    hud_assets: Res<HudAssets>,
) {
    godot_print!("open event get");
    let Some(canvas_handle) = hud_assets.hud else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(&canvas_handle, &mut godot) else {
        return;
    };

    let Some(mut character_menu) = get_character_menu(&canvas) else {
        return;
    };

    let visibility = character_menu.is_visible();

    character_menu.set_visible(!visibility);
}

fn set_player_health(
    hud_assets: Res<HudAssets>,
    query: Query<(&Health, &MaxHealth, &Name), With<Player3DNode>>,
    mut godot: GodotAccess,
    convictions_points: Res<ConvictionPoints>,
) {
    let Ok((health, max_health, name)) = query.single() else {
        return;
    };

    let Some(hud_handle) = hud_assets.hud else {
        godot_print!("Can not find hud");
        return;
    };

    let Some(hud_node) = godot.try_get::<CanvasLayer>(hud_handle) else {
        godot_print!("Can not parse hud");
        return;
    };

    let Some(mut health_bar) = get_health_bar(&hud_node) else {
        return;
    };

    let Some(mut health_label) = get_player_health(&hud_node) else {
        return;
    };

    let Some(mut name_label) = get_player_name(&hud_node) else {
        return;
    };

    let Some(mut avaible_points) = get_available_conviction_points(&hud_node) else {
        return;
    };

    avaible_points.set_text(&convictions_points.avaible_points.to_string());

    health_bar.set_max(max_health.0.as_f64());
    health_bar.set_value(health.0.as_f64());

    let health_text = format!("{:.1}/{:.1}", health.0, max_health.0);
    health_label.set_text(&health_text);

    name_label.set_text(&name.0);
}

fn set_convictions_stats(
    hud_assets: Res<HudAssets>,
    query: Query<&Convictions, With<Player3DNode>>,
    mut godot: GodotAccess,
) {
    let Ok(convictions) = query.single() else {
        return;
    };

    let Some(hud_handle) = hud_assets.hud else {
        godot_print!("Can not find hud");
        return;
    };

    let Some(hud_node) = godot.try_get::<CanvasLayer>(hud_handle) else {
        godot_print!("Can not parse hud");
        return;
    };

    let Some(mut convictions_labels) = get_conviction_labels(&hud_node) else {
        return;
    };

    let conviction_stats = [
        &convictions.aeternum,
        &convictions.alligatio,
        &convictions.ananke,
        &convictions.sekhem,
        &convictions.ataraxia,
    ];

    for ((attack_label, defense_label), stats) in
        convictions_labels.iter_mut().zip(conviction_stats)
    {
        attack_label.set_text(&stats.attack.to_string());
        defense_label.set_text(&stats.defense.to_string());
    }
}

fn connect_convictions_buttons(
    mut hud_assets: ResMut<HudAssets>,
    mut godot: GodotAccess,
    signal_upgrade: GodotSignals<UpgradeConvictionButtonEvent>,
) {
    let Some(hud_handle) = hud_assets.hud else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(&hud_handle, &mut godot) else {
        return;
    };

    let Some(conviction_buttons) = get_conviction_upgrade_buttons(&canvas) else {
        return;
    };

    let convictions = [
        ConvictionType::Aeternum,
        ConvictionType::Alligatio,
        ConvictionType::Ananke,
        ConvictionType::Sekhem,
        ConvictionType::Ataraxia,
    ];

    for (button, conviction) in conviction_buttons.into_iter().zip(convictions) {
        signal_upgrade.connect(
            button.into(),
            BaseButtonSignals::PRESSED,
            None,
            move |_args, _node_handle, _ent| Some(UpgradeConvictionButtonEvent { conviction }),
        );
    }

    hud_assets.convictions_upgrade_buttons_connected = true;
}

fn set_ability_list(
    mut hud_assets: ResMut<HudAssets>,
    ability_registry: Res<AbilityRegistry>,
    ability_inventory: Res<AbilityInventory>,
    mut godot: GodotAccess,
    signal_learn: GodotSignals<LearnAbilitySelect>,
) {
    let Some(hud_handle) = hud_assets.hud else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(&hud_handle, &mut godot) else {
        return;
    };

    let Some(mut ability_list) = get_ability_list(&canvas) else {
        return;
    };

    populate_ability_list(
        &mut ability_list,
        &ability_registry,
        &ability_inventory,
        &signal_learn,
    );

    hud_assets.ability_list_initialized = true;
}

fn set_equiped_ability(
    mut hud_assets: ResMut<HudAssets>,
    ability_resgistry: Res<AbilityRegistry>,
    ability_inventory: Res<AbilityInventory>,
    mut godot: GodotAccess,
    signal_unequip: GodotSignals<UnequipAbilitySelect>,
) {
    let Some(hud_handle) = hud_assets.hud else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(&hud_handle, &mut godot) else {
        return;
    };

    let Some(mut equiped_grid) = get_equipped_grid(&canvas) else {
        return;
    };

    populate_equipped_abilities(
        &mut equiped_grid,
        &ability_resgistry,
        &ability_inventory,
        &signal_unequip,
    );

    hud_assets.equiped_abilities_grid_initialized = true;
}

fn set_avaible_abilities(
    mut hud_assets: ResMut<HudAssets>,
    ability_resgistry: Res<AbilityRegistry>,
    ability_inventory: Res<AbilityInventory>,
    mut godot: GodotAccess,
    signal_unequip: GodotSignals<EquipAbilitySelect>,
) {
    let Some(hud_handle) = hud_assets.hud else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(&hud_handle, &mut godot) else {
        return;
    };

    let Some(mut avaible_grid) = get_available_grid(&canvas) else {
        godot_print!("ERROR: AvailableGrid not found!");
        return;
    };

    populate_available_abilities(
        &mut avaible_grid,
        &ability_resgistry,
        &ability_inventory,
        &signal_unequip,
    );

    hud_assets.avaible_abilities_grid_initialized = true;
}

fn on_learn_ability_select(trigger: On<LearnAbilitySelect>, mut commands: Commands) {
    godot_print!("Learn Pressed");
    commands.trigger(LearnAbilityEvent {
        ability: trigger.ability,
        avaible_points: trigger.avaible_points,
    });
}

fn on_upgrade_conviction(
    trigger: On<UpgradeConvictionButtonEvent>,
    mut commands: Commands,
    conviction_points: Res<ConvictionPoints>,
) {
    godot_print!("Press upgrade in {:?}", trigger.conviction);

    if conviction_points.avaible_points == 0 {
        return;
    }
    commands.trigger(UpgradeConvictionEvent {
        conviction: trigger.conviction,
    });
}

#[allow(clippy::too_many_arguments)]
fn on_reload_ability(
    _trigger: On<ReloadAbilityListEvent>,
    hud_assets: Res<HudAssets>,
    ability_registry: Res<AbilityRegistry>,
    mut godot: GodotAccess,
    signal_learn: GodotSignals<LearnAbilitySelect>,
    signal_unequip: GodotSignals<UnequipAbilitySelect>,
    signal_equip: GodotSignals<EquipAbilitySelect>,
    ability_inventory: Res<AbilityInventory>,
) {
    let Some(hud_handle) = hud_assets.hud else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(&hud_handle, &mut godot) else {
        return;
    };

    let Some(mut ability_list) = get_ability_list(&canvas) else {
        return;
    };

    let Some(mut avaible_grid) = get_available_grid(&canvas) else {
        return;
    };

    let Some(mut equiped_grid) = get_equipped_grid(&canvas) else {
        return;
    };

    for mut child in ability_list.get_children().iter_shared() {
        ability_list.remove_child(&child);
        child.queue_free();
    }

    for mut child in avaible_grid.get_children().iter_shared() {
        avaible_grid.remove_child(&child);
        child.queue_free();
    }

    for mut child in equiped_grid.get_children().iter_shared() {
        equiped_grid.remove_child(&child);
        child.queue_free();
    }

    populate_ability_list(
        &mut ability_list,
        &ability_registry,
        &ability_inventory,
        &signal_learn,
    );
    populate_available_abilities(
        &mut avaible_grid,
        &ability_registry,
        &ability_inventory,
        &signal_equip,
    );

    populate_equipped_abilities(
        &mut equiped_grid,
        &ability_registry,
        &ability_inventory,
        &signal_unequip,
    );
}

fn on_unequip_ability_select(
    trigger: On<UnequipAbilitySelect>,
    mut ability_inventory: ResMut<AbilityInventory>,
    mut commands: Commands,
) {
    ability_inventory
        .abilities
        .retain(|&x| x != trigger.ability);

    commands.trigger(ReloadAbilityListEvent);
}

fn on_equip_ability_select(
    trigger: On<EquipAbilitySelect>,
    mut ability_inventory: ResMut<AbilityInventory>,
    mut commands: Commands,
) {
    if ability_inventory.abilities.len() < 5 {
        ability_inventory.abilities.push(trigger.ability);
    }

    commands.trigger(ReloadAbilityListEvent);
}

fn hud_is_not_initialized(hud_assets: Res<HudAssets>) -> bool {
    !hud_assets.initialized
}

fn hud_initialized_but_signals_not_connected(hud_assets: Res<HudAssets>) -> bool {
    hud_assets.initialized && !hud_assets.signal_connected
}

fn ability_list_not_initialized(hud_assets: Res<HudAssets>) -> bool {
    !hud_assets.ability_list_initialized
}

fn equiped_abilities_grid_not_initialized(hud_assets: Res<HudAssets>) -> bool {
    !hud_assets.equiped_abilities_grid_initialized
}

fn avaible_abilities_grid_not_initialized(hud_assets: Res<HudAssets>) -> bool {
    !hud_assets.avaible_abilities_grid_initialized
}

fn convictions_upgrade_not_connected(hud_assets: Res<HudAssets>) -> bool {
    !hud_assets.convictions_upgrade_buttons_connected
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HudAssets>()
            .add_systems(Update, set_player_health)
            .add_systems(Update, initialized_hud.run_if(hud_is_not_initialized))
            .add_systems(
                Update,
                connect_button.run_if(hud_initialized_but_signals_not_connected),
            )
            .add_systems(
                Update,
                set_convictions_stats.run_if(in_state(GameState::Ingame3D)),
            )
            .add_systems(
                Update,
                set_ability_list
                    .run_if(in_state(GameState::Ingame3D).and_then(ability_list_not_initialized)),
            )
            .add_systems(
                Update,
                set_equiped_ability.run_if(
                    in_state(GameState::Ingame3D).and_then(equiped_abilities_grid_not_initialized),
                ),
            )
            .add_systems(
                Update,
                set_avaible_abilities.run_if(
                    in_state(GameState::Ingame3D).and_then(avaible_abilities_grid_not_initialized),
                ),
            )
            .add_systems(
                Update,
                connect_convictions_buttons.run_if(
                    in_state(GameState::Ingame3D).and_then(convictions_upgrade_not_connected),
                ),
            )
            .add_systems(OnEnter(GameState::Ingame3D), reset_hud_assets)
            .add_observer(on_open_character_menu_event)
            .add_observer(on_learn_ability_select)
            .add_observer(on_unequip_ability_select)
            .add_observer(on_equip_ability_select)
            .add_observer(on_upgrade_conviction)
            .add_observer(on_reload_ability);
    }
}
