use bevy::prelude::*;
use godot::classes::Input;
use godot::global::godot_print;
use godot_bevy::interop::GodotAccess;

use crate::{
    combat::attack::{CurrentAttack, FinishAttackEvent},
    events::damage::DamageEvent,
    state::GameState,
};

pub enum ReactionResult {
    Failed,
    Success,
}

#[derive(Event)]
pub struct StartReactionEvent {
    pub duration: f32,
}

#[derive(Event)]
pub struct ReactionFinished {
    result: ReactionResult,
}

#[derive(Resource, Default)]
pub struct ReactionWindow {
    timer: Timer,
    active: bool,
}

fn on_start_reaction(trigger: On<StartReactionEvent>, mut window: ResMut<ReactionWindow>) {
    let duration = trigger.event().duration;
    window.timer = Timer::from_seconds(duration, TimerMode::Once);
    window.active = true;
    godot_print!("Reaction window START - duration: {:.2}s", duration);
}

fn proccess_reaction_window(
    time: Res<Time>,
    mut window: ResMut<ReactionWindow>,
    mut commands: Commands,
    mut godot: GodotAccess,
) {
    if !window.active {
        return;
    }

    let input = godot.singleton::<Input>();

    if input.is_action_just_pressed("combat_parry") {
        window.active = false;
        godot_print!("Reaction window RESULT - SUCCESS");
        commands.trigger(ReactionFinished {
            result: ReactionResult::Success,
        });

        return;
    }

    window.timer.tick(time.delta());
    if window.timer.just_finished() {
        window.active = false;
        godot_print!("Reaction window RESULT - FAILED (timeout)");
        commands.trigger(ReactionFinished {
            result: ReactionResult::Failed,
        });
    }
}

fn on_reaction_finish(
    trigger: On<ReactionFinished>,
    mut commands: Commands,
    mut attack_resource: ResMut<CurrentAttack>,
) {
    let result = &trigger.event().result;

    let Some(source) = attack_resource.source else {
        return;
    };

    let Some(target) = attack_resource.target else {
        return;
    };

    let Some(definition) = attack_resource.hits.get(attack_resource.current_hit) else {
        return;
    };

    match result {
        ReactionResult::Failed => {
            godot_print!("Hit {} - DAMAGE", attack_resource.current_hit + 1);
            commands.trigger(DamageEvent {
                source,
                target,
                damage: Some(definition.damage),
            });
        }
        ReactionResult::Success => {
            godot_print!("Hit {} - MITIGATED", attack_resource.current_hit + 1);
        }
    }

    let next = attack_resource.current_hit + 1;
    if next >= attack_resource.hits.len() {
        commands.trigger(FinishAttackEvent);
        return;
    }
    attack_resource.current_hit += 1;
    attack_resource.waiting_for_reaction = false;
}

pub struct ReactionPlugin;

impl Plugin for ReactionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ReactionWindow>()
            .add_systems(
                Update,
                proccess_reaction_window.run_if(in_state(GameState::InCombat)),
            )
            .add_observer(on_start_reaction)
            .add_observer(on_reaction_finish);
    }
}
