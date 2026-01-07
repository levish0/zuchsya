//! Beatmap types and serialization

use serde::{Deserialize, Serialize};
use crate::{HitObject, TimingPoint};

/// Beatmap file format version
pub const BEATMAP_VERSION: u32 = 1;

/// Complete beatmap data
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Beatmap {
    /// Format version
    pub version: u32,
    /// Beatmap metadata
    pub metadata: Metadata,
    /// Difficulty settings
    pub difficulty: Difficulty,
    /// Audio file path (relative to beatmap)
    pub audio_file: String,
    /// Background image path (optional)
    pub background_file: Option<String>,
    /// Preview time in milliseconds
    pub preview_time: i32,
    /// Timing points
    pub timing_points: Vec<TimingPoint>,
    /// Hit objects
    pub hit_objects: Vec<HitObject>,
}

impl Default for Beatmap {
    fn default() -> Self {
        Self {
            version: BEATMAP_VERSION,
            metadata: Metadata::default(),
            difficulty: Difficulty::default(),
            audio_file: String::new(),
            background_file: None,
            preview_time: 0,
            timing_points: Vec::new(),
            hit_objects: Vec::new(),
        }
    }
}

/// Beatmap metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// Song title
    pub title: String,
    /// Song title (unicode)
    pub title_unicode: Option<String>,
    /// Artist name
    pub artist: String,
    /// Artist name (unicode)
    pub artist_unicode: Option<String>,
    /// Beatmap creator
    pub creator: String,
    /// Difficulty name
    pub difficulty_name: String,
    /// Source (game, anime, etc.)
    pub source: Option<String>,
    /// Tags for searching
    pub tags: Vec<String>,
}

/// Difficulty settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Difficulty {
    /// Number of keys (1-10)
    pub key_count: u8,
    /// Overall Difficulty (affects hit windows)
    pub overall_difficulty: f32,
    /// HP Drain Rate
    pub hp_drain: f32,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self {
            key_count: 4,
            overall_difficulty: 5.0,
            hp_drain: 5.0,
        }
    }
}

impl Beatmap {
    /// Create a new empty beatmap
    pub fn new() -> Self {
        Self::default()
    }

    /// Load beatmap from JSON string
    pub fn from_json(json: &str) -> Result<Self, serde_json::Error> {
        serde_json::from_str(json)
    }

    /// Serialize beatmap to JSON string
    pub fn to_json(&self) -> Result<String, serde_json::Error> {
        serde_json::to_string_pretty(self)
    }

    /// Get total duration in milliseconds
    pub fn duration(&self) -> f64 {
        self.hit_objects
            .iter()
            .map(|obj| obj.end_time())
            .fold(0.0, f64::max)
    }

    /// Get hit object count
    pub fn object_count(&self) -> usize {
        self.hit_objects.len()
    }
}