//! Timing point types for .zuchsya format

use serde::{Deserialize, Serialize};

/// Timing point (BPM change)
///
/// Defines the BPM at a specific time. SV (scroll velocity) changes
/// are handled separately in ScrollVelocity.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct TimingPoint {
    /// Time in milliseconds
    pub time: f64,
    /// BPM (beats per minute)
    pub bpm: f64,
    /// Time signature (beats per measure, e.g., 4 for 4/4)
    #[serde(default = "default_signature")]
    pub signature: u8,
}

fn default_signature() -> u8 {
    4
}

impl TimingPoint {
    /// Create a new timing point
    pub fn new(time: f64, bpm: f64) -> Self {
        Self {
            time,
            bpm,
            signature: 4,
        }
    }

    /// Create with custom time signature
    pub fn with_signature(time: f64, bpm: f64, signature: u8) -> Self {
        Self {
            time,
            bpm,
            signature,
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
            signature: 4,
        }
    }
}
