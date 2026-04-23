use bevy::prelude::*;
use rand::rng;
use rand::Rng;
use bevy::window::PrimaryWindow;

use crate::player::Player;
use crate::restart::{self, GameState};
use crate::hud::Health;

pub const HEALTH_SIZE: f32 = 64.0;
pub struct HealthPlugin;

impl Plugin for HealthPlugin{
    fn build(&self, app: &mut App)
    {
        app.insert_resource(HealthpackSpawnTimer(Timer::from_seconds(10.0, TimerMode::Repeating)));
        app.add_systems(Update, spawn_health.run_if(in_state(restart::GameState::Playing)));
        app.add_systems(Update, health_player_collision);
    }
}

#[derive(Component)]
struct HealthPack {
    size: f32,
    amount: f32
}


#[derive(Resource)]
struct HealthpackSpawnTimer(Timer);

fn spawn_health(
    mut commands: Commands,
    time: Res<Time>,
    mut timer: ResMut<HealthpackSpawnTimer>,
    window_query: Query<&Window, With<PrimaryWindow>>,
    asset_server: Res<AssetServer>) 
{
    if !timer.0.tick(time.delta()).just_finished() {
        return;
    }
    let window: &Window = window_query.single().unwrap();

    let mut rng = rng();
    let angle = rng.random_range(0.0..=std::f32::consts::TAU);
    let distance = rng.random_range(200.0..=300.0);

    //spawn
    commands.spawn((
        HealthPack {
            size: HEALTH_SIZE,
            amount: 5.0,
        },
        Sprite{
                image: asset_server.load("sprites/health.png"),
                ..Default::default()
            },
        Transform::from_xyz(window.width()/2.0 + angle.cos() * distance, window.height()/2.0 + angle.sin() * distance, 0.0),
        GlobalTransform::default(),
        TextFont {
            font_size: 20.0,
            font: default(),
            ..default()
        },
        restart::AllEntities,
    ));
}





fn health_player_collision(
    mut commands:Commands,
    health_pack: Query<(Entity, &Transform, &HealthPack), With<HealthPack>>,
    mut Player: Query<(
        Entity,
        &Transform,
        &mut Player,
    )>,
    mut health: ResMut<Health>
) {
    for (health_entity, health_tf, health_obj) in &health_pack {
        for (player_entity, player_tf, mut player) in &mut Player
        {
            let distance = health_tf
                .translation
                .truncate()
                .distance(player_tf.translation.truncate());

            let health_obj_radius = health_obj.size/2.0;
            if distance < health_obj_radius + player.size/2.0{
                commands.entity(health_entity).despawn();
                health.0 += health_obj.amount;
                
                break;
            }
        }
    }
}