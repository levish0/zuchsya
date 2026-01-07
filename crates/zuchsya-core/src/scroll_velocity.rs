//! Scroll velocity types for .zuchsya format

use serde::{Deserialize, Serialize};

/// Scroll velocity change point
///
/// Controls how fast notes scroll at a given time,
/// independent of BPM. Used for visual effects and
/// reading difficulty adjustments.
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ScrollVelocity {
    /// Time in milliseconds
    pub time: f64,
    /// Speed multiplier (1.0 = normal, 2.0 = twice as fast, 0.5 = half speed)
    pub multiplier: f64,
}

impl ScrollVelocity {
    /// Create a new scroll velocity point
    pub fn new(time: f64, multiplier: f64) -> Self {
        Self { time, multiplier }
    }
}

impl Default for ScrollVelocity {
    fn default() -> Self {
        Self {
            time: 0.0,
            multiplier: 1.0,
        }
    }
}
