use bevy::prelude::*;
use godot::classes::Input;
use godot::global::godot_print;
use godot_bevy::interop::GodotAccess;

use crate::{
    combat::attack::{CurrentAttack, DodgeDirection, FinishAttackEvent, ReactionType},
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
    result: Option<ReactionResult>,
}

fn on_start_reaction(trigger: On<StartReactionEvent>, mut window: ResMut<ReactionWindow>) {
    let duration = trigger.event().duration;
    window.timer = Timer::from_seconds(duration, TimerMode::Once);
    window.active = true;
    window.result = None;
    godot_print!("Reaction window START - duration: {:.2}s", duration);
}

fn proccess_reaction_window(
    time: Res<Time>,
    mut window: ResMut<ReactionWindow>,
    mut commands: Commands,
    mut godot: GodotAccess,
    attack: Res<CurrentAttack>,
) {
    if !window.active {
        return;
    }

    let Some(hit) = attack.hits.get(attack.current_hit) else {
        return;
    };

    let reaction_type = &hit.reaction;

    let input = godot.singleton::<Input>();

    match reaction_type {
        ReactionType::Parry => {
            if window.result.is_none() && input.is_action_just_pressed("combat_parry") {
                godot_print!("Reaction window RESULT - SUCCESS");
                window.result = Some(ReactionResult::Success);
            }
        }
        ReactionType::Dodge(directions) => {
            if window.result.is_some() {
            } else {
                let dodge_direction = if input.is_action_just_pressed("move_forward") {
                    Some(DodgeDirection::Forward)
                } else if input.is_action_just_pressed("move_backward") {
                    Some(DodgeDirection::Backward)
                } else if input.is_action_just_pressed("move_left") {
                    Some(DodgeDirection::Left)
                } else if input.is_action_just_pressed("move_right") {
                    Some(DodgeDirection::Right)
                } else {
                    None
                };

                if let Some(direction) = dodge_direction
                    && directions.contains(&direction)
                {
                    window.result = Some(ReactionResult::Success);
                    godot_print!("Reaction window RESULT - SUCCESS");
                }
            }
        }
    }

    window.timer.tick(time.delta());
    if window.timer.just_finished() {
        window.active = false;
        let result = window.result.take().unwrap_or(ReactionResult::Failed);

        if matches!(result, ReactionResult::Failed) {
            godot_print!("Reaction window RESULT - FAILED (timeout)");
        }
        commands.trigger(ReactionFinished { result });
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
