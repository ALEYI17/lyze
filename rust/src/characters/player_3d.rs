use bevy::prelude::*;
use godot::classes::Input;
use godot_bevy::prelude::*;

use crate::{
    characters::{
        components::{
            state::Alive,
            stats::{Damage, Gravity, Health, JumpVelocity, Speed},
        },
        helpers::sprite::{get_sprite, set_shader_false, set_shader_true},
    },
    events::combat::{CombatResource, CombatTarget, DiedInCombat},
    godot_utils::nodes::get_custom_character_body_3d,
    state::GameState,
};

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = CharacterBody3D, class_name = Player3D)]
#[gdbevy(
    require(speed: Speed, as = f32, default = 10.0),
    require(jump_velocity: JumpVelocity, as = f32, default = 20.0),
    require(player_gravity: Gravity, as = f32, default = 98.0),
    require(health: Health, as = f32, default = 50.0),
    require(damage: Damage, as = f32, default = 15.0),
    require(alive: Alive, as = bool, default = true),
)]
pub struct Player3DNode;

fn move_player_3d_node(
    query: Query<(&GodotNodeHandle, &Speed, &JumpVelocity, &Gravity), With<Player3DNode>>,
    mut godot: GodotAccess,
    time: Res<Time>,
) {
    let Ok((node_handle, speed, jump_velocity, gravity)) = query.single() else {
        return;
    };

    let Some(mut body) = get_custom_character_body_3d(node_handle, &mut godot) else {
        return;
    };

    let input = godot.singleton::<Input>();

    let mut direction = 0.0;

    let mut direction_z = 0.0;

    let mut velocity = body.get_velocity();

    if input.is_action_pressed("move_right") {
        direction += 1.0;
    }

    if input.is_action_pressed("move_left") {
        direction -= 1.0;
    }

    if input.is_action_pressed("move_forward") {
        direction_z -= 1.0;
    }

    if input.is_action_pressed("move_backward") {
        direction_z += 1.0;
    }

    if !body.is_on_floor() {
        velocity.y -= gravity.0 * time.delta_secs();
    }

    if body.is_on_floor() && input.is_action_just_pressed("jump") {
        velocity.y = jump_velocity.0;
    }

    velocity.x = direction * speed.0;
    velocity.z = direction_z * speed.0;

    body.set_velocity(velocity);

    body.move_and_slide();
}

fn kill_player(query: Query<(Entity, &Health), With<Player3DNode>>, mut commands: Commands) {
    let Ok((entity, health)) = query.single() else {
        return;
    };

    if health.0 <= 0.0 {
        commands.trigger(DiedInCombat { entity });
    }
}

fn is_target(
    target: Res<CombatTarget>,
    query: Query<(Entity, &GodotNodeHandle), With<Player3DNode>>,
    mut godot: GodotAccess,
    combat: Res<CombatResource>,
) {
    let Ok((entity, handle)) = query.single() else {
        return;
    };

    let Some(target) = target.target else {
        return;
    };

    let Some(body) = get_custom_character_body_3d(handle, &mut godot) else {
        return;
    };

    let Some(mut sprite) = get_sprite(body) else {
        return;
    };

    let Some(current_entity) = combat.current_entity() else {
        return;
    };

    let Some(player) = combat.player else {
        return;
    };

    let is_player_turn = current_entity == player;

    let is_selected = is_player_turn && entity == target;

    if is_selected {
        set_shader_true(&mut sprite);
    } else {
        set_shader_false(&mut sprite);
    }
}

pub struct Player3DPlugin;

impl Plugin for Player3DPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            move_player_3d_node.run_if(in_state(GameState::Ingame3D)),
        )
        .add_systems(Update, is_target.run_if(in_state(GameState::InCombat)))
        .add_systems(Update, kill_player.run_if(in_state(GameState::InCombat)));
    }
}
