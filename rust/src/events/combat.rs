use bevy::prelude::*;
use godot::prelude::*;

use crate::state::GameState;

#[derive(Event)]
pub struct EnterCombatEvent {
    pub player: Entity,
    pub enemy: Entity,
}

#[derive(Event)]
pub struct TurnStarted {
    pub entity: Entity,
}
#[derive(Event)]
pub struct NextTurn;

#[derive(Event)]
pub struct DiedInCombat {
    pub entity: Entity,
}

#[derive(Resource, Default)]
pub struct CombatResource {
    pub player: Option<Entity>,
    pub enemy: Option<Entity>,
    activate: bool,
    pub turn_order: Vec<Entity>,
    pub current_turn: usize,
}

impl CombatResource {
    pub fn next_turn(&mut self) {
        self.current_turn = (self.current_turn + 1) % self.turn_order.len();
    }

    pub fn current_entity(&self) -> Option<Entity> {
        self.turn_order.get(self.current_turn).copied()
    }
}

fn on_enter_combat(
    trigger: On<EnterCombatEvent>,
    mut combat_resource: ResMut<CombatResource>,
    state: Res<State<GameState>>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    if combat_resource.activate {
        return;
    }

    if *state != GameState::Ingame3D {
        return;
    }

    combat_resource.activate = true;
    godot_print!("get enter combat event");

    let enemy = trigger.event().enemy;

    let player = trigger.event().player;

    godot_print!("In Combat player: {}, with enemy: {}", player, enemy);

    combat_resource.enemy = Some(enemy);

    combat_resource.player = Some(player);

    combat_resource.turn_order.push(player);

    combat_resource.turn_order.push(enemy);

    combat_resource.current_turn = 0;

    app_state.set(GameState::InCombat);
}

fn on_next_turn(
    _trigger: On<NextTurn>,
    mut combat_resource: ResMut<CombatResource>,
    mut commands: Commands,
) {
    combat_resource.next_turn();

    let Some(entity) = combat_resource.current_entity() else {
        return;
    };

    commands.trigger(TurnStarted { entity });
}

fn on_died_in_combat(
    trigger: On<DiedInCombat>,
    mut combat_resource: ResMut<CombatResource>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    let dead_entity = trigger.event().entity;

    if combat_resource.enemy == Some(dead_entity) {
        godot_print!("Enemy died");

        combat_resource.enemy = None;
        combat_resource.activate = false;
        app_state.set(GameState::Ingame3D);
    }

    if combat_resource.player == Some(dead_entity) {
        godot_print!("Player die");
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatResource>()
            .add_observer(on_died_in_combat)
            .add_observer(on_enter_combat)
            .add_observer(on_next_turn);
    }
}
