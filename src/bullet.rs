use bevy::prelude::*;
use crate::components::{Bullet, AttackPower, Velocity, Player};

pub fn attack_system(
    mut cmd: Commands,
    keyboard: Res<ButtonInput<KeyCode>>,
    player_query: Query<&Transform, With<Player>>,
) {
    if keyboard.just_pressed(KeyCode::Space) {
        if let Ok(player_transform) = player_query.single() {
            let bullet_position = player_transform.translation + Vec3::new(0.0, 25.0, 0.0);
        
            cmd.spawn((
                Bullet,
                Sprite::from_color(Color::linear_rgb(0.5, 0.75, 0.21), Vec2 { x: 6.0, y: 10.0 }),
                AttackPower { value: 15.0 },
                Velocity { value: 500.0 },
                Transform::from_translation(bullet_position),
                Name::new("Bullet"),
            ));
        }
    }
}

pub fn bullet_movement_system(
    time: Res<Time>,
    mut bullet_query: Query<(&Velocity, &mut Transform), With<Bullet>>
) {
    for (velocity, mut transform) in &mut bullet_query {
            transform.translation.y += velocity.value * time.delta_secs();
        }
    }

pub fn bullet_despawn_system(
    mut cmd: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    for (entity, transform) in &bullet_query {
        if transform.translation.y > 320.0 {
            cmd.entity(entity).despawn();
        }
    }
}