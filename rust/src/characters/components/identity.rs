use bevy::prelude::*;
use godot::classes::class_macros::private::virtuals::ZipReader::GString;

#[derive(Component)]
pub struct Name(pub String);

pub fn from_godot_string(value: GString) -> String {
    value.to_string()
}
