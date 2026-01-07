//! Timing point types

use serde::{Deserialize, Serialize};

/// Timing point for BPM and scroll speed
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingPoint {
    /// Time in milliseconds
    pub time: f64,
    /// BPM (beats per minute)
    pub bpm: f64,
    /// Time signature numerator (e.g., 4 for 4/4)
    pub time_signature: u8,
    /// Scroll speed multiplier (1.0 = normal)
    #[serde(default = "default_scroll_speed")]
    pub scroll_speed: f64,
    /// Kiai mode enabled
    #[serde(default)]
    pub kiai: bool,
}

fn default_scroll_speed() -> f64 {
    1.0
}

impl TimingPoint {
    /// Create a new timing point
    pub fn new(time: f64, bpm: f64) -> Self {
        Self {
            time,
            bpm,
            time_signature: 4,
            scroll_speed: 1.0,
            kiai: false,
        }
    }

    /// Get beat length in milliseconds
    pub fn beat_length(&self) -> f64 {
        60000.0 / self.bpm
    }

    /// Snap time to nearest beat division
    pub fn snap_to_beat(&self, time: f64, divisor: u8) -> f64 {
        let snap_length = self.beat_length() / divisor as f64;
        let offset = time - self.time;
        let snapped_offset = (offset / snap_length).round() * snap_length;
        self.time + snapped_offset
    }
}

impl Default for TimingPoint {
    fn default() -> Self {
        Self {
            time: 0.0,
            bpm: 120.0,
            time_signature: 4,
            scroll_speed: 1.0,
            kiai: false,
        }
    }
}