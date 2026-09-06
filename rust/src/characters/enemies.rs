use bevy::prelude::*;
mod cape_enemy;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(cape_enemy::CapeEnemyPlugin);
    }
}
