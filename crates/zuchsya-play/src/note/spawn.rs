//! Note spawning systems

use bevy::prelude::*;

use crate::playfield::PlayfieldConfig;

use super::types::*;

pub fn spawn_notes(
    mut commands: Commands,
    hit_objects: Option<Res<CurrentHitObjects>>,
    config: Res<PlayfieldConfig>,
) {
    let objects = match hit_objects {
        Some(ref ho) if !ho.objects.is_empty() => &ho.objects,
        _ => {
            // No beatmap loaded, spawn test notes for debugging
            spawn_test_notes(&mut commands, &config);
            return;
        }
    };

    let mut hold_note_id: u32 = 0;

    for obj in objects.iter() {
        let column = obj.lane;
        let x = config.column_x(column);
        let color = NOTE_COLORS[column as usize % NOTE_COLORS.len()];

        if obj.is_hold() {
            let end_time = obj.end_time();
            let id = HoldNoteId(hold_note_id);
            hold_note_id += 1;

            // Hold note head (with state tracking)
            commands.spawn((
                id,
                HoldNoteHead {
                    column,
                    start_time_ms: obj.time,
                    end_time_ms: end_time,
                },
                HoldNoteState::default(),
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(config.column_width - 4.0, NOTE_HEIGHT)),
                    ..default()
                },
                Transform::from_xyz(x, 1000.0, 5.0),
            ));

            // Hold note body
            commands.spawn((
                id,
                HoldNoteBody {
                    column,
                    start_time_ms: obj.time,
                    end_time_ms: end_time,
                },
                Sprite {
                    color: color.with_alpha(0.6),
                    custom_size: Some(Vec2::new(config.column_width - 8.0, 100.0)), // Will be updated
                    ..default()
                },
                Transform::from_xyz(x, 1000.0, 4.0),
            ));

            // Hold note tail
            commands.spawn((
                id,
                HoldNoteTail {
                    column,
                    start_time_ms: obj.time,
                    end_time_ms: end_time,
                },
                Sprite {
                    color: color.with_alpha(0.8),
                    custom_size: Some(Vec2::new(config.column_width - 4.0, NOTE_HEIGHT / 2.0)),
                    ..default()
                },
                Transform::from_xyz(x, 1000.0, 5.0),
            ));
        } else {
            // Regular tap note
            commands.spawn((
                Note {
                    column,
                    time_ms: obj.time,
                    hit: false,
                },
                Sprite {
                    color,
                    custom_size: Some(Vec2::new(config.column_width - 4.0, NOTE_HEIGHT)),
                    ..default()
                },
                // Initial position (will be updated by scroll system)
                Transform::from_xyz(x, 1000.0, 5.0),
            ));
        }
    }
}

/// Spawn test notes for debugging when no beatmap is loaded
fn spawn_test_notes(commands: &mut Commands, config: &PlayfieldConfig) {
    // Create a simple test pattern
    let test_times: Vec<(u8, f64)> = vec![
        (0, 1000.0),
        (1, 1250.0),
        (2, 1500.0),
        (3, 1750.0),
        (0, 2000.0),
        (3, 2000.0),
        (1, 2250.0),
        (2, 2250.0),
        (0, 2500.0),
        (1, 2500.0),
        (2, 2500.0),
        (3, 2500.0),
    ];

    for (column, time_ms) in test_times {
        let x = config.column_x(column);
        let color = NOTE_COLORS[column as usize % NOTE_COLORS.len()];

        commands.spawn((
            Note {
                column,
                time_ms,
                hit: false,
            },
            Sprite {
                color,
                custom_size: Some(Vec2::new(config.column_width - 4.0, NOTE_HEIGHT)),
                ..default()
            },
            Transform::from_xyz(x, 1000.0, 5.0),
        ));
    }
}
