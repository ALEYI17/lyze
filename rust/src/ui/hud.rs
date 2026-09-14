use bevy::prelude::*;
use godot::classes::Label;
use godot_bevy::prelude::*;
use godot::prelude::*;
use crate::characters::players::PlayerNode;

use crate::characters::components::stats::Health;
use crate::state::GameState;
#[derive(Resource, Default)]
struct HudAssets {
    pub player_health: Option<GodotNodeHandle>,
    pub initialized: bool,
}

#[derive(NodeTreeView)]
struct HudUi {
    #[node("/root/Node2D/Hud/Control/player_healt")]
    pub health: GodotNodeHandle,
}

fn reset_hud_assets(mut hud_assets: ResMut<HudAssets>){
    hud_assets.player_health = None;
    hud_assets.initialized = false;
}

fn initialized_hud(mut hud_assets: ResMut<HudAssets>, mut scene_tree: SceneTreeRef) {
    if let Some(root) = scene_tree.get().get_root(){
        match HudUi::from_node(root) {
            Ok(hud_ui) => {
                godot_print!("Found hud");
                hud_assets.player_health = Some(hud_ui.health);
                hud_assets.initialized = true;
            }
            Err(_) => {}
        }
    }else {
        godot_print!("Not found hud");
    }
}

fn set_player_health(hud_assets: ResMut<HudAssets>, query: Query<&Health, With<PlayerNode>>, mut godot: GodotAccess) {

    let Ok(health) = query.single() else {
        return;
    };

    let Some(health_label) = hud_assets.player_health else {
        godot_print!("Can not find health label");
        return;
    };

    let Some(mut node) = godot.try_get::<Label>(health_label) else{
        godot_print!("Can not parse health label");
        return;
    };

    let text = format!("Health: {}", &health.0);
    node.set_text(&text);
}

fn hud_is_not_initialized(hud_assets: Res<HudAssets>) -> bool{
    !hud_assets.initialized
}

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<HudAssets>()
            .add_systems(Update, set_player_health)
            .add_systems(Update, initialized_hud.run_if(hud_is_not_initialized))
            .add_systems(OnEnter(GameState::InGame), reset_hud_assets);
    }
}
