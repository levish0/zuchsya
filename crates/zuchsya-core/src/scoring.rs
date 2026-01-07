//! Scoring and judgement types

use serde::{Deserialize, Serialize};

/// Hit result (judgement)
/// Ordered from worst to best: Miss < Meh < Ok < Good < Great < Perfect
#[derive(Debug, Clone, Copy, PartialEq, Eq, Hash, PartialOrd, Ord, Serialize, Deserialize)]
pub enum HitResult {
    Miss,
    Meh,
    Ok,
    Good,
    Great,
    Perfect,
}

impl HitResult {
    /// Get base score value (for display/legacy)
    pub fn base_score(&self) -> i32 {
        match self {
            Self::Perfect => 305,
            Self::Great => 300,
            Self::Good => 200,
            Self::Ok => 100,
            Self::Meh => 50,
            Self::Miss => 0,
        }
    }

    /// Get accuracy weight (0.0 - 1.0)
    pub fn accuracy_weight(&self) -> f64 {
        match self {
            Self::Perfect => 1.0,
            Self::Great => 1.0,  // Great = same accuracy as Perfect in mania
            Self::Good => 2.0 / 3.0,
            Self::Ok => 1.0 / 3.0,
            Self::Meh => 1.0 / 6.0,
            Self::Miss => 0.0,
        }
    }

    /// Get combo score weight (0.0 - 1.0) for osu!mania scoring
    pub fn combo_score_weight(&self) -> f64 {
        match self {
            Self::Perfect => 1.0,
            Self::Great => 1.0,
            Self::Good => 2.0 / 3.0,
            Self::Ok => 1.0 / 3.0,
            Self::Meh => 1.0 / 6.0,
            Self::Miss => 0.0,
        }
    }

    /// Check if this result breaks combo
    pub fn breaks_combo(&self) -> bool {
        matches!(self, Self::Miss)
    }
}

/// Score rank
#[derive(Debug, Clone, Copy, PartialEq, Eq, PartialOrd, Ord, Serialize, Deserialize)]
pub enum ScoreRank {
    D,
    C,
    B,
    A,
    S,
    /// SS (all Perfect/Great)
    X,
}

impl ScoreRank {
    /// Calculate rank from accuracy
    pub fn from_accuracy(accuracy: f64, has_imperfect: bool) -> Self {
        if accuracy >= 0.95 {
            if has_imperfect {
                Self::S
            } else {
                Self::X // SS
            }
        } else if accuracy >= 0.90 {
            Self::A
        } else if accuracy >= 0.80 {
            Self::B
        } else if accuracy >= 0.70 {
            Self::C
        } else {
            Self::D
        }
    }

    /// Get display string
    pub fn as_str(&self) -> &'static str {
        match self {
            Self::X => "SS",
            Self::S => "S",
            Self::A => "A",
            Self::B => "B",
            Self::C => "C",
            Self::D => "D",
        }
    }
}

/// Difficulty range for hit windows
/// (max, mid, min) corresponds to (OD 0, OD 5, OD 10)
#[derive(Debug, Clone, Copy)]
pub struct DifficultyRange {
    pub max: f64, // OD 0
    pub mid: f64, // OD 5
    pub min: f64, // OD 10
}

impl DifficultyRange {
    pub const fn new(max: f64, mid: f64, min: f64) -> Self {
        Self { max, mid, min }
    }

    /// Calculate value for given OD
    pub fn at(&self, od: f64) -> f64 {
        if od > 5.0 {
            self.mid + (self.min - self.mid) * (od - 5.0) / 5.0
        } else if od < 5.0 {
            self.mid + (self.max - self.mid) * (5.0 - od) / 5.0
        } else {
            self.mid
        }
    }
}

/// Hit windows based on osu!mania
pub struct HitWindows {
    pub overall_difficulty: f64,
}

impl HitWindows {
    // Hit window ranges (max/mid/min = OD 0/5/10) in milliseconds
    pub const PERFECT: DifficultyRange = DifficultyRange::new(22.4, 19.4, 13.9);
    pub const GREAT: DifficultyRange = DifficultyRange::new(64.0, 49.0, 34.0);
    pub const GOOD: DifficultyRange = DifficultyRange::new(97.0, 82.0, 67.0);
    pub const OK: DifficultyRange = DifficultyRange::new(127.0, 112.0, 97.0);
    pub const MEH: DifficultyRange = DifficultyRange::new(151.0, 136.0, 121.0);
    pub const MISS: DifficultyRange = DifficultyRange::new(188.0, 173.0, 158.0);

    pub fn new(od: f64) -> Self {
        Self {
            overall_difficulty: od,
        }
    }

    /// Get window for a specific result
    pub fn window_for(&self, result: HitResult) -> f64 {
        let range = match result {
            HitResult::Perfect => Self::PERFECT,
            HitResult::Great => Self::GREAT,
            HitResult::Good => Self::GOOD,
            HitResult::Ok => Self::OK,
            HitResult::Meh => Self::MEH,
            HitResult::Miss => Self::MISS,
        };
        range.at(self.overall_difficulty).floor() + 0.5
    }

    /// Determine result from time offset
    pub fn result_for(&self, time_offset: f64) -> Option<HitResult> {
        let offset = time_offset.abs();

        if offset <= self.window_for(HitResult::Perfect) {
            Some(HitResult::Perfect)
        } else if offset <= self.window_for(HitResult::Great) {
            Some(HitResult::Great)
        } else if offset <= self.window_for(HitResult::Good) {
            Some(HitResult::Good)
        } else if offset <= self.window_for(HitResult::Ok) {
            Some(HitResult::Ok)
        } else if offset <= self.window_for(HitResult::Meh) {
            Some(HitResult::Meh)
        } else if offset <= self.window_for(HitResult::Miss) {
            Some(HitResult::Miss)
        } else {
            None // Outside all windows
        }
    }

    /// Check if can still be hit
    pub fn can_be_hit(&self, time_offset: f64) -> bool {
        time_offset.abs() <= self.window_for(HitResult::Miss)
    }
}
