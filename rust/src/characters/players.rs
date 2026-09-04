use bevy::prelude::*;
use godot::classes::{CharacterBody2D, Input};
use godot_bevy::prelude::*;

const SPEED: f32 = 400.0;

const GRAVITY: f32 = 980.0;

const JUMP_VELOCITY: f32 = -500.0;

#[derive(Component, GodotNode, Default)]
#[godot_node(base(CharacterBody2D), class_name(Player2D))]
pub struct PlayerNode {}

fn find_player(query: Query<(&GodotNodeHandle, &Name), With<PlayerNode>>) {
    let Ok((handler, name)) = query.single() else {
        println!("Not found or more than one");
        return;
    };

    godot::prelude::godot_print!("Found Player");
    godot::prelude::godot_print!("Handler: {:?}", handler);
    godot::prelude::godot_print!("name: {:?}", name);
}

fn move_player(
    query: Query<&GodotNodeHandle, With<PlayerNode>>,
    mut godot: GodotAccess,
    time: Res<Time>,
) {
    if let Ok(handle) = query.single() {
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

        velocity.x = direction * SPEED;

        if !body.is_on_floor() {
            velocity.y += GRAVITY * time.delta_secs();
        }

        if input.is_action_pressed("jump") && body.is_on_floor() {
            velocity.y = JUMP_VELOCITY;
        }

        body.set_velocity(velocity);
        body.move_and_slide();
    }
}

pub struct PlayerPlugin;

impl Plugin for PlayerPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, find_player)
            .add_systems(Update, move_player);
    }
}
