use bevy::prelude::*;
use rand::Rng;
use crate::components::{Enemy, Health, Velocity};

// TODO: make a sprite for the enemy
//       make different types of the enemies
pub fn enemy_spawn_system(
    mut cmd: Commands,
    asset_server: Res<AssetServer>
) {
    let mut rng = rand::rng();

    let spawn = rng.random_range(0..1000);
    // println!("{}", spawn);

    if spawn > 980 {
        let enemy_spawn_position = Vec3::new(rng.random_range(-300.0..300.0), 340.0, 0.0);

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