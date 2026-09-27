use bevy::prelude::*;
use godot::{
    classes::{
        Area3D, CanvasLayer, Input, Label, Label3D,
        class_macros::private::virtuals::ZipReader::{Array, GString},
    },
    global::godot_print,
};
use godot_bevy::prelude::*;

use crate::ui::dialogue_hud::DialogueHudNode;

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = Area3D, class_name = GirlNpc)]
#[gdbevy(require(initialized:NpcInitialized, as = bool, default = false))]
#[gdbevy(require(npc_name:Name, as = GString, with = from_godot_string ,default = GString::from("girl")))]
#[gdbevy(require(npc_dialogue:DialogueText, as = Array<GString>, with = from_godot_array ,default = Array::new()))]
pub struct GirlNpcNode;

#[derive(Event, Debug, Clone)]
struct EnterDialogueZone {
    entity: Option<Entity>,
}

#[derive(Event, Debug, Clone)]
struct ExitDialogueZone;

#[derive(Component, Default)]
struct NpcInitialized(bool);

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct DialogueText(pub Vec<String>);

#[derive(Resource, Default)]
struct CurrentInteraction {
    npc: Option<Entity>,
    current_line: usize,
    in_interaction: bool,
}

fn from_godot_string(value: GString) -> String {
    value.to_string()
}

fn from_godot_array(value: Array<GString>) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..value.len() {
        if let Some(v) = value.get(i) {
            result.push(v.to_string());
        }
    }
    result
}

fn initialized_girl_npc(
    query: Query<(Entity, &GodotNodeHandle, &mut NpcInitialized), With<GirlNpcNode>>,
    mut godot: GodotAccess,
    signal_dialogue: GodotSignals<EnterDialogueZone>,
    exit_dialogue: GodotSignals<ExitDialogueZone>,
) {
    for (entity, handle, mut initialized) in query {
        if initialized.0 {
            continue;
        }
        let Some(area) = godot.try_get::<Area3D>(*handle) else {
            godot_print!("Cannot cast girl npc");
            return;
        };

        signal_dialogue.connect(
            area.clone().into(),
            Area3DSignals::BODY_ENTERED,
            Some(entity),
            |_args, _node_handle, ent| Some(EnterDialogueZone { entity: ent }),
        );

        exit_dialogue.connect(
            area.into(),
            Area3DSignals::BODY_EXITED,
            Some(entity),
            |_args, _node_handle, _ent| Some(ExitDialogueZone),
        );

        initialized.0 = true;
    }
}

fn on_dialogue_enter(
    trigger: On<EnterDialogueZone>,
    query: Query<&GodotNodeHandle, With<GirlNpcNode>>,
    mut godot: GodotAccess,
    mut interaction: ResMut<CurrentInteraction>,
) {
    let Some(entity) = trigger.event().entity else{
        return;
    };

    let Ok(handle) = query.get(entity) else {
        return;
    };
    
    let Some(area) = godot.try_get::<Area3D>(*handle) else {
        return;
    };

    let Some(label_handle) = area.get_node_or_null("Label3D") else {
        return;
    };

    let Ok(mut label) = label_handle.try_cast::<Label3D>() else {
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
    _trigger: On<ExitDialogueZone>,
    query: Query<&GodotNodeHandle, With<GirlNpcNode>>,
    mut godot: GodotAccess,
) {
    for handle in query {
        let Some(area) = godot.try_get::<Area3D>(*handle) else {
            continue;
        };

        let Some(label_handle) = area.get_node_or_null("Label3D") else {
            continue;
        };

        let Ok(mut label) = label_handle.try_cast::<Label3D>() else {
            continue;
        };

        label.set_visible(false);
    }
}

fn start_interaction(
    mut godot: GodotAccess,
    mut interaction: ResMut<CurrentInteraction>,
    query_hud: Query<&GodotNodeHandle, With<DialogueHudNode>>,
    query_name: Query<(&Name, &DialogueText)>,
) {
    let input = godot.singleton::<Input>();

    let Ok(handle) = query_hud.single() else {
        return;
    };

    let Some(mut canvas) = godot.try_get::<CanvasLayer>(*handle) else {
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

        return;
    }

    let Some(speaker_handle) = canvas.get_node_or_null("speaker") else {
        return;
    };

    let Some(text_handle) = canvas.get_node_or_null("Text") else {
        return;
    };

    let Ok(mut speaker) = speaker_handle.try_cast::<Label>() else {
        return;
    };

    let Ok(mut text) = text_handle.try_cast::<Label>() else {
        return;
    };

    canvas.set_visible(true);

    speaker.set_text(&speaker_name.0);

    let Some(current_text) = dialogue_text.0.get(interaction.current_line) else {
        return;
    };

    text.set_text(current_text);
}

pub struct GirlNpcPlugin;

impl Plugin for GirlNpcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentInteraction>()
            .add_systems(Update, initialized_girl_npc)
            .add_systems(Update, start_interaction)
            .add_observer(on_dialogue_enter)
            .add_observer(on_dialogue_exit);
    }
}
