//! Note update and cleanup systems

use bevy::prelude::*;

use crate::playfield::HIT_TARGET_Y;
use crate::scroll::{GameTime, ScrollConfig};

use super::types::*;

pub fn update_note_positions(
    game_time: Res<GameTime>,
    scroll_config: Res<ScrollConfig>,
    mut notes: Query<(&Note, &mut Transform), Without<HoldNoteHead>>,
    mut hold_heads: Query<(&HoldNoteHead, &mut Transform), Without<Note>>,
    mut hold_bodies: Query<
        (&HoldNoteBody, &mut Transform, &mut Sprite),
        (Without<Note>, Without<HoldNoteHead>, Without<HoldNoteTail>),
    >,
    mut hold_tails: Query<
        (&HoldNoteTail, &mut Transform),
        (Without<Note>, Without<HoldNoteHead>, Without<HoldNoteBody>),
    >,
) {
    const PLAYFIELD_HEIGHT: f32 = 600.0;

    // Update regular notes
    for (note, mut transform) in notes.iter_mut() {
        if note.hit {
            continue;
        }
        let time_diff = note.time_ms - game_time.current_ms;
        let y_offset = scroll_config.time_to_y(time_diff, PLAYFIELD_HEIGHT);
        transform.translation.y = HIT_TARGET_Y + y_offset;
    }

    // Update hold note heads
    for (hold_head, mut transform) in hold_heads.iter_mut() {
        let time_diff = hold_head.start_time_ms - game_time.current_ms;
        let y_offset = scroll_config.time_to_y(time_diff, PLAYFIELD_HEIGHT);
        transform.translation.y = HIT_TARGET_Y + y_offset;
    }

    // Update hold note bodies
    for (hold_body, mut transform, mut sprite) in hold_bodies.iter_mut() {
        let start_time_diff = hold_body.start_time_ms - game_time.current_ms;
        let end_time_diff = hold_body.end_time_ms - game_time.current_ms;

        let start_y = HIT_TARGET_Y + scroll_config.time_to_y(start_time_diff, PLAYFIELD_HEIGHT);
        let end_y = HIT_TARGET_Y + scroll_config.time_to_y(end_time_diff, PLAYFIELD_HEIGHT);

        let body_height = (start_y - end_y).abs();
        let center_y = (start_y + end_y) / 2.0;

        transform.translation.y = center_y;
        if let Some(size) = &mut sprite.custom_size {
            size.y = body_height;
        }
    }

    // Update hold note tails
    for (hold_tail, mut transform) in hold_tails.iter_mut() {
        let time_diff = hold_tail.end_time_ms - game_time.current_ms;
        let y_offset = scroll_config.time_to_y(time_diff, PLAYFIELD_HEIGHT);
        transform.translation.y = HIT_TARGET_Y + y_offset;
    }
}

pub fn cleanup_notes(
    mut commands: Commands,
    notes: Query<Entity, With<Note>>,
    hold_heads: Query<Entity, With<HoldNoteHead>>,
    hold_bodies: Query<Entity, With<HoldNoteBody>>,
    hold_tails: Query<Entity, With<HoldNoteTail>>,
) {
    for entity in notes
        .iter()
        .chain(hold_heads.iter())
        .chain(hold_bodies.iter())
        .chain(hold_tails.iter())
    {
        commands.entity(entity).despawn();
    }
}
