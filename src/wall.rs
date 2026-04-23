use bevy::prelude::*;

//pieces needed
use crate::player::Player;
use crate::enemy::Enemy;
use crate::restart;
use crate::hud::Score;
use crate::hud::Coins;
use crate::coin::spawn_coin;


//consts
const WALL_SIZE: f32 = 80.0;


pub struct WallPlugin;
impl Plugin for WallPlugin{

    fn build(&self, app: &mut App){
        app.add_systems(Update, spawn_wall.run_if(in_state(restart::GameState::Playing)));
        app.add_systems(Update, wall_enemy_collision);
    }
}


#[derive(Component)]
struct Wall {
    size: f32
}


fn spawn_wall(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut commands: Commands,
    player_q: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>,
    mut coins: ResMut<Coins>
) {
    //get player
    let Ok(player_tf) = player_q.single() else {
        return; //No player
    };

    //if they are pressed and released
    if keyboard_input.pressed(KeyCode::KeyE){
        if coins.0 >= 5{
            commands.spawn((
            Sprite{
                image: asset_server.load("sprites/wall.png"),
                ..Default::default()
            },Transform::from_xyz(player_tf.translation.x, player_tf.translation.y, player_tf.translation.z),
            Wall {
                size: WALL_SIZE
            },
            restart::AllEntities
            ));
            coins.0 -= 5;
        }
    }
}

fn wall_enemy_collision(
    mut commands:Commands,
    asset_server: Res<AssetServer>,
    walls: Query<(Entity, &Transform, &Wall), With<Wall>>,
    mut enemies: Query<(
        Entity,
        &Transform,
        &mut Enemy,
    )>,
    mut score: ResMut<Score>
) {
    for (wall_entity, wall_tf, wall) in &walls {
        for (enemy_entity, enemy_tf, mut enemy) in &mut enemies
        {
            let distance = wall_tf
                .translation
                .truncate()
                .distance(enemy_tf.translation.truncate());

            let wall_radius = wall.size/2.0;
            if distance < wall_radius + enemy.size/2.0{
                commands.entity(wall_entity).despawn();

                score.0 += 10;
                if enemy.will_drop_coin == true{
                    spawn_coin(&asset_server, &mut commands, enemy_tf);
                }
                commands.entity(enemy_entity).despawn();
                commands.entity(wall_entity).despawn();

                break;
            }
        }
    }
}