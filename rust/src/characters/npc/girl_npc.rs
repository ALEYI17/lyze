use bevy::prelude::*;
use godot::{
    classes::{
        Area3D, Label3D,
        class_macros::private::virtuals::{
            Xrvrs::Gd,
            ZipReader::{Array, GString},
        },
    },
    global::godot_print,
};
use godot_bevy::prelude::*;

use crate::{
    characters::components::{
        identity::{Name, from_godot_string},
        state::Initialized,
    },
    events::dialogue::{EnterDialogueZone, ExitDialogueZone},
    godot_utils::nodes::get_custom_area_3d,
    state::GameState,
};

// Components
#[derive(Component)]
pub struct DialogueText(pub Vec<String>);

// Custom Node
#[derive(Component, GodotNode, Default)]
#[gdbevy(base = Area3D, class_name = GirlNpc)]
#[gdbevy(require(initialized:Initialized, as = bool, default = false))]
#[gdbevy(require(npc_name:Name, as = GString, with = from_godot_string ,default = GString::from("girl")))]
#[gdbevy(require(npc_dialogue:DialogueText, as = Array<GString>, with = from_godot_array ,default = Array::new()))]
pub struct GirlNpcNode;

fn from_godot_array(value: Array<GString>) -> Vec<String> {
    let mut result = Vec::new();
    for i in 0..value.len() {
        if let Some(v) = value.get(i) {
            result.push(v.to_string());
        }
    }
    result
}

pub fn get_interacion_label(area: &Gd<Area3D>) -> Option<Gd<Label3D>> {
    let label_handle = area.get_node_or_null("Label3D")?;

    let Ok(label) = label_handle.try_cast::<Label3D>() else {
        return None;
    };

    Some(label)
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

pub struct GirlNpcPlugin;

impl Plugin for GirlNpcPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(
            Update,
            initialized_girl_npc.run_if(in_state(GameState::Ingame3D)),
        );
    }
}
