use bevy::prelude::*;
use godot::{
    classes::{Node, PackedScene, class_macros::private::virtuals::ZipReader::Vector3},
    global::godot_print,
};
use godot_bevy::{
    interop::{GodotAccess, GodotNodeHandle},
    plugins::{assets::GodotResource, scene_tree::SceneTreeRef},
    prelude::NodeTreeView,
};

use crate::{
    characters::player_3d::Player3DNode, godot_utils::nodes::get_custom_character_body_3d,
    state::GameState,
};

#[derive(Message, Debug, Clone)]
pub struct WorldOperationMessage {
    pub scene: Handle<GodotResource>,
    pub destination: Vector3,
}
impl WorldOperationMessage {
    pub fn change_level(scene: Handle<GodotResource>, destination: Vector3) -> Self {
        Self { scene, destination }
    }
}

#[derive(Resource, Default)]
pub struct WorldAssets {
    pub world: Option<GodotNodeHandle>,
    pub level: Option<GodotNodeHandle>,
    pub initialized: bool,
}

#[derive(NodeTreeView)]
pub struct WorldNodes {
    #[node("/root/main_isometric/world")]
    pub world: GodotNodeHandle,
}

fn reset_world_assets(mut world_assets: ResMut<WorldAssets>) {
    world_assets.world = None;
    world_assets.level = None;
    world_assets.initialized = false;
}

fn initialized_world(
    mut world_assets: ResMut<WorldAssets>,
    mut scene_tree: SceneTreeRef,
    mut godot: GodotAccess,
) {
    if let Some(root) = scene_tree.get().get_root()
        && let Ok(world_nodes) = WorldNodes::from_node(root)
    {
        world_assets.world = Some(world_nodes.world);
        if let Some(world) = world_assets.world.as_ref()
            && let Some(world_node) = godot.try_get::<Node>(*world)
            && let Some(level) = world_node.get_child(0)
        {
            world_assets.level = Some(level.into())
        }
        world_assets.initialized = true;
    }
}

fn world_assets_not_initialized(world_assets: Res<WorldAssets>) -> bool {
    !world_assets.initialized
}

fn process_world_operation(
    mut operations: MessageReader<WorldOperationMessage>,
    mut assets: ResMut<Assets<GodotResource>>,
    mut world_assets: ResMut<WorldAssets>,
    query_player: Query<&GodotNodeHandle, With<Player3DNode>>,
    mut godot: GodotAccess,
) {
    for operation in operations.read() {
        godot_print!("receive message");
        let Some(world_handle) = world_assets.world else {
            godot_print!("continue1");
            continue;
        };

        let Some(current_level_handle) = world_assets.level else {
            godot_print!("continue2");
            continue;
        };

        let Some(mut resource) = assets.get_mut(&operation.scene) else {
            godot_print!("continue3");
            continue;
        };

        let Some(packed_scene) = resource.try_cast::<PackedScene>() else {
            godot_print!("continue4");
            continue;
        };

        let Some(mut world) = godot.try_get::<Node>(world_handle) else {
            godot_print!("continue5");
            continue;
        };

        let Some(mut current_level) = godot.try_get::<Node>(current_level_handle) else {
            godot_print!("continue6");
            continue;
        };

        let Some(new_level) = packed_scene.instantiate() else {
            godot_print!("continue7");
            continue;
        };

        let Ok(player_handle) = query_player.single() else {
            continue;
        };

        let Some(mut player) = get_custom_character_body_3d(player_handle, &mut godot) else {
            continue;
        };

        world.remove_child(&current_level);
        current_level.queue_free();

        world.add_child(&new_level);

        world_assets.level = Some(new_level.clone().into());

        player.set_global_position(operation.destination);
    }
}

pub struct WorldNodesPlugin;

impl Plugin for WorldNodesPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<WorldAssets>()
            .add_message::<WorldOperationMessage>()
            .add_systems(
                Update,
                (initialized_world.run_if(world_assets_not_initialized))
                    .run_if(in_state(GameState::Ingame3D)),
            )
            .add_systems(OnEnter(GameState::Ingame3D), reset_world_assets)
            .add_systems(
                Update,
                process_world_operation.run_if(in_state(GameState::Ingame3D)),
            );
    }
}
