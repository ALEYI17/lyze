use bevy::prelude::*;
use godot::classes::class_macros::private::virtuals::ZipReader::Vector2;

#[derive(Debug, Clone, Copy, Default, Reflect)]
pub struct ConvictionStats {
    pub attack: f32,
    pub defense: f32,
}

#[derive(Component, Debug, Clone, Default, Reflect)]
#[reflect(Component)]
pub struct Convictions {
    pub aeternum: ConvictionStats,
    pub alligatio: ConvictionStats,
    pub ananke: ConvictionStats,
    pub sekhem: ConvictionStats,
    pub ataraxia: ConvictionStats,
}

pub fn from_godot_conviction(value: Vector2) -> ConvictionStats {
    ConvictionStats {
        attack: value.x,
        defense: value.y,
    }
}
