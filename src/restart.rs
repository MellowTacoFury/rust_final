use bevy::prelude::*;

//pieces needed
use crate::player::spawn_player;
use crate::hud::{Health, STARTING_HEALTH, Score, Coins};


pub struct RestartPlugin;
impl Plugin for RestartPlugin{

    fn build(&self, app: &mut App)
    {
        app.insert_state(GameState::Playing);
        app.add_systems(OnEnter(GameState::GameOver), spawn_game_over_text);
        app.add_systems(Update, restart_game.run_if(in_state(GameState::GameOver)));
        app.add_systems(OnEnter(GameState::GameOver), cleanup_entities);
        app.add_systems(OnEnter(GameState::Playing), spawn_player);
        app.add_systems(OnEnter(GameState::Playing), setup_new_game);
    }
}

#[derive(States, Debug, Clone, PartialEq, Eq, Hash, Default)]
pub enum GameState {
    #[default]
    Playing,
    GameOver,
}
#[derive(Component)]
struct GOText;
#[derive(Component)]//everything gets this, for restarting
pub struct AllEntities;



fn cleanup_entities(mut commands: Commands, query: Query<Entity, With<AllEntities>>) {
    for entity in &query {
        commands.entity(entity).despawn();
    }
}

fn restart_game(input: Res<ButtonInput<KeyCode>>, mut next_state: ResMut<NextState<GameState>>) {
    if input.just_pressed(KeyCode::Backspace) {
        next_state.set(GameState::Playing);
    }
}

fn spawn_game_over_text(mut commands: Commands) {
    commands.spawn((
        Text::new("YOU DIED\npress Backspace"),
        TextFont {
            font_size: 60.0,
            font: default(),
            ..default()
        },
        TextColor(Color::linear_rgb(0.17, 0.01, 0.01)),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Percent(40.0),
            left: Val::Percent(25.0),
            ..default()
        },
        GOText,
    ));
}

fn setup_new_game(
    mut commands: Commands,
    mut score: ResMut<Score>,
    mut health: ResMut<Health>,
    mut coins: ResMut<Coins>,
    game_over_text: Query<Entity, With<GOText>>,
) {
    // Reset score
    score.0 = 0;

    //reset health
    health.0 = STARTING_HEALTH;

    coins.0 = 0;

    // Remove game over text
    for entity in &game_over_text {
        commands.entity(entity).despawn();
    }

}