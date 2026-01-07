//! Judgement system - hit detection and scoring

mod hold;
mod note;
mod score;

pub use score::ScoreState;

use bevy::prelude::*;
use zuchsya_core::{GameState, HitResult, HitWindows};

pub struct JudgementPlugin;

impl Plugin for JudgementPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScoreState::default())
            .insert_resource(JudgementConfig::default())
            .add_message::<JudgementEvent>()
            .add_systems(OnEnter(GameState::Playing), reset_score)
            .add_systems(
                Update,
                (
                    // Regular notes
                    note::process_note_hits,
                    note::process_note_misses,
                    note::cleanup_hit_notes,
                    // Hold notes
                    hold::process_hold_head_hits,
                    hold::process_hold_head_misses,
                    hold::update_hold_state,
                    hold::process_hold_release,
                    hold::cleanup_hold_notes,
                )
                    .run_if(in_state(GameState::Playing)),
            );
    }
}

/// Configuration for judgement
#[derive(Resource)]
pub struct JudgementConfig {
    pub hit_windows: HitWindows,
}

impl Default for JudgementConfig {
    fn default() -> Self {
        Self {
            hit_windows: HitWindows::new(5.0), // OD 5 default
        }
    }
}

/// Event fired when a note is judged
#[derive(Message)]
pub struct JudgementEvent {
    pub result: HitResult,
    pub column: u8,
    pub time_offset: f64,
}

fn reset_score(mut score: ResMut<ScoreState>) {
    *score = ScoreState::default();
}
