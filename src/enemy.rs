use bevy::prelude::*;
use rand::rng;
use rand::Rng;

//pieces needed
use crate::player::Player;
use crate::restart;

//consts
//order is -> basic, fast, tank
pub const ENEMY_SIZE: f32 = 64.0;
pub const ENEMY_SPEEDS: [f32; 3] = [150.0, 300.0, 100.0];

pub struct EnemyPlugin;
impl Plugin for EnemyPlugin{

    fn build(&self, app: &mut App){
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Update, spawn_enemies.run_if(in_state(restart::GameState::Playing)));
        app.add_systems(Update, move_enemies_toward_player.run_if(in_state(restart::GameState::Playing)));
    }
}

#[derive(Component, Clone, Copy)]
enum EnemyType {
    Basic,
    Fast,
    Tank,
}

#[derive(Resource)]
struct EnemySpawnTimer(Timer);

#[derive(Component)]
pub struct Enemy {
    // kind: EnemyType,
    pub health: i32,
    pub size: f32,
    pub damage: f32,
    pub speed: f32,
}



fn spawn_enemies(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<EnemySpawnTimer>,
    player_q: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>//for player asset
) {
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }

    let Ok(player) = player_q.single() else {
        return;
    };

    let mut rng = rng();
    let angle = rng.random_range(0.0..=std::f32::consts::TAU);
    let distance = rng.random_range(300.0..=500.0);

    let spawn_pos = Vec3::new(
        player.translation.x + angle.cos() * distance,
        player.translation.y + angle.sin() * distance,
        0.0,
    );

    // Random enemy type
    let enemy_type = match rng.random_range(0..=2) {
        0 => EnemyType::Basic,
        1 => EnemyType::Fast,
        _ => EnemyType::Tank
    };

    //based on the type, give the variables
    let (enemy_speed, enemy_damage, health, sprite) = match enemy_type {
        EnemyType::Basic => (ENEMY_SPEEDS[0], 2.0, 2, asset_server.load("sprites/BlueEnemy.png")),
        EnemyType::Fast => (ENEMY_SPEEDS[1], 1.0, 1, asset_server.load("sprites/WhiteEnemy.png")),
        EnemyType::Tank => (ENEMY_SPEEDS[2], 3.0, 3, asset_server.load("sprites/RedEnemy.png"))
    };

    //spawn
    commands.spawn((
        Enemy {
            health,
            size: ENEMY_SIZE,
            damage: enemy_damage,
            speed: enemy_speed
        },
        Sprite{
                image: sprite,
                ..Default::default()
            },
        Transform::from_translation(spawn_pos),
        GlobalTransform::default(),
        TextFont {
            font_size: 20.0,
            font: default(),
            ..default()
        },
        restart::AllEntities
    ));
}

fn move_enemies_toward_player(
    time: Res<Time>,
    player: Single<&Transform, With<Player>>,
    mut enemies: Query<(&mut Transform, &Enemy), Without<Player>>,
) {
    let player_pos = player.translation;

    for (mut transform, enemy) in &mut enemies {
        let direction = (player_pos - transform.translation).truncate();

        if direction != Vec2::ZERO {
            let speed = enemy.speed;
            let delta = direction.normalize() * speed * time.delta_secs();

            transform.translation.x += delta.x;
            transform.translation.y += delta.y;
        }
    }
}