use bevy::{asset::LoadState, prelude::*};
use godot::{classes::class_macros::private::virtuals::ZipReader::{GString, Vector3}, global::godot_print};
use godot_bevy::prelude::*;

use crate::{
    characters::components::{identity::from_godot_string, state::Initialized},
    godot_utils::nodes::get_custom_area_3d,
    state::GameState, world::world_manager::WorldOperationMessage,
};

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = Area3D, class_name = TransportNode)]
#[gdbevy(
    require(target_scene: TargetScene, as = GString, with = from_godot_string, default = GString::new()),
)]
#[gdbevy(require(initialized:Initialized, as = bool, default = false))]
#[gdbevy(
    require(coordinates: Destination, as= Vector3, default= Vector3::ZERO)
)]
pub struct TransportLevelNode;

#[derive(Component)]
struct TargetScene(pub String);

#[derive(Component, Default, Debug, Clone)]
pub struct Destination(pub Vector3);

//Events
#[derive(Event, Debug, Clone)]
struct EnterLevelTransitionNode {
    scene_path: String,
    destination: Vector3,
}

#[derive(Resource, Default)]
pub struct PendingLevelChange {
    pub scene: Option<Handle<GodotResource>>,
    pub destination: Vector3,
}

fn initialized_target_level_node(
    query: Query<(&GodotNodeHandle, &mut Initialized, &TargetScene, &Destination), With<TransportLevelNode>>,
    mut godot: GodotAccess,
    signal_transition: GodotSignals<EnterLevelTransitionNode>,
) {
    for (handle, mut initialized, target_scene, destination) in query {
        if initialized.0 {
            continue;
        }

        let Some(area) = get_custom_area_3d(handle, &mut godot) else {
            continue;
        };

        let scene_path = target_scene.0.clone();
        let dest = destination.0;

        signal_transition.connect(
            area.into(),
            Area3DSignals::BODY_ENTERED,
            None,
            move |_args, _node_handle, _ent| {
                Some(EnterLevelTransitionNode {
                    scene_path: scene_path.clone(),
                    destination: dest,
                })
            },
        );

        initialized.0 = true;
    }
}

fn on_enter_level_transition(
    trigger: On<EnterLevelTransitionNode>,
    asset_server: Res<AssetServer>,
    mut pending: ResMut<PendingLevelChange>,
) {
    godot_print!("Enter level change {}", trigger.scene_path);

    pending.scene = Some(asset_server.load(trigger.scene_path.clone()));
    pending.destination = trigger.destination;
    
}

fn wait_for_level_load(
    asset_server: Res<AssetServer>,
    mut pending: ResMut<PendingLevelChange>,
    mut operations: MessageWriter<WorldOperationMessage>,
) {
    let Some(scene) = pending.scene.as_ref() else {
        return;
    };

    match asset_server.get_load_state(scene.id()) {
        Some(LoadState::Loaded) => {
            godot_print!("Level loaded successfully");

            operations.write(WorldOperationMessage::change_level(scene.clone(), pending.destination));

            pending.scene = None;
        }
        Some(LoadState::Failed(error)) => {
            godot_print!("Failed to load level: {:?}", error);

            pending.scene = None;
        }
        Some(LoadState::Loading) => {
            // Still loading; check again next frame.
        }
        _ => {}
    }
}

pub struct LevelTransitionPlugin;

impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingLevelChange>()
            .add_systems(Update, wait_for_level_load.run_if(in_state(GameState::Ingame3D)))
            .add_systems(
            Update,
            initialized_target_level_node.run_if(in_state(GameState::Ingame3D)),
        )
        .add_observer(on_enter_level_transition);
    }
}
