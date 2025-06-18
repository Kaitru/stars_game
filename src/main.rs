use bevy::{prelude::*, window::WindowResolution};

mod components;
mod player;
mod enemy;
mod bullet;
mod combat;

use enemy::*;
use player::*;
use bullet::*;
use combat::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin{
                primary_window : Some(Window{
                    title : String::from("StarsGame"),
                    position : WindowPosition::Centered(MonitorSelection::Current),
                    resolution : WindowResolution::new(480.0, 640.0),
                    ..Default::default()
                }),..Default::default()
            })
            .set(ImagePlugin::default_nearest())
        )
        .add_systems(Startup, (setup_camera, player_setup))
        .add_systems(Update, (
            movement_system,
            attack_system,
            bullet_movement_system,
            bullet_despawn_system,
            enemy_spawn_system,
            bullet_hit_system, // Добавьте эту систему
            enemy_movement_system,
            enemy_despawn_system
        ))
        .run();
}

fn setup_camera( mut cmd: Commands ) {
    cmd.spawn(Camera2d::default());
}