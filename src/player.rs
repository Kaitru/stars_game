use bevy::prelude::*;
use crate::components::{Player, Health, AttackPower, Velocity};

pub fn player_setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
) {
    // player inizialization
    cmd.spawn((
        Player,
        Health { value: 100.0 },
        AttackPower { value: 15.0 },
        Velocity { value: 150.0 },
        Sprite::from_image(asset_server.load("player.png")),
        // Transform::from_translation(translation)
        Name::new("Player"),
    ));
}

pub fn movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    if let Ok((speed, mut transform)) = player_query.single_mut() {
        let delta = time.delta_secs();
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }

        if direction.length() > 0.0 {
            transform.translation += direction.normalize() * speed.value * delta;
        }
    }
}