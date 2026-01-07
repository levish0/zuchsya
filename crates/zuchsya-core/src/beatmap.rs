//! Beatmap types for .zuchsya format

use serde::{Deserialize, Serialize};
use std::path::Path;

use crate::{HitObject, ScrollVelocity, TimingPoint};

/// Current format version
pub const FORMAT_VERSION: u32 = 1;

/// Complete beatmap data (.zuchsya file)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct ZuchsyaMap {
    /// Format version
    pub version: u32,
    /// Beatmap metadata
    pub metadata: Metadata,
    /// Audio configuration
    pub audio: AudioInfo,
    /// Background image file (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub background: Option<String>,
    /// Difficulty settings
    pub difficulty: Difficulty,
    /// Timing points (BPM changes)
    pub timing: Vec<TimingPoint>,
    /// Scroll velocity changes (optional)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub scroll_velocities: Vec<ScrollVelocity>,
    /// Hit objects (notes)
    pub hit_objects: Vec<HitObject>,
    /// Editor-only data (optional)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub editor: Option<EditorInfo>,
}

/// Beatmap metadata
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct Metadata {
    /// Song title
    pub title: String,
    /// Artist name
    pub artist: String,
    /// Map creator
    pub creator: String,
    /// Difficulty name
    pub difficulty_name: String,
    /// Song title (unicode)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub title_unicode: Option<String>,
    /// Artist name (unicode)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub artist_unicode: Option<String>,
    /// Source (album, game, etc.)
    #[serde(default, skip_serializing_if = "Option::is_none")]
    pub source: Option<String>,
    /// Tags for searching
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub tags: Vec<String>,
}

/// Audio configuration
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct AudioInfo {
    /// Audio file name
    pub file: String,
    /// Preview start time in ms (-1 = 40% of song)
    #[serde(default = "default_preview_time")]
    pub preview_time: i32,
}

fn default_preview_time() -> i32 {
    -1
}

impl Default for AudioInfo {
    fn default() -> Self {
        Self {
            file: String::new(),
            preview_time: -1,
        }
    }
}

/// Difficulty settings
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct Difficulty {
    /// Number of keys (4-8)
    pub keys: u8,
    /// Overall Difficulty (0-10, affects hit windows)
    pub od: f32,
    /// HP Drain (0-10)
    pub hp: f32,
}

impl Default for Difficulty {
    fn default() -> Self {
        Self {
            keys: 4,
            od: 5.0,
            hp: 5.0,
        }
    }
}

/// Editor-only information
#[derive(Debug, Clone, Default, Serialize, Deserialize)]
pub struct EditorInfo {
    /// Bookmarks (time positions in ms)
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub bookmarks: Vec<i32>,
    /// Break periods
    #[serde(default, skip_serializing_if = "Vec::is_empty")]
    pub breaks: Vec<BreakPeriod>,
}

/// Break period (rest time)
#[derive(Debug, Clone, Serialize, Deserialize)]
pub struct BreakPeriod {
    pub start: f64,
    pub end: f64,
}

impl Default for ZuchsyaMap {
    fn default() -> Self {
        Self {
            version: FORMAT_VERSION,
            metadata: Metadata::default(),
            audio: AudioInfo::default(),
            background: None,
            difficulty: Difficulty::default(),
            timing: vec![TimingPoint::default()],
            scroll_velocities: Vec::new(),
            hit_objects: Vec::new(),
            editor: None,
        }
    }
}

impl ZuchsyaMap {
    /// Create a new empty beatmap
    pub fn new() -> Self {
        Self::default()
    }

    /// Load beatmap from YAML file
    pub fn load(path: &Path) -> Result<Self, BeatmapError> {
        let content = std::fs::read_to_string(path)?;
        Self::from_yaml(&content)
    }

    /// Parse beatmap from YAML string
    pub fn from_yaml(yaml: &str) -> Result<Self, BeatmapError> {
        let map: Self = serde_yaml::from_str(yaml)?;
        map.validate()?;
        Ok(map)
    }

    /// Serialize beatmap to YAML string
    pub fn to_yaml(&self) -> Result<String, BeatmapError> {
        Ok(serde_yaml::to_string(self)?)
    }

    /// Save beatmap to file
    pub fn save(&self, path: &Path) -> Result<(), BeatmapError> {
        let yaml = self.to_yaml()?;
        std::fs::write(path, yaml)?;
        Ok(())
    }

    /// Validate beatmap data
    pub fn validate(&self) -> Result<(), BeatmapError> {
        if self.timing.is_empty() {
            return Err(BeatmapError::Validation(
                "At least one timing point is required".into(),
            ));
        }
        if self.difficulty.keys < 1 || self.difficulty.keys > 10 {
            return Err(BeatmapError::Validation(
                "Key count must be between 1 and 10".into(),
            ));
        }
        Ok(())
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

    /// Get note count (non-hold notes)
    pub fn note_count(&self) -> usize {
        self.hit_objects.iter().filter(|o| !o.is_hold()).count()
    }

    /// Get hold note count
    pub fn hold_count(&self) -> usize {
        self.hit_objects.iter().filter(|o| o.is_hold()).count()
    }

    /// Get BPM at time 0 (or first timing point)
    pub fn bpm(&self) -> f64 {
        self.timing.first().map(|t| t.bpm).unwrap_or(120.0)
    }
}

/// Beatmap loading/saving errors
#[derive(Debug, thiserror::Error)]
pub enum BeatmapError {
    #[error("IO error: {0}")]
    Io(#[from] std::io::Error),
    #[error("YAML parse error: {0}")]
    Yaml(#[from] serde_yaml::Error),
    #[error("Validation error: {0}")]
    Validation(String),
}