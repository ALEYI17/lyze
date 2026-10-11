use bevy::prelude::*;
use godot::{
    classes::{CharacterBody3D, Label3D},
    prelude::*,
};
use godot_bevy::{
    interop::{GodotAccess, GodotNodeHandle},
    prelude::GodotNode,
};

use crate::{
    characters::{
        components::{
            state::Alive,
            stats::{Damage, Gravity, Health, Speed},
        },
        helpers::sprite::{get_sprite, set_shader_false, set_shader_true},
    },
    combat::attack::{AttackDefinition, AttackEvent, DodgeDirection, ReactionType},
    events::{
        combat::{CombatResource, CombatTarget, NextTurn, TurnStarted},
        damage::Die,
    },
    godot_utils::nodes::get_custom_character_body_3d,
    state::GameState,
};

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = CharacterBody3D, class_name = CapeEnemy3D)]
#[gdbevy(
    require(speed: Speed, as = f32, default = 150.0),
    require(health: Health, as = f32, default = 50.0),
    require(damage: Damage, as = f32, default = 5.0),
    require(enemy_gravity: Gravity, as = f32, default = 980.0),
    require(alive: Alive, as = bool, default = true),
)]
pub struct CapeEnemyNode3D;

fn get_health_in_label(body: &Gd<CharacterBody3D>) -> Option<Gd<Label3D>> {
    let label_node = body.get_node_or_null("Label3D")?;

    let Ok(label) = label_node.try_cast::<Label3D>() else {
        return None;
    };

    Some(label)
}

fn on_enemy_turn(
    trigger: On<TurnStarted>,
    query: Query<(&GodotNodeHandle, &Alive, &Health), With<CapeEnemyNode3D>>,
    mut commands: Commands,
    combat_resource: Res<CombatResource>,
) {
    godot_print!("Receive event");
    let entity = trigger.event().entity;

    let Ok((handle, alive, health)) = query.get(entity) else {
        return;
    };

    if !alive.0 || health.0 <= 0.0 {
        commands.trigger(NextTurn);
        return;
    }

    let Some(player) = combat_resource.player else {
        return;
    };

    godot_print!("Enemy entity:{}, handle: {:?}", entity, handle);

    commands.trigger(AttackEvent {
        source: entity,
        target: player,
        attacks: vec![
            AttackDefinition {
                damage: 5.0,
                reaction_window: 1.0,
                reaction: ReactionType::Parry,
            },
            AttackDefinition {
                damage: 7.0,
                reaction_window: 1.5,
                reaction: ReactionType::Parry,
            },
            AttackDefinition {
                damage: 10.0,
                reaction_window: 0.8,
                reaction: ReactionType::Parry,
            },
            AttackDefinition {
                damage: 2.0,
                reaction_window: 1.3,
                reaction: ReactionType::Dodge(vec![DodgeDirection::Left, DodgeDirection::Right]),
            },
        ],
    });
}

fn update_health_label(
    query: Query<(&GodotNodeHandle, &Health), With<CapeEnemyNode3D>>,
    mut godot: GodotAccess,
) {
    for (handle, health) in query {
        let Some(body) = get_custom_character_body_3d(handle, &mut godot) else {
            continue;
        };

        let Some(mut label) = get_health_in_label(&body) else {
            continue;
        };

        let text = format!("{}", health.0);
        label.set_text(&text);
    }
}

fn on_die_enemy(
    trigger: On<Die>,
    query: Query<&GodotNodeHandle, With<CapeEnemyNode3D>>,
    mut godot: GodotAccess,
) {
    let Ok(handle) = query.get(trigger.entity) else {
        return;
    };

    let Some(mut body) = get_custom_character_body_3d(handle, &mut godot) else {
        return;
    };

    body.set_visible(false);
}

fn is_target(
    target: Res<CombatTarget>,
    query: Query<(Entity, &GodotNodeHandle), With<CapeEnemyNode3D>>,
    mut godot: GodotAccess,
    combat: Res<CombatResource>,
) {
    let Some(target) = target.target else {
        return;
    };
    let Some(current_entity) = combat.current_entity() else {
        return;
    };

    let Some(player) = combat.player else {
        return;
    };

    let is_player_turn = player == current_entity;

    for (entity, handle) in query {
        let Some(body) = get_custom_character_body_3d(handle, &mut godot) else {
            return;
        };

        let Some(mut sprite) = get_sprite(body) else {
            return;
        };

        let is_selected = is_player_turn && entity == target;

        if is_selected {
            set_shader_true(&mut sprite);
        } else {
            set_shader_false(&mut sprite);
        }
    }
}

pub struct CapeEnemy3DPlugin;

impl Plugin for CapeEnemy3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            update_health_label.run_if(in_state(GameState::InCombat)),
        )
        .add_systems(Update, is_target.run_if(in_state(GameState::InCombat)))
        .add_observer(on_enemy_turn)
        .add_observer(on_die_enemy);
    }
}
