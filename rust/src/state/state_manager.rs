use bevy::prelude::*;
use godot::prelude::*;
use godot_bevy::{
    plugins::scene_tree::{SceneTreeMessage, SceneTreeMessageType},
    prelude::*,
};

use crate::state::scene_manager::SceneOperationMessage;

#[derive(Event, Debug, Clone)]
struct SceneChanged;

#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, Default, GodotConvert, Var, Export)]
#[godot(via= GString)]
pub enum SceneState {
    #[default]
    MainMenu,
    InGame,
    Pause,
}

impl SceneState {
    pub fn scene_path(&self) -> &'static str {
        match self {
            SceneState::MainMenu => "scenes/ui/main_menu.tscn",
            SceneState::InGame => "scenes/main.tscn",
            SceneState::Pause => "scenes/ui/pause_menu.tscn",
        }
    }
}

#[derive(Resource, Default)]
pub struct CurrentScene {
    pub scene: Option<SceneState>,
}

impl CurrentScene {
    pub fn set(&mut self, scene: SceneState) {
        self.scene = Some(scene);
    }
}

#[derive(Resource, Default)]
struct SceneLoadingState {
    pub loading_handle: Option<Handle<GodotResource>>,
}

#[derive(Resource, Default)]
struct PendingScene {
    pub scene_state: Option<SceneState>,
}

#[derive(Event, Debug, Clone)]
pub struct LoadSceneMessage {
    pub scene_state: SceneState,
}

#[derive(Event, Debug, Clone)]
pub struct SceneLoadedMessage {
    pub scene_state: SceneState,
}

#[derive(Resource, Default)]
struct SceneTreeSignalConnected(bool);

fn connect_scene_tree_signal(
    mut connected: ResMut<SceneTreeSignalConnected>,
    signals: GodotSignals<SceneChanged>,
    mut scene_tree: SceneTreeRef,
) {
    if connected.0 {
        return;
    }

    let tree = scene_tree.get().clone();
    signals.connect_object(tree, SceneTreeSignals::SCENE_CHANGED, |_args| {
        Some(SceneChanged)
    });

    connected.0 = true;

    godot_print!("Connected to SceneTree.scene_changed signal");
}

// fn on_screen_changed(_trigger: On<SceneChanged>) {
//     godot_print!("Scene changed!");
// }

fn on_load_level_request(
    trigger: On<LoadSceneMessage>,
    mut loading_state: ResMut<SceneLoadingState>,
    mut current_scene: ResMut<CurrentScene>,
    asset_server: Res<AssetServer>,
) {
    let event = trigger.event();

    let scene_handle: Handle<GodotResource> = asset_server.load(event.scene_state.scene_path());

    loading_state.loading_handle = Some(scene_handle);

    current_scene.set(event.scene_state);
}

fn handle_level_scene_changed(
    current_scene: Res<CurrentScene>,
    mut loading_state: ResMut<SceneLoadingState>,
    mut pending_scene: ResMut<PendingScene>,
    mut scene_events: MessageWriter<SceneOperationMessage>,
    mut assets: ResMut<Assets<GodotResource>>,
) {
    if let (Some(scene_state), Some(handle)) = (current_scene.scene, &loading_state.loading_handle)
        && assets.get_mut(handle).is_some()
    {
        scene_events.write(SceneOperationMessage::change_to_packed(handle.clone()));

        pending_scene.scene_state = Some(scene_state);

        loading_state.loading_handle = None;
    }
}

fn emit_level_loaded_event_when_scene_ready(
    mut pending_scene: ResMut<PendingScene>,
    mut scene_tree_events: MessageReader<SceneTreeMessage>,
    mut commands: Commands,
    mut godot: GodotAccess,
) {
    if let Some(scene_state) = pending_scene.scene_state {
        let expected_path = match scene_state {
            SceneState::MainMenu => "root/ui/main_menu",
            SceneState::InGame => "root/main",
            SceneState::Pause => "root/ui/pause_menu",
        };
        for event in scene_tree_events.read() {
            if let SceneTreeMessageType::NodeAdded = event.message_type
                && let Some(node) = godot.try_get::<Node>(event.node_id)
                && node.is_inside_tree()
            {
                let node_path = node.get_path().to_string();
                if node_path == expected_path {
                    commands.trigger(SceneLoadedMessage { scene_state });
                    pending_scene.scene_state = None;
                    break;
                }
            }
        }
    }
}

pub struct StateManagerPlugin;

impl Plugin for StateManagerPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentScene>()
            .init_resource::<PendingScene>()
            .init_resource::<SceneLoadingState>()
            .init_resource::<SceneTreeSignalConnected>()
            .add_plugins(GodotSignalsPlugin::<SceneChanged>::default())
            .add_observer(on_load_level_request)
            .add_systems(Startup, connect_scene_tree_signal)
            .add_systems(
                Update,
                (
                    (handle_level_scene_changed, ApplyDeferred).chain(),
                    emit_level_loaded_event_when_scene_ready,
                ),
            );
    }
}
