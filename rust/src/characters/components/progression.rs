use bevy::prelude::*;

#[derive(Component)]
pub struct Experience(pub u32);

#[derive(Component)]
pub struct Level(pub u32);

#[derive(Component)]
pub struct RewardedExperience(pub u32);
