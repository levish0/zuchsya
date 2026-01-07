//! Zuchsya - osu!mania clone built with Bevy

use bevy::prelude::*;
use bevy::window::WindowResolution;
use zuchsya_core::GameState;
use zuchsya_editor::EditorPlugin;
use zuchsya_play::PlayPlugin;

mod ui;

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
        .add_plugins((ui::UiPlugin, PlayPlugin, EditorPlugin))
        .run();
}

fn setup(mut commands: Commands) {
    // Spawn 2D camera
    commands.spawn(Camera2d);
}
