#[derive(Debug, Clone, Copy)]
pub struct CombatContext {
    pub hp_ratio: f32,
}

impl CombatContext {
    pub fn new(hp_ratio: f32) -> Self {
        Self {
            hp_ratio: hp_ratio.clamp(0.0, 1.0),
        }
    }

    pub fn from_health(health: f32, max_health: f32) -> Self {
        let hp_ratio = if max_health > 0.0 {
            health / max_health
        } else {
            0.0
        };

        Self::new(hp_ratio)
    }
}
