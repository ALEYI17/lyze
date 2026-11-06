use bevy::{asset::LoadState, prelude::*};
use godot::{
    classes::{
        Area3D, Label3D,
        class_macros::private::virtuals::{
            Xrvrs::Gd,
            ZipReader::{GString, Vector3},
        },
    },
    global::godot_print,
};
use godot_bevy::prelude::*;

use crate::{
    characters::components::{identity::from_godot_string, state::Initialized},
    godot_utils::nodes::get_custom_area_3d,
    state::GameState,
    world::world_manager::WorldOperationMessage,
};

use godot::classes::Input;

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
    node_handle: GodotNodeHandle,
}

#[derive(Event, Debug, Clone)]
struct ExitLevelTransitionNode {
    node_handle: GodotNodeHandle,
}

fn get_interaction_label(area: &Gd<Area3D>) -> Option<Gd<Label3D>> {
    let label_handle = area.get_node_or_null("Label3D")?;

    let Ok(label) = label_handle.try_cast::<Label3D>() else {
        return None;
    };

    Some(label)
}

#[derive(Resource, Default)]
pub struct PendingLevelChange {
    pub scene: Option<Handle<GodotResource>>,
    pub destination: Option<Vector3>,
}

fn initialized_target_level_node(
    query: Query<
        (
            &GodotNodeHandle,
            &mut Initialized,
            &TargetScene,
            &Destination,
        ),
        With<TransportLevelNode>,
    >,
    mut godot: GodotAccess,
    signal_transition: GodotSignals<EnterLevelTransitionNode>,
    signal_exit_transition: GodotSignals<ExitLevelTransitionNode>,
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
            area.clone().into(),
            Area3DSignals::BODY_ENTERED,
            None,
            move |_args, node_handle, _ent| {
                Some(EnterLevelTransitionNode {
                    scene_path: scene_path.clone(),
                    destination: dest,
                    node_handle,
                })
            },
        );

        signal_exit_transition.connect(
            area.into(),
            Area3DSignals::BODY_EXITED,
            None,
            move |_args, node_handle, _ent| Some(ExitLevelTransitionNode { node_handle }),
        );

        initialized.0 = true;
    }
}

fn on_enter_level_transition(
    trigger: On<EnterLevelTransitionNode>,
    asset_server: Res<AssetServer>,
    mut pending: ResMut<PendingLevelChange>,
    mut godot: GodotAccess,
) {
    godot_print!("Enter level change {}", trigger.scene_path);

    let Some(area) = get_custom_area_3d(&trigger.node_handle, &mut godot) else {
        godot_print!("dont get customa rea transport");
        return;
    };

    let Some(mut label) = get_interaction_label(&area) else {
        godot_print!("Could not get interaction label for transport");
        return;
    };

    label.set_visible(true);

    pending.scene = Some(asset_server.load(trigger.scene_path.clone()));
    pending.destination = Some(trigger.destination);
}

fn on_exit_level_transition(
    trigger: On<ExitLevelTransitionNode>,
    mut pending: ResMut<PendingLevelChange>,
    mut godot: GodotAccess,
) {
    godot_print!("Exit transport area");
    let Some(area) = get_custom_area_3d(&trigger.node_handle, &mut godot) else {
        return;
    };

    let Some(mut label) = get_interaction_label(&area) else {
        return;
    };

    label.set_visible(false);
    pending.scene = None;
    pending.destination = None;
}

fn wait_for_level_load(
    asset_server: Res<AssetServer>,
    mut pending: ResMut<PendingLevelChange>,
    mut operations: MessageWriter<WorldOperationMessage>,
    mut godot: GodotAccess,
) {
    let Some(scene) = pending.scene.as_ref() else {
        return;
    };
    let Some(destination) = pending.destination else {
        return;
    };

    let input = godot.singleton::<Input>();

    if input.is_action_just_pressed("interact") {
        match asset_server.get_load_state(scene.id()) {
            Some(LoadState::Loaded) => {
                godot_print!("Level loaded successfully");

                operations.write(WorldOperationMessage::change_level(
                    scene.clone(),
                    destination,
                ));

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
}

pub struct LevelTransitionPlugin;

impl Plugin for LevelTransitionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<PendingLevelChange>()
            .add_systems(
                Update,
                wait_for_level_load.run_if(in_state(GameState::Ingame3D)),
            )
            .add_systems(
                Update,
                initialized_target_level_node.run_if(in_state(GameState::Ingame3D)),
            )
            .add_observer(on_enter_level_transition)
            .add_observer(on_exit_level_transition);
    }
}
