use bevy::prelude::*;
use godot::{
    classes::{
        Area3D, Input, Label3D,
        class_macros::private::virtuals::{
            Xrvrs::Gd,
            ZipReader::{Array, GString},
        },
    },
    global::godot_print,
};
use godot_bevy::prelude::*;

use crate::{
    characters::components::stats::Initialized,
    state::GameState,
    ui::dialogue_hud::{
        DialogueHudNode, get_custom_canvas_layer, get_speaker_label, get_text_label,
    },
};

// Components

#[derive(Component)]
pub struct Name(pub String);

#[derive(Component)]
pub struct DialogueText(pub Vec<String>);

// Custom Node
#[derive(Component, GodotNode, Default)]
#[gdbevy(base = Area3D, class_name = GirlNpc)]
#[gdbevy(require(initialized:Initialized, as = bool, default = false))]
#[gdbevy(require(npc_name:Name, as = GString, with = from_godot_string ,default = GString::from("girl")))]
#[gdbevy(require(npc_dialogue:DialogueText, as = Array<GString>, with = from_godot_array ,default = Array::new()))]
pub struct GirlNpcNode;

//Events
#[derive(Event, Debug, Clone)]
struct EnterDialogueZone {
    entity: Option<Entity>,
}

#[derive(Event, Debug, Clone)]
struct ExitDialogueZone {
    entity: Option<Entity>,
}

// resources
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

fn get_interacion_label(area: &Gd<Area3D>) -> Option<Gd<Label3D>> {
    let label_handle = area.get_node_or_null("Label3D")?;

    let Ok(label) = label_handle.try_cast::<Label3D>() else {
        return None;
    };

    Some(label)
}

fn get_custom_area_3d(handle: &GodotNodeHandle, godot: &mut GodotAccess) -> Option<Gd<Area3D>> {
    let Some(area) = godot.try_get::<Area3D>(*handle) else {
        godot_print!("Cannot cast girl npc");
        return None;
    };
    Some(area)
}

fn initialized_girl_npc(
    query: Query<(Entity, &GodotNodeHandle, &mut Initialized), With<GirlNpcNode>>,
    mut godot: GodotAccess,
    signal_dialogue: GodotSignals<EnterDialogueZone>,
    exit_dialogue: GodotSignals<ExitDialogueZone>,
) {
    for (entity, handle, mut initialized) in query {
        if initialized.0 {
            continue;
        }

        let Some(area) = get_custom_area_3d(handle, &mut godot) else {
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
            |_args, _node_handle, ent| Some(ExitDialogueZone { entity: ent }),
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
    mut interaction: ResMut<CurrentInteraction>,
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

fn start_interaction(
    mut godot: GodotAccess,
    mut interaction: ResMut<CurrentInteraction>,
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

pub struct GirlNpcPlugin;

impl Plugin for GirlNpcPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<CurrentInteraction>()
            .add_systems(
                Update,
                initialized_girl_npc.run_if(in_state(GameState::Ingame3D)),
            )
            .add_systems(
                Update,
                start_interaction.run_if(
                    in_state(GameState::Ingame3D).or_eager(in_state(GameState::Indialogue)),
                ),
            )
            .add_observer(on_dialogue_enter)
            .add_observer(on_dialogue_exit);
    }
}
