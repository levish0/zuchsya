//! Hit object types

use serde::{Deserialize, Serialize};

/// Hit object types
#[derive(Debug, Clone, Serialize, Deserialize)]
#[serde(tag = "type")]
pub enum HitObject {
    /// Single tap note
    #[serde(rename = "note")]
    Note {
        /// Column index (0-based)
        column: u8,
        /// Hit time in milliseconds
        time: f64,
    },
    /// Hold note (long note)
    #[serde(rename = "hold")]
    HoldNote {
        /// Column index (0-based)
        column: u8,
        /// Start time in milliseconds
        time: f64,
        /// End time in milliseconds
        end_time: f64,
    },
}

impl HitObject {
    /// Create a new note
    pub fn note(column: u8, time: f64) -> Self {
        Self::Note { column, time }
    }

    /// Create a new hold note
    pub fn hold_note(column: u8, time: f64, end_time: f64) -> Self {
        Self::HoldNote {
            column,
            time,
            end_time,
        }
    }

    /// Get the column index
    pub fn column(&self) -> u8 {
        match self {
            Self::Note { column, .. } => *column,
            Self::HoldNote { column, .. } => *column,
        }
    }

    /// Get the start time
    pub fn start_time(&self) -> f64 {
        match self {
            Self::Note { time, .. } => *time,
            Self::HoldNote { time, .. } => *time,
        }
    }

    /// Get the end time (same as start_time for notes)
    pub fn end_time(&self) -> f64 {
        match self {
            Self::Note { time, .. } => *time,
            Self::HoldNote { end_time, .. } => *end_time,
        }
    }

    /// Check if this is a hold note
    pub fn is_hold(&self) -> bool {
        matches!(self, Self::HoldNote { .. })
    }

    /// Get duration (0 for notes)
    pub fn duration(&self) -> f64 {
        self.end_time() - self.start_time()
    }
}