use bevy::prelude::*;


mod camera;
mod player;
mod bullet;



fn main()
{
    let mut app: App = App::new();
    app.add_plugins(DefaultPlugins);//will panic without this
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(bullet::BulletPlugin);


    app.run();
}
// use bevy::audio::{AudioPlayer, PlaybackSettings};


// //need the player and window to keep the player in the window

/*Needs
    collision/health
    firing
    enemies - drop coins
    turrets & walls - cost coins


    displaying text for which buttons place turret/walls
    audio for shooting/getting hit/hitting/placing
*/

