use bevy::prelude::*;
use godot::global::godot_print;

use crate::{combat::reaction::StartReactionEvent, events::combat::NextTurn, state::GameState};

#[derive(Clone, PartialEq, Eq, Debug)]
pub enum DodgeDirection {
    Forward,
    Backward,
    Left,
    Right,
}

#[derive(Clone)]
pub enum ReactionType {
    Parry,
    Dodge(Vec<DodgeDirection>),
}

#[derive(Clone)]
pub struct AttackDefinition {
    pub damage: f32,
    pub reaction_window: f32,
    pub reaction: ReactionType,
}

#[derive(Event)]
pub struct AttackEvent {
    pub target: Entity,
    pub source: Entity,
    pub attacks: Vec<AttackDefinition>,
}

#[derive(Event)]
pub struct FinishAttackEvent;

#[derive(Resource, Default)]
pub struct CurrentAttack {
    pub source: Option<Entity>,
    pub target: Option<Entity>,
    pub hits: Vec<AttackDefinition>,
    pub current_hit: usize,
    pub waiting_for_reaction: bool,
}

fn on_attack_event(trigger: On<AttackEvent>, mut attack_resource: ResMut<CurrentAttack>) {
    attack_resource.source = Some(trigger.event().source);
    attack_resource.target = Some(trigger.event().target);
    attack_resource.hits = trigger.event().attacks.clone();
    attack_resource.current_hit = 0;
}

fn proccess_attacks(mut attacks: ResMut<CurrentAttack>, mut commands: Commands) {
    if attacks.waiting_for_reaction {
        return;
    }

    let Some(current_attack) = attacks.hits.get(attacks.current_hit) else {
        return;
    };

    match &current_attack.reaction {
        ReactionType::Parry => {
            godot_print!("Required reaction: PARRY");
        }
        ReactionType::Dodge(directions) => {
            godot_print!("Required reaction: DODGE - {:?}", directions);
        }
    }

    commands.trigger(StartReactionEvent {
        duration: current_attack.reaction_window,
    });

    attacks.waiting_for_reaction = true;
}

fn on_finish_attacks_event(
    _trigger: On<FinishAttackEvent>,
    mut attack_resource: ResMut<CurrentAttack>,
    mut commands: Commands,
) {
    godot_print!("ATTACK FINISHED");

    attack_resource.waiting_for_reaction = false;
    attack_resource.current_hit = 0;
    attack_resource.target = None;
    attack_resource.source = None;
    attack_resource.hits = Vec::new();

    commands.trigger(NextTurn);
}

pub struct AttackPlugin;

impl Plugin for AttackPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentAttack>()
            .add_systems(
                Update,
                proccess_attacks.run_if(in_state(GameState::InCombat)),
            )
            .add_observer(on_attack_event)
            .add_observer(on_finish_attacks_event);
    }
}
