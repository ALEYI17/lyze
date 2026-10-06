use godot::{classes::{Area3D, CanvasLayer, CharacterBody3D}, prelude::*};
use godot_bevy::interop::{GodotAccess, GodotNodeHandle};

pub fn get_custom_character_body_3d(
    handle: &GodotNodeHandle,
    godot: &mut GodotAccess,
) -> Option<Gd<CharacterBody3D>> {
    let body = godot.try_get::<CharacterBody3D>(*handle)?;
    Some(body)
}

pub fn get_custom_area_3d(handle: &GodotNodeHandle, godot: &mut GodotAccess) -> Option<Gd<Area3D>> {
    let area = godot.try_get::<Area3D>(*handle)?;
    Some(area)
}

pub fn get_custom_canvas_layer(
    handle: &GodotNodeHandle,
    godot: &mut GodotAccess,
) -> Option<Gd<CanvasLayer>> {
    let canvas = godot.try_get::<CanvasLayer>(*handle)?;
    Some(canvas)
}



