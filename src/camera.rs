use bevy::prelude::*;
use bevy::window::PrimaryWindow;
pub struct CameraPlugin;

impl Plugin for CameraPlugin{
    
    fn build(&self, app: &mut App){
        app.add_systems(Startup, spawn_camera);
    }
}

fn spawn_camera(
    mut commands: Commands,//for spawning entity
    window_query: Query<&Window, With<PrimaryWindow>>,//for window width,hight info
)
{
    //let the window of type &Window 
    //be the only query that is window with PW, unwrap to give it to us
    let window: &Window = window_query.single().unwrap();

    //Spawn defalt camera, in the center of the screen
    commands.spawn(
        (
        Camera2d::default(),
        Transform::from_xyz(window.width()/2.0, window.height()/2.0, 0.0),
        )
    );
}