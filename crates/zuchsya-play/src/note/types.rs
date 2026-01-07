//! Note component types

use bevy::prelude::*;
use zuchsya_core::HitObject;

/// Note constants
pub const NOTE_HEIGHT: f32 = 20.0;

/// Colors for notes (matching column colors)
pub const NOTE_COLORS: [Color; 4] = [
    Color::srgb(1.0, 0.3, 0.3), // Red
    Color::srgb(0.3, 0.3, 1.0), // Blue
    Color::srgb(0.3, 0.3, 1.0), // Blue
    Color::srgb(1.0, 0.3, 0.3), // Red
];

/// Marker for regular tap note entities
#[derive(Component)]
pub struct Note {
    /// Column index (0-based)
    pub column: u8,
    /// Hit time in milliseconds
    pub time_ms: f64,
    /// Whether this note has been hit
    pub hit: bool,
}

/// Unique ID for linking hold note parts together
#[derive(Component, Clone, Copy, PartialEq, Eq, Hash)]
pub struct HoldNoteId(pub u32);

/// Marker for hold note head
#[derive(Component)]
pub struct HoldNoteHead {
    pub column: u8,
    pub start_time_ms: f64,
    pub end_time_ms: f64,
}

/// Marker for hold note body
#[derive(Component)]
pub struct HoldNoteBody {
    pub column: u8,
    pub start_time_ms: f64,
    pub end_time_ms: f64,
}

/// Marker for hold note tail
#[derive(Component)]
pub struct HoldNoteTail {
    pub column: u8,
    pub start_time_ms: f64,
    pub end_time_ms: f64,
}

/// State tracking for hold notes (attached to head)
#[derive(Component, Default)]
pub struct HoldNoteState {
    /// Whether the head has been hit
    pub head_hit: bool,
    /// The judgement result for the head (if hit)
    pub head_result: Option<zuchsya_core::HitResult>,
    /// Whether currently holding the key
    pub is_holding: bool,
    /// Whether the hold was broken (released early)
    pub hold_broken: bool,
    /// Whether the tail has been judged
    pub tail_judged: bool,
}

/// Resource containing all hit objects for current beatmap
#[derive(Resource, Default)]
pub struct CurrentHitObjects {
    pub objects: Vec<HitObject>,
}
