use bevy::prelude::*;
use godot::classes::{CanvasLayer, Control, Label};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::characters::components::state::Alive;
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

    let Some(entity) =  turn_transition.pending_entity else{
        label.set_text("");
        return;
    };

    let text = if is_player_turn(combat_resource){
        format!("PLAYER TURN: {}", entity)
    }else{
        format!("ENEMY TURN: {}", entity)
    };


    label.set_text(&text);
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
                )
                    .run_if(in_state(GameState::InCombat)),
            )
            .add_observer(on_attack_button);
    }
}
