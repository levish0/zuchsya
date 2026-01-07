//! Zuchsya Play - Gameplay systems

use bevy::prelude::*;

pub mod input;
pub mod judgement;
pub mod note;
pub mod playfield;
pub mod scroll;
pub mod hud;

pub use input::{InputPlugin, KeyBindings, KeyState};
pub use judgement::{JudgementEvent, JudgementPlugin, ScoreState};
pub use note::{CurrentHitObjects, HoldNoteBody, HoldNoteHead, HoldNoteId, HoldNoteState, HoldNoteTail, Note, NotePlugin};
pub use playfield::{Column, HitTarget, Playfield, PlayfieldConfig, PlayfieldPlugin};
pub use scroll::{GameTime, ScrollConfig, ScrollPlugin};
pub use hud::HudPlugin;

/// Gameplay plugin - adds all gameplay systems
pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(scroll::GameTime::default())
            .insert_resource(note::CurrentHitObjects::default())
            .add_plugins((
                playfield::PlayfieldPlugin,
                input::InputPlugin,
                scroll::ScrollPlugin,
                note::NotePlugin,
                judgement::JudgementPlugin,
                hud::HudPlugin,
            ));
    }
}
