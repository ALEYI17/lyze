use bevy::prelude::*;

#[derive(Component, Eq, PartialEq, Default, Debug, Clone, Copy)]
pub enum EnemyState {
    #[default]
    Patrol,
    #[allow(dead_code)]
    Chase,
    #[allow(dead_code)]
    Attack,
}

#[derive(Component)]
pub struct PatrolTimer(pub Timer);

#[derive(Component)]
pub struct PatrolInterval(pub f32);
