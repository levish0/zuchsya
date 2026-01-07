//! Zuchsya - osu!mania clone built with Bevy

use bevy::prelude::*;
use bevy::window::WindowResolution;

mod app;
mod ui;

use app::state::GameState;

fn main() {
    App::new()
        .add_plugins(DefaultPlugins.set(WindowPlugin {
            primary_window: Some(Window {
                title: "Zuchsya".to_string(),
                resolution: WindowResolution::new(1280, 720),
                ..default()
            }),
            ..default()
        }))
        .init_state::<GameState>()
        .add_systems(Startup, setup)
        .add_plugins(ui::UiPlugin)
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn 2D camera
    commands.spawn(Camera2d);
}