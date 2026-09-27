use bevy::prelude::*;
use godot_bevy::prelude::*;

#[derive(Component, GodotNode, Default)]
#[gdbevy(base = CanvasLayer, class_name = DialogueHud)]
pub struct DialogueHudNode;
