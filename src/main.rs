// TODO: make modules and refactor them so the code isn't that shitty one file
//       make enemy dying when got shot
//       make different types of enemies
use bevy::{prelude::*, window::WindowResolution};
use rand::Rng;

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health { value: f32 }

#[derive(Component)]
struct AttackPower { value: f32 }

#[derive(Component)]
struct Velocity { value: f32 }

#[derive(Component)]
struct Bullet;

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
        .add_systems(Startup, setup)
        .add_systems(Update, (
            movement_system,
            attack_system,
            bullet_movement_system,
            bullet_despawn_system,
            enemy_spawn_system,
            enemy_movement_system
        ))
        .run();
}

fn setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    ) {
    cmd.spawn(Camera2d::default());

    // player inizialization
    cmd.spawn((
        Player,
        Health { value: 100.0 },
        AttackPower { value: 15.0 },
        Velocity { value: 50.0 },
        Sprite::from_image(asset_server.load("player.png")),
        Name::new("Player"),
    ));
}

fn movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Velocity, &mut Transform), With<Player>>,
) {
    if let Ok((speed, mut transform)) = player_query.single_mut() {
        let delta = time.delta_secs();
        let mut direction = Vec3::ZERO;

        if keyboard.pressed(KeyCode::KeyA) {
            direction.x -= speed.value;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= speed.value;
        }
        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += speed.value;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += speed.value;
        }

        if direction.length() > 0.0 {
            transform.translation += direction.normalize() * speed.value * delta
        }
    }
}

fn attack_system(
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
                Velocity { value: 100.0 },
                Transform::from_translation(bullet_position),
                Name::new("Bullet"),
            ));
        }
    }
}

fn bullet_movement_system(
    time: Res<Time>,
    mut bullet_query: Query<(&Velocity, &mut Transform), With<Bullet>>
) {
    for (velocity, mut transform) in &mut bullet_query {
            transform.translation.y += velocity.value * time.delta_secs();
        }
    }

fn bullet_despawn_system(
    mut cmd: Commands,
    bullet_query: Query<(Entity, &Transform), With<Bullet>>,
) {
    let max_height = 320.0;
    
    for (entity, transform) in &bullet_query {
        if transform.translation.y > max_height {
            cmd.entity(entity.entity()).despawn();
        }
    }
}

// TODO: make a sprite for the enemy
//       make different types of the enemies
fn enemy_spawn_system(
    mut cmd: Commands,
    asset_server: Res<AssetServer>
) {
    let mut rng = rand::rng();

    let spawn = rng.random_range(0..1000);
    println!("{}", spawn);

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

fn enemy_movement_system(
    time: Res<Time>,
    mut enemy_query: Query<(&Velocity, &mut Transform), With<Enemy>>) {
        for (velocity, mut transform) in &mut enemy_query {
            transform.translation.y -= velocity.value * time.delta_secs();
        }
}