//! Note rendering and management

mod spawn;
mod systems;
mod types;

pub use types::*;

use bevy::prelude::*;
use zuchsya_core::GameState;

pub struct NotePlugin;

impl Plugin for NotePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), spawn::spawn_notes)
            .add_systems(
                Update,
                systems::update_note_positions.run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), systems::cleanup_notes);
    }
}
