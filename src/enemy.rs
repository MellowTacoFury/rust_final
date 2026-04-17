use bevy::prelude::*;
use rand::rng;
use rand::Rng;

use crate::player::Player;//the public player component.

pub const ENEMY_SIZE: f32 = 64.0;
pub struct EnemyPlugin;



impl Plugin for EnemyPlugin{

    fn build(&self, app: &mut App){
        app.insert_resource(EnemySpawnTimer(Timer::from_seconds(1.0, TimerMode::Repeating)));
        app.add_systems(Update, spawn_enemies);
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
    kind: EnemyType,
    pub health: i32,
    pub size: f32,
    pub damage: f32
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

    let (enemy_damage, health, sprite) = match enemy_type {
        EnemyType::Basic => (2.0, 2, asset_server.load("sprites/BlueEnemy.png")),
        EnemyType::Fast => (1.0, 1, asset_server.load("sprites/WhiteEnemy.png")),
        EnemyType::Tank => (3.0, 3, asset_server.load("sprites/RedEnemy.png"))
    };

    commands.spawn((
        Enemy {
            kind: enemy_type,
            health,
            size: ENEMY_SIZE,
            damage: enemy_damage
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
    ));
}