use bevy::prelude::*;
use rand::Rng;
use crate::components::{Star, Velocity};

pub fn background_star_spawn_system(
    mut cmd: Commands
) {
    let mut rng = rand::rng();

    let spawn = rng.random_range(0..100);
    if spawn > 90 {
        let star_spawn_position = Vec3::new(rng.random_range(-240.0..240.0), 340.0, 0.0);
        let size  = rng.random_range(1.0..5.0);
        cmd.spawn((
            Star,
            Velocity { value: (size + 10.0) * 5.0 },
            Sprite::from_color(
                Color::LinearRgba(
                    LinearRgba { red: 0.7, green: 0.7, blue: 1.0, alpha: 1.0 }),
                    Vec2 { x: size, y: size }),
            Transform::from_translation(star_spawn_position),
            Name::new("Star")
        ));
    }
}

pub fn background_star_movement_system(
    time: Res<Time>,
    star_query: Query<(&Velocity, &mut Transform), With<Star>>
) {
    for (velocity, mut transform) in star_query {
        transform.translation.y -= velocity.value * time.delta_secs();
    }
}

pub fn background_star_despawn_system(
    mut cmd: Commands,
    star_query: Query<(Entity, &Transform), With<Star>>
) {
    for (star_entity, transform) in &star_query {
        if transform.translation.y < -320.0 {
            cmd.entity(star_entity).despawn();
        }
    }
}