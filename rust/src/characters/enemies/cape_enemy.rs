use std::sync::Mutex;

use bevy::prelude::*;
use godot::classes::{Area2D, Sprite2D, Timer};
use godot::prelude::*;
use godot_bevy::prelude::*;

use crate::state::GameState;

#[derive(Component, GodotNode, Default)]
#[godot_node(base(Area2D), class_name(CapeEnemy))]
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
struct EnteredBody;

fn connect_timeout_signal(
    signal_direction: GodotSignals<ChangeDirectionRequested>,
    signal_enter: GodotSignals<EnteredBody>,
    mut query: Query<&GodotNodeHandle, With<CapeEnemyNode>>,
    mut godot: GodotAccess,
    mut state: ResMut<CapeEnemyState>,
) {
    for handle in &mut query {
        let Some(body) = godot.try_get::<Area2D>(*handle) else {
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

        signal_enter.connect(
            body.into(),
            Area2DSignals::BODY_ENTERED,
            None,
            |_args, _node_handle, _ent| Some(EnteredBody),
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
        let Some(body) = godot.try_get::<Area2D>(*handle) else {
            return;
        };

        let Some(sprite_node) = body.get_node_or_null("Sprite2D") else {
            godot_print!("Dont get sprite");
            return;
        };

        let Ok(mut sprite) = sprite_node.try_cast::<Sprite2D>() else {
            godot_print!("Cant cast sprite");
            return;
        };

        let flip = sprite.is_flipped_h();
        sprite.set_flip_h(!flip);
    }

    let mut direction = DIRECTION.lock().unwrap();
    *direction = *direction * -1.0;
}

fn on_enter_body(_tigger: On<EnteredBody>) {
    godot_print!("Enter body kill player");
}

fn is_not_initialized(state: Res<CapeEnemyState>) -> bool {
    !state.initialized
}

fn find_cape_enemy(
    mut query: Query<&GodotNodeHandle, With<CapeEnemyNode>>,
    mut godot: GodotAccess,
    time: Res<Time>,
) {
    for handle in &mut query {
        let Some(mut body) = godot.try_get::<Area2D>(*handle) else {
            return;
        };

        let mut pos = body.get_position();

        let direction = DIRECTION.lock().unwrap();
        pos.x += *direction * SPEED * time.delta_secs();

        body.set_position(pos);
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
