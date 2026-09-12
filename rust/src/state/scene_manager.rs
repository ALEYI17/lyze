use bevy::prelude::*;
use godot_bevy::prelude::*;

#[derive(Message, Debug)]
pub enum SceneOperationMessage {
    ChangeToPacked { scene: Handle<GodotResource> },
}

fn proccess_scene_operation(
    mut scene_tree: SceneTreeRef,
    mut operation_events: MessageReader<SceneOperationMessage>,
    mut assets: ResMut<Assets<GodotResource>>,
) {
    for operation in operation_events.read() {
        match operation {
            SceneOperationMessage::ChangeToPacked { scene } => {
                if let Some(mut godot_resource) = assets.get_mut(scene) {
                    if let Some(packed_scene) =
                        godot_resource.try_cast::<godot::classes::PackedScene>()
                    {
                        scene_tree.get().change_scene_to_packed(&packed_scene);
                    } else {
                        warn!("SceneManager: Resource is not a PackedScene");
                    }
                } else {
                    warn!("SceneManager: PackedScene asset not found or not loaded");
                }
            }
        }
    }
}

impl SceneOperationMessage {
    pub fn change_to_packed(scene: Handle<GodotResource>) -> Self {
        Self::ChangeToPacked { scene }
    }
}

pub struct SceneManagerPlugin;

impl Plugin for SceneManagerPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SceneOperationMessage>()
            .add_systems(Update, proccess_scene_operation);
    }
}
