//! Game state management

use bevy::prelude::*;

/// Main game state
#[derive(States, Debug, Clone, Copy, Eq, PartialEq, Hash, Default)]
pub enum GameState {
    /// Loading assets
    #[default]
    Loading,
    /// Logo splash screen
    Intro,
    /// Main menu
    MainMenu,
    /// Song selection
    SongSelect,
    /// Gameplay
    Playing,
    /// Results screen
    Results,
    /// Beatmap editor
    Editor,
    /// Settings screen
    Settings,
}
