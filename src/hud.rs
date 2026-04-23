use bevy::prelude::*;
use bevy::input::*;

use crate::restart::{self, GameState};

//consts
pub const STARTING_HEALTH: f32 = 10.0;

pub struct HUDPlugin;
impl Plugin for HUDPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_score);
        app.add_systems(Startup, setup_health);
        app.add_systems(Startup, setup_coins);
        app.add_systems(Startup, setup_directions);

        app.insert_resource(Score(0));
        app.insert_resource(Coins(0));
        app.insert_resource(Health(STARTING_HEALTH));

        app.add_systems(Update, update_score_ui);
        app.add_systems(Update, update_health_ui);
        app.add_systems(Update, update_coins_ui);
        app.add_systems(Update, die_faster);
    }
}

#[derive(Component)]
struct ScoreText;
#[derive(Resource)]
pub struct Score(pub u32);

#[derive(Component)]
struct HealthText;
#[derive(Resource)]
pub struct Health(pub f32);

#[derive(Component)]
struct CoinsText;
#[derive(Resource)]
pub struct Coins(pub i32);



fn setup_score(mut commands: Commands) {
    // Score UI (screen space)
    commands.spawn((
        Text::new("Score: 0"),
        TextFont {
            font_size: 28.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
        ScoreText,
    ));
}

fn update_score_ui(score: Res<Score>, mut query: Query<&mut Text, With<ScoreText>>) {
    if score.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            text.0 = format!("Score: {}", score.0);
        }
    }
}

fn setup_health(health: Res<Health>,mut commands: Commands) {
    // Score UI (screen space)
    commands.spawn((
        Text::new(format!("Health: {}", health.0)),
        TextFont {
            font_size: 28.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            top: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        HealthText,
    ));
}

fn update_health_ui(health: Res<Health>, mut query: Query<&mut Text, With<HealthText>>) {
    if health.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            text.0 = format!("Health: {}", health.0);
        }
    }
}

fn setup_coins(mut commands: Commands) {
    commands.spawn((
        Text::new("Coins: 0"),
        TextFont {
            font_size: 28.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            right: Val::Px(10.0),
            ..default()
        },
        CoinsText,
    ));
}

fn update_coins_ui(coins: Res<Coins>, mut query: Query<&mut Text, With<CoinsText>>) {
    if coins.is_changed() {
        if let Ok(mut text) = query.single_mut() {
            text.0 = format!("Coins: {}", coins.0);
        }
    }
}


fn setup_directions(mut commands: Commands) {
    commands.spawn((
        Text::new("move - WASD\nshoot - LMB\nBuy Wall (5 coins) - E\nDie faster - Q"),
        TextFont {
            font_size: 28.0,
            font: default(),
            ..default()
        },
        TextColor(Color::WHITE),
        Node {
            position_type: PositionType::Absolute,
            bottom: Val::Px(10.0),
            left: Val::Px(10.0),
            ..default()
        },
    ));
}


fn die_faster(keyboard_input: Res<ButtonInput<KeyCode>>,
    mut next_state: ResMut<NextState<GameState>>,)
{
    if keyboard_input.pressed(KeyCode::KeyQ)
    {
        next_state.set(GameState::GameOver);
    }
}


