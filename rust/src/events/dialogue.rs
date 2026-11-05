use bevy::prelude::*;
use godot::{classes::Label3D, global::godot_print};
use godot_bevy::interop::{GodotAccess, GodotNodeHandle};

use crate::{
    characters::npc::girl_npc::{ GirlNpcNode, get_interacion_label},
    godot_utils::nodes::get_custom_area_3d, interactions::dialogue::CurrentDialogueInteraction,
};

//Events
#[derive(Event, Debug, Clone)]
pub struct EnterDialogueZone {
    pub entity: Option<Entity>,
}

#[derive(Event, Debug, Clone)]
pub struct ExitDialogueZone {
    pub entity: Option<Entity>,
}

fn on_dialogue_enter(
    trigger: On<EnterDialogueZone>,
    query: Query<&GodotNodeHandle, With<GirlNpcNode>>,
    mut godot: GodotAccess,
    mut interaction: ResMut<CurrentDialogueInteraction>,
) {
    let Some(entity) = trigger.event().entity else {
        return;
    };

    let Ok(handle) = query.get(entity) else {
        return;
    };

    let Some(area) = get_custom_area_3d(handle, &mut godot) else {
        return;
    };

    let Some(mut label) = get_interacion_label(&area) else {
        return;
    };

    if !interaction.in_interaction {
        interaction.npc = Some(entity);
        interaction.current_line = 0;
        interaction.in_interaction = false;
    }

    label.set_visible(true);
    godot_print!("Player enter dialogue zone");
}

fn on_dialogue_exit(
    trigger: On<ExitDialogueZone>,
    query: Query<&GodotNodeHandle, With<GirlNpcNode>>,
    mut godot: GodotAccess,
    mut interaction: ResMut<CurrentDialogueInteraction>,
) {
    let Some(entity) = trigger.event().entity else {
        return;
    };

    let Ok(handle) = query.get(entity) else {
        return;
    };

    let Some(area) = get_custom_area_3d(handle, &mut godot) else {
        return;
    };

    let Some(label_handle) = area.get_node_or_null("Label3D") else {
        return;
    };

    let Ok(mut label) = label_handle.try_cast::<Label3D>() else {
        return;
    };

    if !interaction.in_interaction {
        interaction.npc = None;
        interaction.current_line = 0;
    }
    label.set_visible(false);
}

pub struct DialogueEventPlugin;

impl Plugin for DialogueEventPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(on_dialogue_exit)
            .add_observer(on_dialogue_enter);
    }
}
