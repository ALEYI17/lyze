use bevy::prelude::*;

#[derive(Component, Eq, PartialEq, Default, Debug, Clone, Copy)]
pub enum EnemyState {
    #[default]
    Patrol,
    Chase,
    Attack,
}

impl EnemyState {
    pub fn is_patrolling(&self) -> bool {
        matches!(self, EnemyState::Patrol)
    }

    pub fn is_chasing(&self) -> bool {
        matches!(self, EnemyState::Chase)
    }

    pub fn is_attacking(&self) -> bool {
        matches!(self, EnemyState::Attack)
    }

    pub fn change_to(&mut self, new_state: EnemyState) {
        *self = new_state;
    }
}

#[derive(Component)]
pub struct PatrolTimer(pub Timer);

#[derive(Component)]
pub struct PatrolInterval(pub f32);

#[derive(Component)]
pub struct AttackTimer(pub Timer);

#[derive(Component)]
pub struct AttackInterval(pub f32);

#[derive(Component)]
pub struct AttackCooldownInterval(pub f32);

#[derive(Component)]
pub struct AttackCooldownTimer(pub Timer);
