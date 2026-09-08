use bevy::prelude::*;
use godot::classes::{CharacterBody2D, CollisionShape2D, Input};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::state::GameState;

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct PlayerNode;

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Speed(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct JumpVelocity(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Gravity(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Damage(pub f32);

#[derive(Bundle, GodotNode, Default)]
#[godot_node(base(CharacterBody2D), class_name(Player2D))]
pub struct PlayerGodotNode {
    pub player: PlayerNode,

    #[export_fields(value(export_type(f32), default(400.0)))]
    pub speed: Speed,

    #[export_fields(value(export_type(f32), default(-500.0)))]
    pub jump_velocity: JumpVelocity,

    #[export_fields(value(export_type(f32), default(980.0)))]
    pub gravity: Gravity,

    #[export_fields(value(export_type(f32), default(50.0)))]
    pub health: Health,

    #[export_fields(value(export_type(f32), default(5.0)))]
    pub damage: Damage,
}

#[derive(Resource)]
struct PlayerAttackTimer {
    cooldown_timer: Timer,
    active_timer: Timer,
    is_attacking: bool,
}

impl Default for PlayerAttackTimer {
    fn default() -> Self {
        Self {
            cooldown_timer: Timer::from_seconds(0.5, TimerMode::Once),
            active_timer: Timer::from_seconds(0.5, TimerMode::Once),
            is_attacking: false,
        }
    }
}

fn move_player(
    query: Query<(&GodotNodeHandle, &Gravity, &Speed, &JumpVelocity), With<PlayerNode>>,
    mut godot: GodotAccess,
    time: Res<Time>,
) {
    if let Ok((handle, gravity, speed, jump_velocity)) = query.single() {
        let Some(mut body) = godot.try_get::<CharacterBody2D>(*handle) else {
            return;
        };

        let mut velocity = body.get_velocity();

        let mut direction = 0.0;

        let input = godot.singleton::<Input>();

        if input.is_action_pressed("move_right") {
            direction += 1.0;
        }
        if input.is_action_pressed("move_left") {
            direction -= 1.0;
        }

        velocity.x = direction * speed.0;

        if !body.is_on_floor() {
            velocity.y += gravity.0 * time.delta_secs();
        }

        if input.is_action_pressed("jump") && body.is_on_floor() {
            velocity.y = jump_velocity.0;
        }

        body.set_velocity(velocity);
        body.move_and_slide();
    }
}

fn player_attack(
    query: Query<&GodotNodeHandle, With<PlayerNode>>,
    mut timer: ResMut<PlayerAttackTimer>,
    mut godot: GodotAccess,
) {
    if let Ok(handle) = query.single() {
        let Some(body) = godot.try_get::<CharacterBody2D>(*handle) else {
            return;
        };

        let input = godot.singleton::<Input>();

        if timer.is_attacking || !timer.cooldown_timer.is_finished() {
            return;
        }

        if input.is_action_just_pressed("hit") {
            let Some(hittbox_node) = body.get_node_or_null("hitbox/CollisionShape2D") else {
                godot_print!("dont get player hitbox");
                return;
            };

            let Ok(mut hitbox) = hittbox_node.try_cast::<CollisionShape2D>() else {
                godot_print!("cant cast hitbox");
                return;
            };

            hitbox.set_disabled(false);

            timer.is_attacking = true;
            timer.active_timer.reset();
            timer.cooldown_timer.reset();
        }
    }
}

fn update_attack(
    query: Query<&GodotNodeHandle, With<PlayerNode>>,
    mut timer: ResMut<PlayerAttackTimer>,
    mut godot: GodotAccess,
    time: Res<Time>,
) {
    timer.cooldown_timer.tick(time.delta());

    if !timer.is_attacking {
        return;
    }

    timer.active_timer.tick(time.delta());

    if !timer.active_timer.is_finished() {
        return;
    }

    let Ok(handle) = query.single() else {
        return;
    };

    let Some(body) = godot.try_get::<CharacterBody2D>(*handle) else {
        return;
    };

    let Some(hittbox_node) = body.get_node_or_null("hitbox/CollisionShape2D") else {
        return;
    };

    let Ok(mut hitbox) = hittbox_node.try_cast::<CollisionShape2D>() else {
        return;
    };

    hitbox.set_disabled(true);
    timer.is_attacking = false;
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PlayerAttackTimer>()
            .add_systems(Update, move_player.run_if(in_state(GameState::InGame)))
            .add_systems(Update, player_attack.run_if(in_state(GameState::InGame)))
            .add_systems(Update, update_attack.run_if(in_state(GameState::InGame)));
    }
}
