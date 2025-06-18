// TODO: make modules and refactor them so the code isn't that shitty one file
//       make enemy dying when got shot
//       make different types of enemies
use bevy::{prelude::*, window::WindowResolution};

mod components;
mod player;
mod enemy;
mod bullet;

use enemy::*;
use player::*;
use bullet::*;

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
        .add_systems(Startup, setup_camera)
        .add_systems(Update, (
            player_setup,
            movement_system,
            attack_system,
            bullet_movement_system,
            bullet_despawn_system,
            enemy_spawn_system,
            enemy_movement_system
        ))
        .run();
}

fn setup_camera( mut cmd: Commands ) {
    cmd.spawn(Camera2d::default());
}