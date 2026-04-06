use bevy::prelude::*;


mod camera;
mod player;



fn main()
{
    let mut app: App = App::new();
    app.add_plugins(DefaultPlugins);//will panic without this
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(player::PlayerPlugin);


    app.run();
}
