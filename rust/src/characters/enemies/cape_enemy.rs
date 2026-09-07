use std::sync::Mutex;

use bevy::prelude::*;
use godot::classes::{Area2D, CharacterBody2D, Timer};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::state::GameState;

#[derive(Component, GodotNode, Default)]
#[godot_node(base(CharacterBody2D), class_name(CapeEnemy))]
pub struct CapeEnemyNode {}

const SPEED: f32 = 150.0;

static DIRECTION: Mutex<f32> = Mutex::new(-1.0);

#[derive(Resource, Default)]
struct CapeEnemyState {
    pub initialized: bool,
}

#[derive(Event, Debug, Clone)]
struct ChangeDirectionRequested;

#[derive(Event, Debug, Clone)]
struct EnteredBody{
    node: GodotNodeHandle,
    entity: Option<Entity>,
}

fn connect_timeout_signal(
    signal_direction: GodotSignals<ChangeDirectionRequested>,
    signal_enter: GodotSignals<EnteredBody>,
    mut query: Query<&GodotNodeHandle, With<CapeEnemyNode>>,
    mut godot: GodotAccess,
    mut state: ResMut<CapeEnemyState>,
) {
    for handle in &mut query {
        let Some(body) = godot.try_get::<CharacterBody2D>(*handle) else {
            godot_print!("Dont get body enemy");
            return;
        };

        let Some(timer_node) = body.get_node_or_null("Timer") else {
            godot_print!("Dont get timer");
            return;
        };

        let Ok(timer) = timer_node.try_cast::<Timer>() else {
            godot_print!("Cant cast");
            return;
        };

        let Some(hurtbox_node) = body.get_node_or_null("hitbox") else {
            godot_print!("dont get hitbox");
            return;
        };

        let Ok(hurtbox) = hurtbox_node.try_cast::<Area2D>() else {
            godot_print!("cant cast hitbox");
            return;
        };

        signal_enter.connect(
            hurtbox.into(),
            Area2DSignals::AREA_ENTERED,
            None,
            |_args, node_handle, ent| {
                Some(EnteredBody { entity: ent, node: node_handle })
            },
        );

        signal_direction.connect(
            timer.into(),
            TimerSignals::TIMEOUT,
            None,
            |_args, _node_handle, _ent| Some(ChangeDirectionRequested),
        );

        state.initialized = true;
    }
}

fn on_timeout(
    _tigger: On<ChangeDirectionRequested>,
    mut query: Query<&GodotNodeHandle, With<CapeEnemyNode>>,
    mut godot: GodotAccess,
) {
    for handle in &mut query {
        let Some(mut body) = godot.try_get::<CharacterBody2D>(*handle) else {
            return;
        };


        let scale = body.get_scale();
        body.set_scale(Vector2::new(-scale.x, scale.y));
    }

    let mut direction = DIRECTION.lock().unwrap();
    *direction = *direction * -1.0;
}

fn on_enter_body(tigger: On<EnteredBody>) {
    godot_print!("enter body kill player: {:?}, {:?}", tigger.entity, tigger.node);
}

fn is_not_initialized(state: Res<CapeEnemyState>) -> bool {
    !state.initialized
}

fn find_cape_enemy(
    mut query: Query<&GodotNodeHandle, With<CapeEnemyNode>>,
    mut godot: GodotAccess,
) {
    for handle in &mut query {
        let Some(mut body) = godot.try_get::<CharacterBody2D>(*handle) else {
            godot_print!("dont find enemy");
            return;
        };

        let mut dir = 0.0;

        let mut pos = body.get_velocity();

        let direction = DIRECTION.lock().unwrap();

        dir += *direction;
        pos.x = dir * SPEED;

        body.set_velocity(pos);

        body.move_and_slide();
    }
}

pub struct CapeEnemyPlugin;

impl Plugin for CapeEnemyPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CapeEnemyState>()
            .add_systems(Update, find_cape_enemy.run_if(in_state(GameState::InGame)))
            .add_systems(Update, connect_timeout_signal.run_if(is_not_initialized))
            .add_plugins(GodotSignalsPlugin::<ChangeDirectionRequested>::default())
            .add_plugins(GodotSignalsPlugin::<EnteredBody>::default())
            .add_observer(on_timeout)
            .add_observer(on_enter_body);
    }
}
