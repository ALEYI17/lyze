use bevy::prelude::*;
use godot::global::godot_print;
use godot_bevy::interop::{GodotAccess, GodotNodeHandle};

use crate::{
    characters::{
        components::{
            state::Alive,
            stats::{Health, MaxHealth},
        },
        helpers::sprite::{get_sprite, set_damage_flash, set_shader_false, set_success_flash},
        player_3d::Player3DNode,
    },
    godot_utils::nodes::{get_custom_canvas_layer, get_custom_character_body_3d},
    ui::hud::{HudAssets, get_you_die_label},
    world::save_point::SavePoint,
};

#[derive(Event, Debug)]
pub struct RespawnEvent {
    pub player: Option<Entity>,
}

#[derive(Resource, Default)]
pub struct RespawnResource {
    pub player: Option<Entity>,
    pub respawn_timer: Timer,
}

fn on_respawn_event(trigger: On<RespawnEvent>, mut respawn_resource: ResMut<RespawnResource>) {
    godot_print!("get respawn event");
    respawn_resource.player = trigger.player;
    respawn_resource.respawn_timer = Timer::from_seconds(3.0, TimerMode::Once);
}

fn proccess_respawn_system(
    mut respawn_resource: ResMut<RespawnResource>,
    hud_assets: Res<HudAssets>,
    mut godot: GodotAccess,
    time: Res<Time>,
    mut player_query: Query<
        (&GodotNodeHandle, &mut Alive, &mut Health, &MaxHealth),
        With<Player3DNode>,
    >,
    save_point: Res<SavePoint>,
) {
    let Some(player_entity) = respawn_resource.player else {
        return;
    };

    let Some(hud_handle) = hud_assets.hud else {
        godot_print!("fail respawn here 1");
        return;
    };

    let Some(canvas) = get_custom_canvas_layer(&hud_handle, &mut godot) else {
        godot_print!("fail respawn here 2");
        return;
    };

    let Some(mut die_label) = get_you_die_label(&canvas) else {
        godot_print!("fail respawn here 3");
        return;
    };

    die_label.set_visible(true);
    respawn_resource.respawn_timer.tick(time.delta());

    if respawn_resource.respawn_timer.is_finished() {
        let Ok((handle, mut alive, mut health, max_health)) = player_query.get_mut(player_entity)
        else {
            godot_print!("fail respawn here 4");
            return;
        };
        let Some(mut body) = get_custom_character_body_3d(handle, &mut godot) else {
            godot_print!("fail respawn here 5");
            return;
        };

        let Some(mut sprite) = get_sprite(&body) else {
            return;
        };
        alive.0 = true;
        health.0 = max_health.0 / 2.0;

        set_shader_false(&mut sprite);
        set_damage_flash(&mut sprite, false);
        set_success_flash(&mut sprite, false);
        body.set_global_position(save_point.position);
        body.set_visible(true);
        respawn_resource.player = None;
        die_label.set_visible(false);
    }
}

pub struct RespawnPlugin;

impl Plugin for RespawnPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<RespawnResource>()
            .add_systems(Update, proccess_respawn_system)
            .add_observer(on_respawn_event);
    }
}
