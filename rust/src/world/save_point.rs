use bevy::prelude::*;
use godot::classes::class_macros::private::virtuals::ZipReader::Vector3;

use crate::persistent::save::SaveRequestedEvent;

#[derive(Resource)]
pub struct SavePoint {
    pub scene_path: Option<String>,
    pub position: Vector3,
}

impl Default for SavePoint {
    fn default() -> Self {
        Self {
            scene_path: Some("res://scenes/levels/level_1.tscn".to_string()),
            position: Vector3 {
                x: 2.411,
                y: 0.768,
                z: 5.353,
            },
        }
    }
}

#[derive(Event, Debug)]
pub struct ChangeSavepointEvent {
    pub scene_path: String,
    pub position: Vector3,
}

fn on_change_save_point_event(
    trigger: On<ChangeSavepointEvent>,
    mut save_point: ResMut<SavePoint>,
    mut commands: Commands,
) {
    save_point.scene_path = Some(trigger.scene_path.clone());
    save_point.position = trigger.position;

    commands.trigger(SaveRequestedEvent);
}

pub struct SavePointPlugin;

impl Plugin for SavePointPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<SavePoint>()
            .add_observer(on_change_save_point_event);
    }
}
