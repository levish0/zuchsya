//! Regular note judgement

use bevy::prelude::*;
use zuchsya_core::HitResult;

use crate::input::KeyState;
use crate::note::Note;
use crate::scroll::GameTime;

use super::{JudgementConfig, JudgementEvent, ScoreState};

/// Process regular note hits (when key is pressed)
pub fn process_note_hits(
    key_state: Res<KeyState>,
    game_time: Res<GameTime>,
    config: Res<JudgementConfig>,
    mut notes: Query<(Entity, &mut Note)>,
    mut score: ResMut<ScoreState>,
    mut events: MessageWriter<JudgementEvent>,
) {
    for (column_idx, just_pressed) in key_state.just_pressed.iter().enumerate() {
        if !just_pressed {
            continue;
        }

        let column = column_idx as u8;
        let mut closest_note: Option<(Entity, f64)> = None;

        for (entity, note) in notes.iter() {
            if note.column != column || note.hit {
                continue;
            }

            let time_offset = note.time_ms - game_time.current_ms;

            if !config.hit_windows.can_be_hit(time_offset) {
                continue;
            }

            match closest_note {
                None => closest_note = Some((entity, time_offset)),
                Some((_, prev_offset)) => {
                    if time_offset.abs() < prev_offset.abs() {
                        closest_note = Some((entity, time_offset));
                    }
                }
            }
        }

        if let Some((entity, time_offset)) = closest_note {
            if let Some(result) = config.hit_windows.result_for(time_offset) {
                if let Ok((_, mut note)) = notes.get_mut(entity) {
                    note.hit = true;
                }

                score.add_judgement(result);

                events.write(JudgementEvent {
                    result,
                    column,
                    time_offset,
                });
            }
        }
    }
}

/// Process regular note misses (note passed without being hit)
pub fn process_note_misses(
    game_time: Res<GameTime>,
    config: Res<JudgementConfig>,
    mut notes: Query<(Entity, &mut Note)>,
    mut score: ResMut<ScoreState>,
    mut events: MessageWriter<JudgementEvent>,
) {
    let miss_window = config.hit_windows.window_for(HitResult::Miss);

    for (_entity, mut note) in notes.iter_mut() {
        if note.hit {
            continue;
        }

        let time_offset = note.time_ms - game_time.current_ms;

        if time_offset < -miss_window {
            note.hit = true;

            score.add_judgement(HitResult::Miss);

            events.write(JudgementEvent {
                result: HitResult::Miss,
                column: note.column,
                time_offset,
            });
        }
    }
}

/// Cleanup hit notes after delay
pub fn cleanup_hit_notes(
    mut commands: Commands,
    notes: Query<(Entity, &Note)>,
    game_time: Res<GameTime>,
) {
    for (entity, note) in notes.iter() {
        if note.hit {
            let time_since_hit = game_time.current_ms - note.time_ms;
            if time_since_hit > 200.0 {
                commands.entity(entity).despawn();
            }
        }
    }
}
