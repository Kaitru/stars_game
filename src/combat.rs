use bevy::prelude::*;
use crate::components::{Bullet, Enemy, Health, AttackPower};

pub fn bullet_hit_system(
    mut cmd: Commands,
    mut enemy_query: Query<(Entity, &mut Health, &Transform), With<Enemy>>,
    bullet_query: Query<(Entity, &Transform, &AttackPower), With<Bullet>>
) {
    for (enemy_entity, mut health, enemy_transform) in enemy_query.iter_mut() {
        for (bullet_entity, bullet_transform, attack_power) in bullet_query.iter() {
            let distance = enemy_transform.translation.distance(bullet_transform.translation);

            if distance < 20.0 {
                health.value -= attack_power.value;

                cmd.entity(bullet_entity).despawn();

                if health.value <= 0.0 {
                    cmd.entity(enemy_entity).despawn();
                }
                
                break;
            }
        }
    }
}