use bevy::prelude::*;
use godot::classes::{CanvasLayer, Control, Label, PanelContainer, ProgressBar};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::characters::components::state::Alive;
use crate::combat::attack::FinishAttackEvent;
use crate::combat::reaction::{
    ReactionFinished, ReactionResult, ReactionWindow, StartReactionEvent,
};
use crate::events::combat::{CombatTarget, NextTurn, TurnTransition, is_player_turn};
use crate::godot_utils::nodes::get_custom_canvas_layer;
use crate::{
    events::{combat::CombatResource, damage::DamageEvent},
    state::GameState,
};

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

fn get_attack_label(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let label_handle = canvas.get_node_or_null("ReactionPrompt/ReactionContent/AttackLabel")?;

    let Ok(label) = label_handle.try_cast::<Label>() else {
        return None;
    };

    Some(label)
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

#[derive(Resource, Default)]
pub struct CombatHudAssets {
    pub combat_hud: Option<GodotNodeHandle>,
    pub attack_button: Option<GodotNodeHandle>,
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
}

fn reset_combat_assets(mut combat_assets: ResMut<CombatHudAssets>) {
    combat_assets.combat_hud = None;
    combat_assets.attack_button = None;
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
            combat_assets.initialized = true;
        }
    } else {
        godot_print!("Main Menu scene not avaible");
    }
}

#[derive(Event, Debug, Clone)]
struct AttackEvent;

fn connect_button(
    mut combat_assets: ResMut<CombatHudAssets>,
    signals_attack: GodotSignals<AttackEvent>,
) {
    if combat_assets.attack_button.is_some() && !combat_assets.signal_connected {
        if let Some(attack_handle) = combat_assets.attack_button {
            signals_attack.connect(
                attack_handle,
                BaseButtonSignals::PRESSED,
                None,
                |_args, _node_handle, _ent| Some(AttackEvent),
            );
        }

        combat_assets.signal_connected = true;
    }
}

fn on_attack_button(
    _trigger: On<AttackEvent>,
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

    commands.trigger(DamageEvent {
        source: player,
        target: enemy,
        damage: None,
    });

    commands.trigger(NextTurn);
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

    let Some(mut attack_label) = get_attack_label(&canvas) else {
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
    if !window.active{
        return;
    }

    let Ok(handle) = query.single() else{
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(handle, &mut godot) else{
        return;
    };

    let Some(mut bar) = get_reaction_timer(&canvas) else{
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

fn on_reaction_finished(
    trigger: On<ReactionFinished>,
    query: Query<&GodotNodeHandle, With<CombatHudNode>>,
    mut godot: GodotAccess,
) {
    let result = &trigger.result;

    let Ok(handle) = query.single() else {
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(handle, &mut godot) else {
        return;
    };

    let Some(mut reaction_label) = get_reaction_label(&canvas) else {
        return;
    };

    let result_text = match result {
        ReactionResult::Success => "Success".to_string(),
        ReactionResult::Failed => "Failed".to_string(),
    };
    reaction_label.set_text(&result_text);
}

fn on_finish_attack_event(_trigger: On<FinishAttackEvent>,
    query: Query<&GodotNodeHandle, With<CombatHudNode>>,
    mut godot: GodotAccess,
){
    let Ok(handle) = query.single() else{
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(handle, &mut godot) else {
        return;
    };

    let Some(mut reaction_prompt) = get_reaction_prompt(&canvas) else{
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
                )
                    .run_if(in_state(GameState::InCombat)),
            )
            .add_observer(on_attack_button)
            .add_observer(on_start_reaction)
            .add_observer(on_reaction_finished)
            .add_observer(on_finish_attack_event);
    }
}
