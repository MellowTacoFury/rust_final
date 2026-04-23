use bevy::prelude::*;

use crate::player::Player;
use crate::restart::{self, GameState};
use crate::hud::Coins;

pub const COIN_SIZE: f32 = 64.0;
pub struct CoinPlugin;
impl Plugin for CoinPlugin{
    fn build(&self, app: &mut App)
    {
        app.add_systems(Update, coin_player_collision);
    }
}

#[derive(Component)]
struct Coin {
    size: f32
}

#[derive(Component)]
struct Collider;


pub fn spawn_coin(
    mut asset_server: &Res<AssetServer>, 
    mut commands: &mut Commands,
    mut transform: &Transform)
{
    commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/Bullet.png"),
                ..Default::default()
            },
            Transform::from_xyz(transform.translation.x, transform.translation.y, transform.translation.z),
            Coin{
                size: COIN_SIZE
            },
            Collider,
            restart::AllEntities
        )
    );
}

fn coin_player_collision(
    mut commands:Commands,
    coins: Query<(Entity, &Transform, &Coin), With<Coin>>,
    mut Player: Query<(
        Entity,
        &Transform,
        &mut Player,
    )>,
    mut coinCount: ResMut<Coins>
) {
    for (coin_entity, coin_tf, coin) in &coins {
        for (player_entity, player_tf, mut player) in &mut Player
        {
            let distance = coin_tf
                .translation
                .truncate()
                .distance(player_tf.translation.truncate());

            let coin_radius = coin.size/2.0;
            if distance < coin_radius + player.size/2.0{
                commands.entity(coin_entity).despawn();
                coinCount.0 += 1;
                
                break;
            }
        }
    }
}