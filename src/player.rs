use bevy::prelude::*;
use bevy::window::PrimaryWindow;

pub struct PlayerPlugin;

pub const PLAYER_SPEED: f32 = 500.0;
impl Plugin for PlayerPlugin{

    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_player);
        app.add_systems(Update, player_movement);
    }
}


#[derive(Component)]
struct Player{
    pub speed: f32
}

#[derive(Component)]
struct Collider;



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
            },
            Collider
        )
    );
}



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