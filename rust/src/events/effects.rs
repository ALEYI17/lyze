use bevy::prelude::*;
use godot::global::godot_print;
use godot_bevy::interop::{GodotAccess, GodotNodeHandle};

use crate::{
    characters::{
        helpers::sprite::{get_sprite, set_damage_flash, set_success_flash},
        player_3d::Player3DNode,
    },
    godot_utils::nodes::get_custom_character_body_3d,
    state::GameState,
};

#[derive(Debug)]
pub enum CombatVisualResult {
    ParrySuccess,
    PlayerDamaged,
}

#[derive(Event, Debug)]
pub struct CombatVisulEvent {
    pub entity: Entity,
    pub result: CombatVisualResult,
}

#[derive(Resource, Default)]
struct VisualTimer {
    timer: Option<Timer>,
}

fn on_combat_visual_result(
    trigger: On<CombatVisulEvent>,
    query: Query<&GodotNodeHandle, With<Player3DNode>>,
    mut godot: GodotAccess,
    mut visual_resource: ResMut<VisualTimer>,
) {
    let Ok(handle) = query.get(trigger.event().entity) else {
        return;
    };

    let Some(body) = get_custom_character_body_3d(handle, &mut godot) else {
        return;
    };

    let Some(mut sprite) = get_sprite(body) else {
        return;
    };

    match trigger.event().result {
        CombatVisualResult::PlayerDamaged => {
            godot_print!("VISUAL: PlayerDamaged received");
            set_damage_flash(&mut sprite, true);
        }
        CombatVisualResult::ParrySuccess => {
            godot_print!("set shader for parry");
            set_success_flash(&mut sprite, true);
        }
    }

    visual_resource.timer = Some(Timer::from_seconds(0.15, TimerMode::Once));
}

fn process_visual_effect(
    time: Res<Time>,
    mut visual_resource: ResMut<VisualTimer>,
    query: Query<&GodotNodeHandle, With<Player3DNode>>,
    mut godot: GodotAccess,
) {
    let Some(timer) = visual_resource.timer.as_mut() else {
        return;
    };

    timer.tick(time.delta());

    if timer.just_finished() {
        let Ok(handle) = query.single() else {
            return;
        };

        let Some(body) = get_custom_character_body_3d(handle, &mut godot) else {
            return;
        };

        let Some(mut sprite) = get_sprite(body) else {
            return;
        };

        set_damage_flash(&mut sprite, false);
        set_success_flash(&mut sprite, false);
        visual_resource.timer = None;
    }
}

pub struct EffectPlugin;

impl Plugin for EffectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<VisualTimer>()
            .add_systems(
                Update,
                process_visual_effect.run_if(in_state(GameState::InCombat)),
            )
            .add_observer(on_combat_visual_result);
    }
}
