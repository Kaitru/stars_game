use bevy::prelude::*;
use rand::Rng;
use crate::components::{Enemy, Health, Velocity, Player};

// TODO: make a sprite for the enemy
//       make different types of the enemies
//       make enemy dying when got shot
//       make different types of enemies
pub fn enemy_spawn_system(
    mut cmd: Commands,
    asset_server: Res<AssetServer>
) {
    let mut rng = rand::rng();

    let spawn = rng.random_range(0..1000);
    // println!("{}", spawn);

    if spawn > 990 {
        let enemy_spawn_position = Vec3::new(rng.random_range(-240.0..240.0), 340.0, 0.0);

        cmd.spawn((
            Enemy,
            Health { value: 30.0 },
            Velocity { value: 50.0 },
            Sprite::from_color(
                Color::LinearRgba(
                    LinearRgba { red: 1.0, green: 1.0, blue: 1.0, alpha: 1.0 }), 
                    Vec2 { x: 15.0, y: 15.0 }),
            Transform::from_translation(enemy_spawn_position),
            Name::new("Jet")
        ));
    }
}

pub fn enemy_movement_system(
    time: Res<Time>,
    mut enemy_query: Query<(&Velocity, &mut Transform), With<Enemy>>) {
        for (velocity, mut transform) in &mut enemy_query {
            transform.translation.y -= velocity.value * time.delta_secs();
        }
}

pub fn enemy_despawn_system(
    mut cmd: Commands,
    enemy_query: Query<(Entity, &Transform), With<Enemy>>,
    mut player_query: Query<(Entity, &mut Health), With<Player>>
) {
    for (entity, transform) in &enemy_query {
        if transform.translation.y < -320.0 {
            cmd.entity(entity).despawn();
            // println!("Enemy despawned");
            if let Ok((player_entity, mut health)) = player_query.single_mut() {
                health.value -= 10.0;
                if health.value <= 0.0 {
                    cmd.entity(player_entity).despawn();
                }
            }
        }
    }
}