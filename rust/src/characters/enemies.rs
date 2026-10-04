use bevy::prelude::*;
mod ai;
mod cape_enemy;
pub mod cape_enemy_3d;
pub mod enemy_encounter;

pub struct EnemiesPlugin;

impl Plugin for EnemiesPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(cape_enemy::CapeEnemyPlugin)
            .add_plugins(cape_enemy_3d::CapeEnemy3DPlugin)
            .add_plugins(enemy_encounter::EnemyEncounterPlugin);
    }
}
