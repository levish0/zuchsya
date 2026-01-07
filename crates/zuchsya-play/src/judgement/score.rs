//! Score state and calculation (osu!mania scoring)

use bevy::prelude::*;
use zuchsya_core::HitResult;

/// Current score state (osu!mania scoring)
/// Max score = 1,000,000 = 150,000 (combo) + 850,000 (accuracy)
#[derive(Resource)]
pub struct ScoreState {
    pub combo: u32,
    pub max_combo: u32,
    pub accuracy: f64,

    // Judgement counts
    pub perfect_count: u32,
    pub great_count: u32,
    pub good_count: u32,
    pub ok_count: u32,
    pub meh_count: u32,
    pub miss_count: u32,

    // For score calculation
    total_objects: u32,
    objects_judged: u32,
    combo_score: f64,
    max_combo_score: f64,
    accuracy_sum: f64,
}

impl Default for ScoreState {
    fn default() -> Self {
        Self {
            combo: 0,
            max_combo: 0,
            accuracy: 1.0,
            perfect_count: 0,
            great_count: 0,
            good_count: 0,
            ok_count: 0,
            meh_count: 0,
            miss_count: 0,
            total_objects: 0,
            objects_judged: 0,
            combo_score: 0.0,
            max_combo_score: 0.0,
            accuracy_sum: 0.0,
        }
    }
}

impl ScoreState {
    /// Set total object count (call before gameplay starts)
    pub fn set_total_objects(&mut self, count: u32) {
        self.total_objects = count;
    }

    pub fn add_judgement(&mut self, result: HitResult) {
        // Update counts
        match result {
            HitResult::Perfect => self.perfect_count += 1,
            HitResult::Great => self.great_count += 1,
            HitResult::Good => self.good_count += 1,
            HitResult::Ok => self.ok_count += 1,
            HitResult::Meh => self.meh_count += 1,
            HitResult::Miss => self.miss_count += 1,
        }

        self.objects_judged += 1;

        // Update combo
        if result.breaks_combo() {
            self.combo = 0;
        } else {
            self.combo += 1;
            if self.combo > self.max_combo {
                self.max_combo = self.combo;
            }
        }

        // Update combo score
        let combo_base: f64 = 4.0;
        let combo_multiplier = (self.combo as f64)
            .log(combo_base)
            .max(0.5)
            .min(combo_base.log(400.0));
        let hit_combo_value = result.combo_score_weight() * combo_multiplier;
        let max_combo_value = 1.0 * combo_multiplier;

        self.combo_score += hit_combo_value;
        self.max_combo_score += max_combo_value;

        // Update accuracy
        self.accuracy_sum += result.accuracy_weight();
        self.accuracy = self.accuracy_sum / self.objects_judged as f64;
    }

    /// Add a combo break without affecting accuracy (for hold body breaks)
    pub fn add_combo_break(&mut self) {
        self.combo = 0;
    }

    /// Calculate current score (0 - 1,000,000)
    pub fn score(&self) -> i64 {
        if self.objects_judged == 0 {
            return 0;
        }

        let total = if self.total_objects > 0 {
            self.total_objects
        } else {
            self.objects_judged
        };

        let accuracy_progress = self.objects_judged as f64 / total as f64;
        let combo_progress = if self.max_combo_score > 0.0 {
            self.combo_score / self.max_combo_score
        } else {
            1.0
        };

        // osu!mania formula: 150000 * combo + 850000 * acc^(2+2*acc)
        let combo_portion = 150_000.0 * combo_progress;
        let acc_exp = 2.0 + 2.0 * self.accuracy;
        let accuracy_portion = 850_000.0 * self.accuracy.powf(acc_exp) * accuracy_progress;

        (combo_portion + accuracy_portion) as i64
    }

    pub fn total_notes(&self) -> u32 {
        self.perfect_count
            + self.great_count
            + self.good_count
            + self.ok_count
            + self.meh_count
            + self.miss_count
    }
}