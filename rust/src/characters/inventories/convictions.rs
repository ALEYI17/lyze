use bevy::prelude::*;

#[derive(Resource)]
pub struct ConvictionPoints {
    pub avaible_points: u32,
}

impl Default for ConvictionPoints {
    fn default() -> Self {
        ConvictionPoints { avaible_points: 4 }
    }
}

pub struct ConvictionPlugin;

impl Plugin for ConvictionPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<ConvictionPoints>();
    }
}
