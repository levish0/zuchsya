//! Gameplay systems

use bevy::prelude::*;

pub mod playfield;
pub mod input;
pub mod scroll;

use crate::app::state::GameState;

/// Gameplay plugin
pub struct GameplayPlugin;

impl Plugin for GameplayPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            playfield::PlayfieldPlugin,
            input::InputPlugin,
            scroll::ScrollPlugin,
        ));
    }
}
