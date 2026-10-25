use crate::abilities::definition::AbilityDefinition;
use crate::abilities::registry::AbilityRegistry;
use crate::characters::components::convictions::Convictions;
use crate::characters::components::identity::Name;
use crate::characters::player_3d::Player3DNode;
use crate::godot_utils::nodes::get_custom_canvas_layer;
use bevy::prelude::*;
use godot::classes::{CanvasLayer, ItemList, Label, PanelContainer, ProgressBar};
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

fn get_ability_list(canvas: &Gd<CanvasLayer>) -> Option<Gd<ItemList>> {
    let path = "Control/CharacterMenu/VBoxContainer/Tabs/Abilities/VBoxContainer/AbilityList";

    let node = canvas.get_node_or_null(path)?;

    let Ok(ability_list) = node.try_cast::<ItemList>() else {
        return None;
    };

    Some(ability_list)
}

fn reset_hud_assets(mut hud_assets: ResMut<HudAssets>) {
    hud_assets.hud = None;
    hud_assets.menu_button = None;
    hud_assets.close_button = None;
    hud_assets.initialized = false;
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

fn set_ability_list(
    mut hud_assets: ResMut<HudAssets>,
    ability_registry: Res<AbilityRegistry>,
    mut godot: GodotAccess,
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

    let learned_abilities: Vec<&AbilityDefinition> = ability_registry
        .abilities
        .values()
        .filter(|v| v.learned)
        .collect();

    for ability in learned_abilities {
        ability_list.add_item(ability.name);
    }

    hud_assets.ability_list_initialized = true;
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
            .add_systems(OnEnter(GameState::Ingame3D), reset_hud_assets)
            .add_observer(on_open_character_menu_event);
    }
}
