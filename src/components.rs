use bevy::prelude::*;

#[derive(Component)]
pub struct Player;

#[derive(Component)]
pub struct Enemy;

#[derive(Component)]
pub struct Health { pub value: f32 }

#[derive(Component)]
pub struct AttackPower { pub value: f32 }

#[derive(Component)]
pub struct Velocity { pub value: f32 }

#[derive(Component)]
pub struct Bullet;