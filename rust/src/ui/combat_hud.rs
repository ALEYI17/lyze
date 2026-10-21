use bevy::prelude::*;
use godot::classes::{Button, CanvasLayer, Control, Label, PanelContainer, ProgressBar};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::abilities::definition::{AbilityId, format_ability_scaling};
use crate::abilities::registry::AbilityRegistry;
use crate::characters::components::convictions::Convictions;
use crate::characters::components::identity::Name;
use crate::characters::components::state::Alive;
use crate::characters::components::stats::{Damage, Health, MaxHealth};
use crate::characters::inventories::abilities::AbilityInventory;
use crate::combat::attack::FinishAttackEvent;
use crate::combat::reaction::{ReactionWindow, StartReactionEvent};
use crate::events::abilities::AbilityEvent;
use crate::events::combat::{CombatTarget, NextTurn, TurnTransition, is_player_turn};
use crate::godot_utils::nodes::get_custom_canvas_layer;
use crate::{events::combat::CombatResource, state::GameState};

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = CanvasLayer, class_name = CombatHud)]
pub struct CombatHudNode;

fn get_turn_label(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let label_handle = canvas.get_node_or_null("turn")?;

    let Ok(label) = label_handle.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_control(canvas: &Gd<CanvasLayer>) -> Option<Gd<Control>> {
    let label_handle = canvas.get_node_or_null("Control")?;

    let Ok(control) = label_handle.try_cast::<Control>() else {
        return None;
    };

    Some(control)
}

fn get_attack_button(button: &GodotNodeHandle, godot: &mut GodotAccess) -> Option<Gd<Button>> {
    godot.try_get::<Button>(*button)
}

fn get_reaction_label(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let label_handle = canvas.get_node_or_null("ReactionPrompt/ReactionContent/ReactionLabel")?;

    let Ok(label) = label_handle.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_reaction_prompt(canvas: &Gd<CanvasLayer>) -> Option<Gd<PanelContainer>> {
    let label_handle = canvas.get_node_or_null("ReactionPrompt")?;

    let Ok(panel) = label_handle.try_cast::<PanelContainer>() else {
        return None;
    };

    Some(panel)
}

fn get_reaction_timer(canvas: &Gd<CanvasLayer>) -> Option<Gd<ProgressBar>> {
    let label_handle = canvas.get_node_or_null("ReactionPrompt/ReactionContent/ReactionTimer")?;

    let Ok(bar) = label_handle.try_cast::<ProgressBar>() else {
        return None;
    };

    Some(bar)
}

fn get_ability_tooltip(canvas: &Gd<CanvasLayer>) -> Option<Gd<PanelContainer>> {
    let tooltip_handle = canvas.get_node_or_null("Control/AbilityTooltip")?;

    let Ok(tooltip) = tooltip_handle.try_cast::<PanelContainer>() else {
        return None;
    };

    Some(tooltip)
}

fn get_ability_name(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let node = canvas
        .get_node_or_null("Control/AbilityTooltip/MarginContainer/VBoxContainer/AbilityName")?;

    let Ok(label) = node.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_ability_scaling(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let label_handle = canvas
        .get_node_or_null("Control/AbilityTooltip/MarginContainer/VBoxContainer/AbilityScaling")?;

    let Ok(label) = label_handle.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_target_name(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let label_handle = canvas.get_node_or_null("TargetInfo/Content/TargetName")?;

    let Ok(label) = label_handle.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_health_label(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let label_handle = canvas.get_node_or_null("TargetInfo/Content/HealthLabel")?;

    let Ok(label) = label_handle.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_health_bar(canvas: &Gd<CanvasLayer>) -> Option<Gd<ProgressBar>> {
    let bar_handle = canvas.get_node_or_null("TargetInfo/Content/HealthBar")?;

    let Ok(bar) = bar_handle.try_cast::<ProgressBar>() else {
        return None;
    };

    Some(bar)
}

fn get_damage_label(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let label_handle = canvas.get_node_or_null("TargetInfo/Content/DamageLabel")?;

    let Ok(label) = label_handle.try_cast::<Label>() else {
        return None;
    };

    Some(label)
}

fn get_target_info(canvas: &Gd<CanvasLayer>) -> Option<Gd<PanelContainer>> {
    let target_info_handle = canvas.get_node_or_null("TargetInfo")?;

    let Ok(target_info) = target_info_handle.try_cast::<PanelContainer>() else {
        return None;
    };

    Some(target_info)
}

fn get_conviction_labels(canvas: &Gd<CanvasLayer>) -> Option<[(Gd<Label>, Gd<Label>); 5]> {
    let grid_path = "TargetInfo/Content/ConvictionGrid/";

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

#[derive(Resource, Default)]
pub struct CombatHudAssets {
    pub combat_hud: Option<GodotNodeHandle>,
    pub attack_button: Option<GodotNodeHandle>,
    pub attack_button_b: Option<GodotNodeHandle>,
    pub attack_button_c: Option<GodotNodeHandle>,
    pub attack_button_d: Option<GodotNodeHandle>,
    pub attack_button_e: Option<GodotNodeHandle>,
    pub initialized: bool,
    pub signal_connected: bool,
    pub menu_entered: bool,
}

#[derive(NodeTreeView)]
pub struct CombatHUdUi {
    #[node("/root/main_isometric/CombatHud")]
    pub combat_hud: GodotNodeHandle,

    #[node("/root/main_isometric/CombatHud/Control/attack_1")]
    pub attack_button: GodotNodeHandle,

    #[node("/root/main_isometric/CombatHud/Control/attack_2")]
    pub attack_button_b: GodotNodeHandle,

    #[node("/root/main_isometric/CombatHud/Control/attack_3")]
    pub attack_button_c: GodotNodeHandle,

    #[node("/root/main_isometric/CombatHud/Control/attack_4")]
    pub attack_button_d: GodotNodeHandle,

    #[node("/root/main_isometric/CombatHud/Control/attack_5")]
    pub attack_button_e: GodotNodeHandle,
}

#[derive(Event, Debug, Clone)]
struct AttackEvent {
    pub ability_id: AbilityId,
}

#[derive(Event, Debug, Clone)]
struct AbilityHover {
    pub ability_id: AbilityId,
}

#[derive(Event, Debug, Clone)]
struct AbilityUnHover;

fn reset_combat_assets(mut combat_assets: ResMut<CombatHudAssets>) {
    combat_assets.combat_hud = None;
    combat_assets.attack_button = None;
    combat_assets.attack_button_b = None;
    combat_assets.attack_button_c = None;
    combat_assets.attack_button_d = None;
    combat_assets.attack_button_e = None;
    combat_assets.initialized = false;
    combat_assets.signal_connected = false;
    combat_assets.menu_entered = false;
}

fn initialized_combat_hud(
    mut combat_assets: ResMut<CombatHudAssets>,
    mut scene_tree: SceneTreeRef,
) {
    if let Some(root) = scene_tree.get().get_root() {
        if let Ok(menu_ui) = CombatHUdUi::from_node(root) {
            combat_assets.combat_hud = Some(menu_ui.combat_hud);
            combat_assets.attack_button = Some(menu_ui.attack_button);
            combat_assets.attack_button_b = Some(menu_ui.attack_button_b);
            combat_assets.attack_button_c = Some(menu_ui.attack_button_c);
            combat_assets.attack_button_d = Some(menu_ui.attack_button_d);
            combat_assets.attack_button_e = Some(menu_ui.attack_button_e);
            combat_assets.initialized = true;
        }
    } else {
        godot_print!("Main Menu scene not avaible");
    }
}

fn connect_button(
    mut combat_assets: ResMut<CombatHudAssets>,
    signals_attack: GodotSignals<AttackEvent>,
    signals_hover: GodotSignals<AbilityHover>,
    signals_unhover: GodotSignals<AbilityUnHover>,
    ability_inventory: Res<AbilityInventory>,
    ability_registry: Res<AbilityRegistry>,
    mut godot: GodotAccess,
) {
    if combat_assets.signal_connected {
        return;
    }

    let buttons = [
        combat_assets.attack_button,
        combat_assets.attack_button_b,
        combat_assets.attack_button_c,
        combat_assets.attack_button_d,
        combat_assets.attack_button_e,
    ];

    for (index, button) in buttons.into_iter().enumerate() {
        let Some(button_handle) = button else {
            continue;
        };

        let Some(mut button) = get_attack_button(&button_handle, &mut godot) else {
            return;
        };

        let Some(ability_id) = ability_inventory.abilities.get(index).copied() else {
            continue;
        };

        let Some(ability) = ability_registry.get(ability_id) else {
            return;
        };

        button.set_text(ability.name);

        signals_attack.connect(
            button_handle,
            BaseButtonSignals::PRESSED,
            None,
            move |_args, _node_handle, _ent| Some(AttackEvent { ability_id }),
        );

        signals_unhover.connect(
            button_handle,
            ControlSignals::MOUSE_EXITED,
            None,
            move |_args, _node_handle, _ent| Some(AbilityUnHover),
        );

        signals_hover.connect(
            button_handle,
            ControlSignals::MOUSE_ENTERED,
            None,
            move |_args, _node_handle, _ent| Some(AbilityHover { ability_id }),
        );
    }

    combat_assets.signal_connected = true;

    // if combat_assets.attack_button.is_some() && !combat_assets.signal_connected {
    //     if let Some(attack_handle) = combat_assets.attack_button {
    //         signals_attack.connect(
    //             attack_handle,
    //             BaseButtonSignals::PRESSED,
    //             None,
    //             |_args, _node_handle, _ent| Some(AttackEvent),
    //         );
    //     }
    //
    //     combat_assets.signal_connected = true;
    // }
}

fn on_attack_button(
    trigger: On<AttackEvent>,
    combat_resource: ResMut<CombatResource>,
    mut commands: Commands,
    _alive_query: Query<&Alive>,
    target: Res<CombatTarget>,
) {
    let Some(current_entity) = combat_resource.current_entity() else {
        return;
    };

    let Some(enemy) = target.target else {
        return;
    };

    let Some(player) = combat_resource.player else {
        return;
    };

    if player != current_entity {
        godot_print!("Not your turn");
        return;
    }

    commands.trigger(AbilityEvent {
        ability: trigger.ability_id,
        source: player,
        target: enemy,
    });

    // commands.trigger(DamageEvent {
    //     source: player,
    //     target: enemy,
    //     damage: None,
    // });

    commands.trigger(NextTurn);
}

fn on_hover_enter(
    trigger: On<AbilityHover>,
    ability_registry: Res<AbilityRegistry>,
    mut godot: GodotAccess,
    combat: Res<CombatHudAssets>,
) {
    let Some(handle) = combat.combat_hud else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(&handle, &mut godot) else {
        return;
    };

    let Some(ability) = ability_registry.get(trigger.ability_id) else {
        return;
    };

    let Some(mut ability_tooltip) = get_ability_tooltip(&canvas) else {
        return;
    };

    let Some(mut name_label) = get_ability_name(&canvas) else {
        return;
    };

    let Some(mut scaling_label) = get_ability_scaling(&canvas) else {
        return;
    };

    ability_tooltip.set_visible(true);

    name_label.set_text(ability.name);

    let scaling_text = format_ability_scaling(&ability.effect);

    scaling_label.set_text(&scaling_text);
}

fn on_hover_exit(
    _trigger: On<AbilityUnHover>,
    mut godot: GodotAccess,
    combat: Res<CombatHudAssets>,
) {
    let Some(handle) = combat.combat_hud else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(&handle, &mut godot) else {
        return;
    };

    let Some(mut ability_tooltip) = get_ability_tooltip(&canvas) else {
        return;
    };

    ability_tooltip.set_visible(false);
}

fn enter_combat_hud(
    mut combat_resource: ResMut<CombatHudAssets>,
    mut godot: GodotAccess,
    query_hud: Query<&GodotNodeHandle, With<CombatHudNode>>,
) {
    let Ok(hud_handle) = query_hud.single() else {
        return;
    };

    let Some(mut hud) = get_custom_canvas_layer(hud_handle, &mut godot) else {
        return;
    };

    hud.set_visible(true);

    combat_resource.menu_entered = true;
}

fn toogle_combat_hud_visibility(
    mut godot: GodotAccess,
    query_hud: Query<&GodotNodeHandle, With<CombatHudNode>>,
    combat: Res<CombatResource>,
) {
    let Some(current_entity) = combat.current_entity() else {
        return;
    };

    let Some(player) = combat.player else {
        return;
    };

    let Ok(hud_handle) = query_hud.single() else {
        return;
    };

    let Some(hud) = get_custom_canvas_layer(hud_handle, &mut godot) else {
        return;
    };

    let Some(mut control) = get_control(&hud) else {
        return;
    };

    if player != current_entity {
        control.set_visible(false);
    } else {
        control.set_visible(true);
    }
}

fn update_target_information(
    mut godot: GodotAccess,
    query: Query<&GodotNodeHandle, With<CombatHudNode>>,
    query_components: Query<(&Convictions, &Health, &MaxHealth, &Damage, &Name)>,
    combat_target: Res<CombatTarget>,
    combat_resource: Res<CombatResource>,
) {
    
    let Some(current_entity) = combat_resource.current_entity() else {
        return;
    };
    let Some(player) = combat_resource.player else {
        return;
    };

    let Ok(handle) = query.single() else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(handle, &mut godot) else {
        return;
    };

    let Some(mut target_info) = get_target_info(&canvas)else {
        return;
    };

    if player != current_entity{
        target_info.set_visible(false);
        return;
    }

    let Some(target_entity) = combat_target.target else {
        return;
    };

    let Ok((convictions, healt, max_health, damage, name)) = query_components.get(target_entity) else {
        return;
    };

    let Some(mut health_label) = get_health_label(&canvas) else {
        return;
    };

    let Some(mut health_bar) = get_health_bar(&canvas) else {
        return;
    };

    let Some(mut damage_label) = get_damage_label(&canvas) else {
        return;
    };

    let Some(mut convictions_labels) = get_conviction_labels(&canvas) else {
        return;
    };

    let Some(mut name_label) = get_target_name(&canvas)else {
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

    target_info.set_visible(true);

    let health_text = format!("HP {:.1}/{}", healt.0, max_health.0);
    health_label.set_text(&health_text);

    let damage_text = format!("Base Damage {}", damage.0);
    damage_label.set_text(&damage_text);

    health_bar.set_max(max_health.0.into());
    health_bar.set_value(healt.0.into());

    name_label.set_text(&name.0);
}

fn toogle_turn_label(
    mut godot: GodotAccess,
    query: Query<&GodotNodeHandle, With<CombatHudNode>>,
    combat_resource: Res<CombatResource>,
    turn_transition: Res<TurnTransition>,
) {
    let Ok(handle) = query.single() else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(handle, &mut godot) else {
        return;
    };

    let Some(mut label) = get_turn_label(&canvas) else {
        return;
    };

    let Some(entity) = turn_transition.pending_entity else {
        label.set_text("");
        return;
    };

    let text = if is_player_turn(combat_resource) {
        format!("PLAYER TURN: {}", entity)
    } else {
        format!("ENEMY TURN: {}", entity)
    };

    label.set_text(&text);
}

fn on_start_reaction(
    trigger: On<StartReactionEvent>,
    query: Query<&GodotNodeHandle, With<CombatHudNode>>,
    mut godot: GodotAccess,
) {
    let prompt = &trigger.prompt;

    let Ok(handle) = query.single() else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(handle, &mut godot) else {
        return;
    };

    let Some(mut attack_label) = get_reaction_label(&canvas) else {
        return;
    };

    let Some(mut panel) = get_reaction_prompt(&canvas) else {
        return;
    };

    panel.set_visible(true);

    attack_label.set_text(prompt);
}

fn update_reaction_timer(
    query: Query<&GodotNodeHandle, With<CombatHudNode>>,
    mut godot: GodotAccess,
    window: Res<ReactionWindow>,
) {
    if !window.active {
        return;
    }

    let Ok(handle) = query.single() else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(handle, &mut godot) else {
        return;
    };

    let Some(mut bar) = get_reaction_timer(&canvas) else {
        return;
    };

    let remaining = window.timer.remaining_secs();
    let duration = window.timer.duration().as_secs_f32();

    let progress = if duration > 0.0 {
        remaining / duration
    } else {
        0.0
    };

    bar.set_value(f64::from(progress));
}

fn on_finish_attack_event(
    _trigger: On<FinishAttackEvent>,
    query: Query<&GodotNodeHandle, With<CombatHudNode>>,
    mut godot: GodotAccess,
) {
    let Ok(handle) = query.single() else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(handle, &mut godot) else {
        return;
    };

    let Some(mut reaction_prompt) = get_reaction_prompt(&canvas) else {
        return;
    };

    reaction_prompt.set_visible(false);
}

fn exit_combat_hud(
    mut combat_resource: ResMut<CombatHudAssets>,
    mut godot: GodotAccess,
    query_hud: Query<&GodotNodeHandle, With<CombatHudNode>>,
) {
    let Ok(hud_handle) = query_hud.single() else {
        return;
    };

    let Some(mut hud) = get_custom_canvas_layer(hud_handle, &mut godot) else {
        return;
    };

    hud.set_visible(false);
    combat_resource.menu_entered = false;
}

fn combat_hud_not_initialized(menu_assets: Res<CombatHudAssets>) -> bool {
    !menu_assets.initialized
}

fn combat_hud_initialized_but_signals_not_connected(menu_assets: Res<CombatHudAssets>) -> bool {
    menu_assets.initialized && !menu_assets.signal_connected
}

fn not_entered_hud(combat_assets: Res<CombatHudAssets>) -> bool {
    combat_assets.initialized && combat_assets.signal_connected && !combat_assets.menu_entered
}

pub struct CombatHudPlugin;

impl Plugin for CombatHudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatHudAssets>()
            .add_plugins(GodotSignalsPlugin::<AttackEvent>::default())
            .add_systems(OnEnter(GameState::InCombat), reset_combat_assets)
            .add_systems(OnExit(GameState::InCombat), exit_combat_hud)
            .add_systems(
                Update,
                (
                    initialized_combat_hud.run_if(combat_hud_not_initialized),
                    connect_button.run_if(combat_hud_initialized_but_signals_not_connected),
                    enter_combat_hud.run_if(not_entered_hud),
                    toogle_combat_hud_visibility,
                    toogle_turn_label,
                    update_reaction_timer,
                    update_target_information,
                )
                    .run_if(in_state(GameState::InCombat)),
            )
            .add_observer(on_attack_button)
            .add_observer(on_start_reaction)
            .add_observer(on_finish_attack_event)
            .add_observer(on_hover_enter)
            .add_observer(on_hover_exit);
    }
}
