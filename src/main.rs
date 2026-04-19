use bevy::prelude::*;

mod restart;
mod camera;
mod player;
mod bullet;
mod enemy;
mod hud;

fn main()
{
    let mut app: App = App::new();
    app.add_plugins(DefaultPlugins);//will panic without this
    app.add_plugins(camera::CameraPlugin);
    app.add_plugins(player::PlayerPlugin);
    app.add_plugins(bullet::BulletPlugin);
    app.add_plugins(enemy::EnemyPlugin);
    app.add_plugins(hud::HUDPlugin);
    app.add_plugins(restart::RestartPlugin);


    app.run();
}

// use bevy::audio::{AudioPlayer, PlaybackSettings};
/*Needs
    audio
    add turrets? maybe walls
*/