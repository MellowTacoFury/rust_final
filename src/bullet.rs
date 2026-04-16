use bevy::prelude::*;
use bevy::window::PrimaryWindow;
use bevy::input::mouse::MouseButtonInput;
use bevy::input::ButtonState;

use crate::player::Player;//the public player component.

pub struct BulletPlugin;

impl Plugin for BulletPlugin{

    fn build(&self, app: &mut App){
        app.add_systems(Update, shoot_bullet);
        app.add_systems(Update, move_bullet);
    }
}



#[derive(Component)]
struct Bullet {
    damage: i32,
    direction: Vec2,
    speed: f32,
}

fn shoot_bullet(
    mut mousebtn_evr: MessageReader<MouseButtonInput>,
    mut commands: Commands,
    window_query: Query<&Window, With<PrimaryWindow>>,//for window info
    camera_q: Query<(&Camera, &GlobalTransform)>,
    player_q: Query<&Transform, With<Player>>,
    asset_server: Res<AssetServer>
) {
    //get player
    let Ok(player_tf) = player_q.single() else {
        return; //No player
    };
    //get window
    let window: &Window = window_query.single().unwrap();

    //camera for turning actual screen pos to the game world
    let (camera, cam_gt) = camera_q.single().unwrap();

    //read inputs
    for ev in mousebtn_evr.read() {
        //if they are pressed and released
        if ev.state == ButtonState::Pressed && ev.button == MouseButton::Left {
            //the mouse is in the window
            if let Some(cursor_pos) = window.cursor_position() {
                //translate the input from the mouse to game world
                if let Ok(world_pos) = camera.viewport_to_world_2d(cam_gt, cursor_pos) {
                    let dir = (world_pos - player_tf.translation.truncate()).normalize();

                    commands.spawn((
                    Sprite{
                        image: asset_server.load("sprites/ball_blue_large.png"),

                        ..Default::default()
                    },Transform::from_xyz(player_tf.translation.x, player_tf.translation.y, player_tf.translation.z),
                    Bullet {
                        damage: 1,
                        direction: dir,
                        speed: 600.0,
                    },
                ));
                }
            }
        }
    }
}



fn move_bullet(
    time: Res<Time>,
    mut commands: Commands,
    mut q: Query<(Entity, &mut Transform, &Bullet)>,
) {
    for (entity, mut tf, bullet) in q.iter_mut() {
        let delta = bullet.direction * bullet.speed * time.delta_secs();
        tf.translation.x += delta.x;
        tf.translation.y += delta.y;

        //Destroy once it moves far enough
        if tf.translation.length() > 5000.0 {
            commands.entity(entity).despawn();
        }
    }
}