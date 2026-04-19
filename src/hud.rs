use bevy::prelude::*;


//consts
pub const STARTING_HEALTH: f32 = 10.0;

pub struct HUDPlugin;
impl Plugin for HUDPlugin
{
    fn build(&self, app: &mut App) {
        app.add_systems(Startup, setup_score);
        app.add_systems(Startup, setup_health);

        app.insert_resource(Score(0));
        app.insert_resource(Health(STARTING_HEALTH));

        app.add_systems(Update, update_score_ui);
        app.add_systems(Update, update_health_ui);
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

fn setup_health(mut commands: Commands) {
    // Score UI (screen space)
    commands.spawn((
        Text::new("Health: 0"),
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