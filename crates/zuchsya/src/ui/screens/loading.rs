//! Loading screen

use bevy::prelude::*;
use zuchsya_core::GameState;

pub struct LoadingPlugin;

impl Plugin for LoadingPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Loading), setup_loading)
            .add_systems(Update, update_loading.run_if(in_state(GameState::Loading)))
            .add_systems(OnExit(GameState::Loading), cleanup_loading);
    }
}

#[derive(Component)]
struct LoadingScreen;

#[derive(Component)]
struct LoadingText;

fn setup_loading(mut commands: Commands) {
    // Loading screen root
    commands
        .spawn((
            LoadingScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.1)),
        ))
        .with_children(|parent| {
            // Title
            parent.spawn((
                Text::new("ZUCHSYA"),
                TextFont {
                    font_size: 64.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Loading text
            parent.spawn((
                LoadingText,
                Text::new("Loading..."),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.7, 0.7, 0.7)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

fn update_loading(
    mut next_state: ResMut<NextState<GameState>>,
    time: Res<Time>,
    mut elapsed: Local<f32>,
) {
    *elapsed += time.delta_secs();

    // Simulate loading (go to main menu after 1 second)
    // TODO: Replace with actual asset loading
    if *elapsed > 1.0 {
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_loading(mut commands: Commands, query: Query<Entity, With<LoadingScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}