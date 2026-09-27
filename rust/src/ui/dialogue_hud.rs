use bevy::prelude::*;
use godot::classes::{CanvasLayer, Label, class_macros::private::virtuals::Xrvrs::Gd};
use godot_bevy::prelude::*;

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = CanvasLayer, class_name = DialogueHud)]
pub struct DialogueHudNode;

pub fn get_custom_canvas_layer(
    handle: &GodotNodeHandle,
    godot: &mut GodotAccess,
) -> Option<Gd<CanvasLayer>> {
    let Some(canvas) = godot.try_get::<CanvasLayer>(*handle) else {
        return None;
    };
    Some(canvas)
}
pub fn get_speaker_label(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let Some(speaker_handle) = canvas.get_node_or_null("Panel/speaker") else {
        return None;
    };

    let Ok(speaker) = speaker_handle.try_cast::<Label>() else {
        return None;
    };

    Some(speaker)
}

pub fn get_text_label(canvas: &Gd<CanvasLayer>) -> Option<Gd<Label>> {
    let Some(speaker_handle) = canvas.get_node_or_null("Panel/Text") else {
        return None;
    };

    let Ok(speaker) = speaker_handle.try_cast::<Label>() else {
        return None;
    };

    Some(speaker)
}
