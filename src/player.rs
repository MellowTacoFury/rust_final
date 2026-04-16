use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

//consts
const PLAYER_SPEED: f32 = 500.0;
//in pixels for the current sprite
const PLAYER_SIZE: f32 = 64.0;

impl Plugin for PlayerPlugin{

    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
        app.add_systems(Update, confine_player);
    }
}


#[derive(Component)]
pub struct Player{
    //give these in the spawn for Player
    pub speed: f32,
    pub size: f32
}

#[derive(Component)]
struct Collider;


//creation
fn spawn_player(
    mut commands: Commands,//for spawning
    window_query: Query<&Window, With<PrimaryWindow>>,//for window info
    asset_server: Res<AssetServer>//for player asset
)
{
    //let the window of type &Window 
    //be the only query that is window with the PW, unwrap to get it
    let window: &Window = window_query.single().unwrap();

    //spawn an entity with the Sprite, the Transform
    //and give it the custom "Player" component, so we can query for the component
    //and collider component for later collision
    //src\assets\sprites\ASSETNAME.png
    commands.spawn(
        (
            Sprite{
                image: asset_server.load("sprites/ball_blue_large.png"),
                ..Default::default()
            },
            Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
            Player{
                speed: PLAYER_SPEED,
                size: PLAYER_SIZE
            },
            Collider
        )
    );
}


//movement
fn player_movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    mut player_query: Query<(&mut Transform, &Player)>,//get the transform and the player under playerQuery
    time: Res<Time>
){
    if let Ok ((mut transform, player)) = player_query.single_mut() 
    {
        let mut direction = Vec3::ZERO;

        //get keyboard input
        if keyboard_input.pressed(KeyCode::ArrowLeft) || keyboard_input.pressed(KeyCode::KeyA)
        {
            direction += Vec3::new(-10.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowRight) || keyboard_input.pressed(KeyCode::KeyD)
        {
            direction += Vec3::new(10.0, 0.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowUp) || keyboard_input.pressed(KeyCode::KeyW)
        {
            direction += Vec3::new(0.0, 10.0, 0.0);
        }
        if keyboard_input.pressed(KeyCode::ArrowDown) || keyboard_input.pressed(KeyCode::KeyS)
        {
            direction += Vec3::new(0.0, -10.0, 0.0);
        }

        //normalize it
        if direction.length() > 0.0{
            direction = direction.normalize();
        }

        //move
        transform.translation += direction * player.speed * time.delta_secs();


    }
}

fn confine_player(
    mut player_query: Query<(&mut Transform, &Player)>,
    window_query: Query<&Window, With<PrimaryWindow>>,//for window width,hight info
){
    if let Ok((mut transform, player)) = player_query.single_mut()
    {
        let window: &Window = window_query.single().unwrap();
        //calc from middle
        let half_player_size: f32 = player.size /2.0;

        let x_min = 0.0 + half_player_size;
        let x_max = window.width() - half_player_size;
        let y_min = 0.0 + half_player_size;
        let y_max = window.height() - half_player_size;

        //Grab because its called so much
        let mut t_translation = transform.translation;

        //lock the x
        if t_translation.x < x_min{
            t_translation.x = x_min;
        }else if t_translation.x > x_max{
            t_translation.x = x_max;
        }
        //lock the y
        if t_translation.y < y_min{
            t_translation.y = y_min;
        }else if t_translation.y > y_max{
            t_translation.y = y_max;
        }

        //put it back
        transform.translation = t_translation;

    }
}





