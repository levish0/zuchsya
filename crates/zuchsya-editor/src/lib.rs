//! Zuchsya Editor - Beatmap editor systems

use bevy::prelude::*;
use zuchsya_core::GameState;

/// Editor plugin - adds all editor systems
pub struct EditorPlugin;

impl Plugin for EditorPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Editor), setup_editor)
            .add_systems(OnExit(GameState::Editor), cleanup_editor);
    }
}

#[derive(Component)]
struct EditorRoot;

fn setup_editor(mut commands: Commands) {
    commands
        .spawn((
            EditorRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                ..default()
            },
            BackgroundColor(Color::srgb(0.1, 0.1, 0.12)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Editor (WIP)"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
            ));
        });
}

fn cleanup_editor(mut commands: Commands, query: Query<Entity, With<EditorRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
