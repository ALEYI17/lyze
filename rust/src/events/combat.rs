use bevy::prelude::*;
use godot::{classes::Input, prelude::*};
use godot_bevy::interop::{GodotAccess, GodotNodeHandle};

use crate::{
    characters::{
        components::stats::Alive,
        enemies::enemy_encounter::{
            EnemyEncounterNode, get_player_starting_position,
        },
        player_3d::Player3DNode,
    }, godot_utils::nodes::{get_custom_area_3d, get_custom_character_body_3d}, state::GameState
};

#[derive(Event)]
pub struct EnterCombatEvent {
    pub player: Entity,
    pub enemy: Vec<Entity>,
    pub encounter: Entity,
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

#[derive(Event)]
struct EndCombat {
    win: bool,
}

#[derive(Resource, Default)]
pub struct CombatTarget {
    pub target: Option<Entity>,
    index: usize,
}

#[derive(Resource, Default)]
pub struct CombatResource {
    pub player: Option<Entity>,
    pub enemy: Vec<Entity>,
    pub encounter: Option<Entity>,
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

    let enemy = &trigger.event().enemy;

    let player = trigger.event().player;

    godot_print!("In Combat player: {}, with enemy: {:?}", player, enemy);

    combat_resource.enemy = enemy.clone();

    combat_resource.player = Some(player);

    combat_resource.turn_order.push(player);

    combat_resource.turn_order.extend(enemy.iter().copied());

    combat_resource.encounter = Some(trigger.event().encounter);

    combat_resource.current_turn = 0;

    app_state.set(GameState::InCombat);
}

fn on_enter_combat_positions(
    trigger: On<EnterCombatEvent>,
    query_player: Query<&GodotNodeHandle, With<Player3DNode>>,
    query_encounter: Query<&GodotNodeHandle, With<EnemyEncounterNode>>,
    mut godot: GodotAccess,
) {
    let Ok(handle) = query_player.single() else {
        return;
    };

    let Some(mut body) = get_custom_character_body_3d(handle, &mut godot) else {
        return;
    };

    let Ok(encounter_handle) = query_encounter.get(trigger.event().encounter) else {
        return;
    };

    let Some(encounter_area) = get_custom_area_3d(encounter_handle, &mut godot) else {
        return;
    };

    let Some(position) = get_player_starting_position(&encounter_area) else {
        return;
    };

    let starting_position = position.get_global_position();

    body.set_global_position(starting_position);
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
    combat_resource: ResMut<CombatResource>,
    alive_query: Query<&Alive>,
    mut commands: Commands,
) {
    let dead_entity = trigger.event().entity;

    if combat_resource.enemy.contains(&dead_entity) {
        godot_print!("Enemy died: {}", dead_entity);

        let all_enemies_dead = combat_resource
            .enemy
            .iter()
            .all(|entity| alive_query.get(*entity).is_ok_and(|alive| !alive.0));

        if all_enemies_dead {
            commands.trigger(EndCombat { win: true });
        }
    }

    if combat_resource.player == Some(dead_entity) {
        godot_print!("Player die");
        commands.trigger(EndCombat { win: false });
    }
}

fn on_end_combat(
    trigger: On<EndCombat>,
    mut app_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut combat_resource: ResMut<CombatResource>,
    mut target: ResMut<CombatTarget>,
) {
    if trigger.event().win {
        for enemy in &combat_resource.enemy {
            commands.entity(*enemy).despawn();
        }
        let Some(encounter) = combat_resource.encounter else {
            return;
        };
        commands.entity(encounter).despawn();
        combat_resource.enemy.clear();
        combat_resource.player = None;
        combat_resource.activate = false;
        combat_resource.encounter = None;
        combat_resource.turn_order.clear();
        combat_resource.current_turn = 0;

        target.target = None;
        app_state.set(GameState::Ingame3D);
    } else {
        let Some(player) = combat_resource.player else {
            return;
        };
        commands.entity(player).despawn();
        app_state.set(GameState::Ingame3D);
    }
}

fn select_target(
    combat_resource: Res<CombatResource>,
    mut target: ResMut<CombatTarget>,
    mut godot: GodotAccess,
    query_alive: Query<&Alive>,
) {
    let alive_entities: Vec<Entity> = combat_resource
        .turn_order
        .iter()
        .filter(|entity| query_alive.get(**entity).is_ok_and(|alive| alive.0))
        .copied()
        .collect();

    if alive_entities.is_empty() {
        return;
    }

    if target.target.is_none() {
        let Some(first) = alive_entities.first().copied() else {
            return;
        };

        target.target = Some(first);
        target.index = 0;

        godot_print!(
            "Actual target: {:?}, actual index: {}",
            target.target,
            target.index
        );
    }

    let input = godot.singleton::<Input>();

    if input.is_action_just_pressed("move_right") {
        let actual_index = (target.index + 1) % alive_entities.len();

        let Some(next_entity) = alive_entities.get(actual_index).copied() else {
            return;
        };

        target.target = Some(next_entity);
        target.index = actual_index;

        godot_print!(
            "Actual target: {:?}, actual index: {}",
            target.target,
            target.index
        );
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatResource>()
            .init_resource::<CombatTarget>()
            .add_observer(on_died_in_combat)
            .add_observer(on_enter_combat)
            .add_observer(on_next_turn)
            .add_observer(on_end_combat)
            .add_observer(on_enter_combat_positions)
            .add_systems(Update, select_target.run_if(in_state(GameState::InCombat)));
    }
}
