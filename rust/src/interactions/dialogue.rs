use bevy::prelude::*;
use godot_bevy::interop::{GodotAccess, GodotNodeHandle};

use crate::{
    characters::{components::identity::Name, npc::girl_npc::DialogueText},
    godot_utils::nodes::get_custom_canvas_layer,
    state::GameState,
    ui::dialogue_hud::{DialogueHudNode, get_speaker_label, get_text_label},
};

use godot::classes::Input;
// resources
#[derive(Resource, Default)]
pub struct CurrentDialogueInteraction {
    pub npc: Option<Entity>,
    pub current_line: usize,
    pub in_interaction: bool,
}

fn start_interaction(
    mut godot: GodotAccess,
    mut interaction: ResMut<CurrentDialogueInteraction>,
    query_hud: Query<&GodotNodeHandle, With<DialogueHudNode>>,
    query_name: Query<(&Name, &DialogueText)>,
    mut app_state: ResMut<NextState<GameState>>,
) {
    let input = godot.singleton::<Input>();

    let Ok(handle) = query_hud.single() else {
        return;
    };

    let Some(mut canvas) = get_custom_canvas_layer(handle, &mut godot) else {
        return;
    };

    let Some(npc_entity) = interaction.npc else {
        return;
    };

    if !interaction.in_interaction {
        if !input.is_action_just_pressed("interact") {
            return;
        }
        interaction.in_interaction = true;
        interaction.current_line = 0;
        app_state.set(GameState::Indialogue);
    } else if input.is_action_just_pressed("enter") {
        interaction.current_line += 1;
    }

    let Ok((speaker_name, dialogue_text)) = query_name.get(npc_entity) else {
        return;
    };

    if interaction.current_line >= dialogue_text.0.len() {
        interaction.in_interaction = false;
        interaction.current_line = 0;
        interaction.npc = None;

        canvas.set_visible(false);
        app_state.set(GameState::Ingame3D);

        return;
    }

    let Some(mut speaker) = get_speaker_label(&canvas) else {
        return;
    };

    let Some(mut text) = get_text_label(&canvas) else {
        return;
    };

    canvas.set_visible(true);

    speaker.set_text(&speaker_name.0);

    let Some(current_text) = dialogue_text.0.get(interaction.current_line) else {
        return;
    };

    text.set_text(current_text);
}

pub struct DialogueInteractionPlugin;

impl Plugin for DialogueInteractionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentDialogueInteraction>()
            .add_systems(
                Update,
                start_interaction.run_if(
                    in_state(GameState::Ingame3D).or_eager(in_state(GameState::Indialogue)),
                ),
            );
    }
}
