use bevy::prelude::*;
use godot::classes::{CanvasLayer, class_macros::private::virtuals::Xrvrs::Gd};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::events::combat::NextTurn;
use crate::{
    events::{combat::CombatResource, damage::DamageEvent},
    state::GameState,
};

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = CanvasLayer, class_name = CombatHud)]
pub struct CombatHudNode;

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

fn get_custom_canvas_layer(
    handle: &GodotNodeHandle,
    godot: &mut GodotAccess,
) -> Option<Gd<CanvasLayer>> {
    let canvas = godot.try_get::<CanvasLayer>(*handle)?;
    Some(canvas)
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
) {
    let Some(current_entity) = combat_resource.current_entity() else {
        return;
    };

    let Some(enemy) = combat_resource.enemy else {
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
                )
                    .run_if(in_state(GameState::InCombat)),
            )
            .add_observer(on_attack_button);
    }
}
