use bevy::prelude::*;

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Speed(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct JumpVelocity(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Gravity(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Health(pub f32);

#[derive(Component, Default, Debug, Clone, Reflect)]
#[reflect(Component)]
pub struct Damage(pub f32);

#[derive(Component, Default)]
pub struct Direction(pub f32);
