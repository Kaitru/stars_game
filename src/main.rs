use bevy::{prelude::*, window::WindowResolution};

#[derive(Component)]
struct Player;

#[derive(Component)]
struct Enemy;

#[derive(Component)]
struct Health { value: f32 }

#[derive(Component)]
struct AttackPower { value: f32 }

#[derive(Component)]
struct Position { x: f32, y: f32 }

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
        .add_systems(Update, (movement_system, attack_system))
        .run();
}

fn setup(
    mut cmd: Commands,
    asset_server: Res<AssetServer>,
    ) {
    // TODO: make actual game camera and render
    cmd.spawn(Camera2d::default());

    // player inizialization
    cmd.spawn((
        Player,
        Health { value: 100.0 },
        AttackPower { value: 15.0 },
        Velocity { value: 50.0 },
        Sprite::from_image(
            asset_server.load("player.png")
        ),
        Name::new("Player"),
    ));

    // enemy inizialzation
    cmd.spawn((
        Enemy,
        Health { value: 30.0 },
        AttackPower { value: 15.0 },
        Velocity { value: 20.0 },
        Name::new("Jet"),
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
            direction.x -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            direction.y -= 1.0;
        }
        if keyboard.pressed(KeyCode::KeyW) {
            direction.y += 1.0;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            direction.x += 1.0;
        }

        if direction.length() > 0.0 {
            transform.translation += direction.normalize() * speed.value * delta
        }
    }
}

// TODO: rewrite to make it match gameplay
fn attack_system(
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&AttackPower, &Position), With<Player>>,
    mut enemy_query: Query<(&mut Health, &Position), With<Enemy>>,
) {
    if !keyboard.just_pressed(KeyCode::Space) {
        return;
    }

    let (player_power, player_pos) = match player_query.single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };

    let (mut enemy_health, enemy_pos) = match enemy_query.single_mut() {
        Ok(data) => data,
        Err(_) => return,
    };

    let dx = player_pos.x - enemy_pos.x;
    let dy = player_pos.y - enemy_pos.y;

    let distance = (dx * dx + dy * dy).sqrt();

    if distance < 100.0 {
        enemy_health.value -= player_power.value;
        println!("Enemy health: {}", enemy_health.value);
    }
}