use bevy::prelude::*;
use godot::classes::class_macros::private::virtuals::ZipReader::GString;

#[derive(Debug, Clone, Copy, Default)]
pub enum EnemyPersonality {
    Aggressive,
    Defensive,
    #[default]
    Neutral,
    Crazy,
}

#[derive(Component, Debug, Clone, Copy, Default)]
pub struct EnemyPersonalityComponent(pub EnemyPersonality);

#[derive(Debug, Clone, Copy)]
pub struct EnemyPersonalityStats {
    pub aggression: f32,
    pub defense: f32,
}

impl EnemyPersonalityStats {
    pub fn new(aggression: f32, defense: f32) -> Self {
        Self {
            aggression: aggression.clamp(0.0, 5.0),
            defense: defense.clamp(0.0, 5.0),
        }
    }
}
impl From<EnemyPersonality> for EnemyPersonalityStats {
    fn from(value: EnemyPersonality) -> Self {
        match value {
            EnemyPersonality::Aggressive => Self::new(4.5, 2.4),

            EnemyPersonality::Defensive => Self::new(2.5, 4.0),

            EnemyPersonality::Neutral => Self::new(3.0, 3.0),

            EnemyPersonality::Crazy => Self::new(4.0, 2.0),
        }
    }
}

impl From<GString> for EnemyPersonality {
    fn from(value: GString) -> Self {
        match value.to_string().trim().to_ascii_lowercase().as_str() {
            "aggressive" => Self::Aggressive,
            "defensive" => Self::Defensive,
            "crazy" => Self::Crazy,
            _ => Self::Neutral,
        }
    }
}

pub fn from_godot_personality(value: GString) -> EnemyPersonality  {
    value.into()
}
