use bevy::prelude::*;

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

fn main() {
    App::new()
        .add_plugins(DefaultPlugins)
        .add_systems(Startup, setup)
        .add_systems(Update, (movement_system, attack_system))
        .run();
}

fn setup(mut cmd: Commands) {
    // инициализация игрока
    cmd.spawn((
        Player,
        Health { value: 100.0 },
        AttackPower { value: 15.0 },
        Velocity { value: 50.0 },
        Position { x: 0.0, y: 0.0 },
        Name::new("Player"),
    ));

    // инициализация врага
    cmd.spawn((
        Enemy,
        Health { value: 30.0 },
        AttackPower { value: 15.0 },
        Velocity { value: 20.0 },
        Position { x: 0.0, y: 500.0 },
        Name::new("Jet"),
    ));
}

fn movement_system(
    time: Res<Time>,
    keyboard: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&Velocity, &mut Position), With<Player>>,
) {
    if let Ok((speed, mut position)) = player_query.single_mut() {
        let delta = time.delta_secs();

        if keyboard.pressed(KeyCode::KeyA) {
            position.x -= speed.value * delta;
        }
        if keyboard.pressed(KeyCode::KeyS) {
            position.y -= speed.value * delta;
        }
        if keyboard.pressed(KeyCode::KeyW) {
            position.y += speed.value * delta;
        }
        if keyboard.pressed(KeyCode::KeyD) {
            position.x += speed.value * delta;
        }
    }
}

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