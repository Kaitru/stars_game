use bevy::{prelude::*, window::WindowResolution};

mod components;
mod player;
mod enemy;
mod bullet;
mod combat;
mod background;

use enemy::*;
use player::*;
use bullet::*;
use combat::*;
use background::*;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins
            .set(WindowPlugin{
                primary_window : Some(
                    Window {
                        title : String::from("StarsGame"),
                        position : WindowPosition::Centered(MonitorSelection::Current),
                        resolution : WindowResolution::new(480.0, 640.0),
                    ..Default::default()
                    }
                ),..Default::default()
            }
        ).set(ImagePlugin::default_nearest())
        )
        .insert_resource(
            ClearColor(
                Color::LinearRgba(
                    LinearRgba { red: 0.005, green: 0.005, blue: 0.01, alpha: 1.0 }
                )
            )
        )
        .add_systems(Startup, (setup, player_setup))
        .add_systems(FixedUpdate, (
            background_star_spawn_system,
            background_star_movement_system,
            movement_system,
            attack_system,
            bullet_movement_system,
            bullet_despawn_system,
            enemy_spawn_system,
            bullet_hit_system,
            enemy_movement_system,
            enemy_despawn_system,
            background_star_despawn_system
        ))
        .run();
}

fn setup( mut cmd: Commands ) {
    cmd.spawn(Camera2d::default());
}