use bevy::prelude::*;
pub mod dialogue;

pub struct InteractionPlugin;

impl Plugin for InteractionPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(dialogue::DialogueInteractionPlugin);
    }
    
}
