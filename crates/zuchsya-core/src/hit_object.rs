//! Hit object types for .zuchsya format

use serde::{Deserialize, Serialize};

/// A hit object (note or hold note)
///
/// Uses a flat struct with optional duration:
/// - `duration: None` = regular tap note
/// - `duration: Some(ms)` = hold note with given length
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct HitObject {
    /// Hit time in milliseconds
    pub time: f64,
    /// Lane index (0-based, 0 to keys-1)
    pub lane: u8,
    /// Hold duration in milliseconds (None = tap note)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub duration: Option<f64>,
}

impl HitObject {
    /// Create a new tap note
    pub fn note(lane: u8, time: f64) -> Self {
        Self {
            time,
            lane,
            duration: None,
        }
    }

    /// Create a new hold note
    pub fn hold(lane: u8, time: f64, duration: f64) -> Self {
        Self {
            time,
            lane,
            duration: Some(duration),
        }
    }

    /// Get the lane index (0-based)
    pub fn lane(&self) -> u8 {
        self.lane
    }

    /// Alias for lane() - for compatibility
    pub fn column(&self) -> u8 {
        self.lane
    }

    /// Get the start time
    pub fn start_time(&self) -> f64 {
        self.time
    }

    /// Get the end time (start_time + duration for holds, start_time for notes)
    pub fn end_time(&self) -> f64 {
        self.time + self.duration.unwrap_or(0.0)
    }

    /// Check if this is a hold note
    pub fn is_hold(&self) -> bool {
        self.duration.is_some()
    }

    /// Get duration (0 for tap notes)
    pub fn get_duration(&self) -> f64 {
        self.duration.unwrap_or(0.0)
    }
}

impl Default for HitObject {
    fn default() -> Self {
        Self {
            time: 0.0,
            lane: 0,
            duration: None,
        }
    }
}
