use bevy::prelude::*;
use godot::prelude::*;

use crate::{characters::components::stats::Alive, state::GameState};

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
        app_state.set(GameState::Ingame3D);
    } else {
        let Some(player) = combat_resource.player else {
            return;
        };
        commands.entity(player).despawn();
        app_state.set(GameState::Ingame3D);
    }
}

pub struct CombatPlugin;

impl Plugin for CombatPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CombatResource>()
            .add_observer(on_died_in_combat)
            .add_observer(on_enter_combat)
            .add_observer(on_next_turn)
            .add_observer(on_end_combat);
    }
}
