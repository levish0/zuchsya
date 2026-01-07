//! Zuchsya Play - Gameplay systems

use bevy::prelude::*;

pub mod playfield;
pub mod input;
pub mod scroll;

pub use playfield::{PlayfieldPlugin, PlayfieldConfig, Playfield, Column, HitTarget};
pub use input::{InputPlugin, KeyBindings, KeyState};
pub use scroll::{ScrollPlugin, ScrollConfig, GameTime};

/// Gameplay plugin - adds all gameplay systems
pub struct PlayPlugin;

impl Plugin for PlayPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(scroll::GameTime::default())
            .add_plugins((
                playfield::PlayfieldPlugin,
                input::InputPlugin,
                scroll::ScrollPlugin,
            ));
    }
}