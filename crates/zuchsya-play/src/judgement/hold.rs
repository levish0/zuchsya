//! Hold note judgement (osu!mania style)
//!
//! - Head: judged when pressed
//! - Body: tracks holding state, breaks combo if released early
//! - Tail: judged when released, 1.5x lenient timing, capped to Meh if head missed or body broken

use bevy::prelude::*;
use zuchsya_core::HitResult;

use crate::input::KeyState;
use crate::note::{HoldNoteBody, HoldNoteHead, HoldNoteId, HoldNoteState, HoldNoteTail};
use crate::scroll::GameTime;

use super::{JudgementConfig, JudgementEvent, ScoreState};

/// Tail release timing lenience multiplier (osu! uses 1.5)
const RELEASE_WINDOW_LENIENCE: f64 = 1.5;

/// Process hold note head hits (when key is pressed)
pub fn process_hold_head_hits(
    key_state: Res<KeyState>,
    game_time: Res<GameTime>,
    config: Res<JudgementConfig>,
    mut hold_heads: Query<(&HoldNoteHead, &mut HoldNoteState)>,
    mut score: ResMut<ScoreState>,
    mut events: MessageWriter<JudgementEvent>,
) {
    for (column_idx, just_pressed) in key_state.just_pressed.iter().enumerate() {
        if !just_pressed {
            continue;
        }

        let column = column_idx as u8;

        // Find and update the closest head
        let mut best_offset: Option<f64> = None;
        for (head, mut state) in hold_heads.iter_mut() {
            if head.column != column || state.head_hit {
                continue;
            }

            let time_offset = head.start_time_ms - game_time.current_ms;

            if !config.hit_windows.can_be_hit(time_offset) {
                continue;
            }

            let dominated = best_offset.is_some_and(|best| time_offset.abs() >= best.abs());
            if dominated {
                continue;
            }
            best_offset = Some(time_offset);

            if let Some(result) = config.hit_windows.result_for(time_offset) {
                state.head_hit = true;
                state.head_result = Some(result);
                state.is_holding = true;

                score.add_judgement(result);

                events.write(JudgementEvent {
                    result,
                    column,
                    time_offset,
                });

                break; // Only hit one note per key press
            }
        }
    }
}

/// Process hold note head misses (head not hit in time)
pub fn process_hold_head_misses(
    game_time: Res<GameTime>,
    config: Res<JudgementConfig>,
    mut hold_heads: Query<(&HoldNoteHead, &mut HoldNoteState)>,
    mut score: ResMut<ScoreState>,
    mut events: MessageWriter<JudgementEvent>,
) {
    let miss_window = config.hit_windows.window_for(HitResult::Miss);

    for (head, mut state) in hold_heads.iter_mut() {
        if state.head_hit {
            continue;
        }

        let time_offset = head.start_time_ms - game_time.current_ms;

        if time_offset < -miss_window {
            state.head_hit = true;
            state.head_result = Some(HitResult::Miss);
            state.hold_broken = true;

            score.add_judgement(HitResult::Miss);

            events.write(JudgementEvent {
                result: HitResult::Miss,
                column: head.column,
                time_offset,
            });
        }
    }
}

/// Update hold state based on key being held or released
pub fn update_hold_state(
    key_state: Res<KeyState>,
    game_time: Res<GameTime>,
    mut hold_heads: Query<(&HoldNoteHead, &mut HoldNoteState)>,
    mut score: ResMut<ScoreState>,
) {
    for (head, mut state) in hold_heads.iter_mut() {
        if !state.head_hit || state.tail_judged {
            continue;
        }

        let column = head.column as usize;
        let is_key_held = key_state.pressed.get(column).copied().unwrap_or(false);
        let current_time = game_time.current_ms;

        let in_hold_duration =
            current_time >= head.start_time_ms && current_time < head.end_time_ms;

        if in_hold_duration {
            if state.is_holding && !is_key_held {
                state.is_holding = false;
                state.hold_broken = true;
                score.add_combo_break();
            } else if !state.is_holding && is_key_held && !state.hold_broken {
                state.is_holding = true;
            }
        }
    }
}

/// Process hold note tail (when key is released or tail time passes)
pub fn process_hold_release(
    key_state: Res<KeyState>,
    game_time: Res<GameTime>,
    config: Res<JudgementConfig>,
    mut hold_heads: Query<(&HoldNoteHead, &mut HoldNoteState)>,
    mut score: ResMut<ScoreState>,
    mut events: MessageWriter<JudgementEvent>,
) {
    for (head, mut state) in hold_heads.iter_mut() {
        if !state.head_hit || state.tail_judged {
            continue;
        }

        let column = head.column as usize;
        let is_key_held = key_state.pressed.get(column).copied().unwrap_or(false);
        let just_released = key_state.just_released.get(column).copied().unwrap_or(false);
        let current_time = game_time.current_ms;

        let tail_time_offset = head.end_time_ms - current_time;
        let lenient_offset = tail_time_offset / RELEASE_WINDOW_LENIENCE;

        let should_judge = if just_released && state.is_holding {
            true
        } else if !is_key_held && current_time >= head.end_time_ms {
            true
        } else if is_key_held && current_time >= head.end_time_ms {
            true
        } else {
            false
        };

        if !should_judge {
            continue;
        }

        state.tail_judged = true;
        state.is_holding = false;

        let mut result = if is_key_held && current_time >= head.end_time_ms {
            config
                .hit_windows
                .result_for(0.0)
                .unwrap_or(HitResult::Perfect)
        } else {
            config
                .hit_windows
                .result_for(lenient_offset)
                .unwrap_or(HitResult::Miss)
        };

        // Cap result to Meh if head was missed or hold was broken
        let has_combo_break = state.head_result == Some(HitResult::Miss) || state.hold_broken;
        if has_combo_break && result > HitResult::Meh {
            result = HitResult::Meh;
        }

        score.add_judgement(result);

        events.write(JudgementEvent {
            result,
            column: head.column,
            time_offset: tail_time_offset,
        });
    }
}

/// Cleanup hold notes after they've been fully judged
pub fn cleanup_hold_notes(
    mut commands: Commands,
    game_time: Res<GameTime>,
    hold_heads: Query<(Entity, &HoldNoteHead, &HoldNoteState, &HoldNoteId)>,
    hold_bodies: Query<(Entity, &HoldNoteBody, &HoldNoteId)>,
    hold_tails: Query<(Entity, &HoldNoteTail, &HoldNoteId)>,
) {
    for (head_entity, head, state, head_id) in hold_heads.iter() {
        if !state.tail_judged {
            continue;
        }

        let time_since_end = game_time.current_ms - head.end_time_ms;
        if time_since_end < 200.0 {
            continue;
        }

        commands.entity(head_entity).despawn();

        for (body_entity, _, body_id) in hold_bodies.iter() {
            if body_id.0 == head_id.0 {
                commands.entity(body_entity).despawn();
            }
        }

        for (tail_entity, _, tail_id) in hold_tails.iter() {
            if tail_id.0 == head_id.0 {
                commands.entity(tail_entity).despawn();
            }
        }
    }
}
