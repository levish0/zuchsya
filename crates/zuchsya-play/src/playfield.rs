//! Playfield rendering - Columns and hit target

use bevy::prelude::*;
use zuchsya_core::GameState;

/// Playfield constants (based on osu!mania)
pub const COLUMN_WIDTH: f32 = 80.0;
pub const COLUMN_SPACING: f32 = 2.0;
pub const HIT_TARGET_HEIGHT: f32 = 20.0;
pub const HIT_TARGET_Y: f32 = -250.0; // Distance from center

/// Column colors for visual distinction
const COLUMN_COLORS: [Color; 4] = [
    Color::srgb(0.8, 0.2, 0.2), // Red
    Color::srgb(0.2, 0.2, 0.8), // Blue
    Color::srgb(0.2, 0.2, 0.8), // Blue
    Color::srgb(0.8, 0.2, 0.2), // Red
];

pub struct PlayfieldPlugin;

impl Plugin for PlayfieldPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(PlayfieldConfig::default())
            .add_systems(OnEnter(GameState::Playing), setup_playfield)
            .add_systems(OnExit(GameState::Playing), cleanup_playfield);
    }
}

/// Playfield configuration
#[derive(Resource)]
pub struct PlayfieldConfig {
    pub key_count: u8,
    pub column_width: f32,
    pub column_spacing: f32,
}

impl Default for PlayfieldConfig {
    fn default() -> Self {
        Self {
            key_count: 4,
            column_width: COLUMN_WIDTH,
            column_spacing: COLUMN_SPACING,
        }
    }
}

impl PlayfieldConfig {
    /// Get total playfield width
    pub fn total_width(&self) -> f32 {
        self.key_count as f32 * self.column_width
            + (self.key_count - 1) as f32 * self.column_spacing
    }

    /// Get X position for a column (0-indexed, centered)
    pub fn column_x(&self, column: u8) -> f32 {
        let total_width = self.total_width();
        let start_x = -total_width / 2.0 + self.column_width / 2.0;
        start_x + column as f32 * (self.column_width + self.column_spacing)
    }
}

/// Marker for playfield root
#[derive(Component)]
pub struct Playfield;

/// Column component
#[derive(Component)]
pub struct Column {
    pub index: u8,
}

/// Hit target (judgement line)
#[derive(Component)]
pub struct HitTarget {
    pub column: u8,
}

/// Column background
#[derive(Component)]
pub struct ColumnBackground {
    pub column: u8,
}

fn setup_playfield(
    mut commands: Commands,
    config: Res<PlayfieldConfig>,
) {
    // Spawn playfield container
    commands
        .spawn((
            Playfield,
            Transform::default(),
            Visibility::default(),
        ))
        .with_children(|parent| {
            // Spawn columns
            for i in 0..config.key_count {
                let x = config.column_x(i);
                let color = COLUMN_COLORS[i as usize % COLUMN_COLORS.len()];

                // Column background (darker)
                parent.spawn((
                    ColumnBackground { column: i },
                    Sprite {
                        color: color.with_alpha(0.15),
                        custom_size: Some(Vec2::new(config.column_width, 600.0)),
                        ..default()
                    },
                    Transform::from_xyz(x, 0.0, 0.0),
                ));

                // Hit target (judgement line)
                parent.spawn((
                    HitTarget { column: i },
                    Sprite {
                        color: color.with_alpha(0.8),
                        custom_size: Some(Vec2::new(config.column_width, HIT_TARGET_HEIGHT)),
                        ..default()
                    },
                    Transform::from_xyz(x, HIT_TARGET_Y, 1.0),
                ));

                // Column marker
                parent.spawn((
                    Column { index: i },
                    Transform::from_xyz(x, 0.0, 0.0),
                    Visibility::default(),
                ));
            }

            // Playfield border (left)
            parent.spawn((
                Sprite {
                    color: Color::WHITE.with_alpha(0.5),
                    custom_size: Some(Vec2::new(2.0, 600.0)),
                    ..default()
                },
                Transform::from_xyz(-config.total_width() / 2.0 - 1.0, 0.0, 2.0),
            ));

            // Playfield border (right)
            parent.spawn((
                Sprite {
                    color: Color::WHITE.with_alpha(0.5),
                    custom_size: Some(Vec2::new(2.0, 600.0)),
                    ..default()
                },
                Transform::from_xyz(config.total_width() / 2.0 + 1.0, 0.0, 2.0),
            ));
        });
}

fn cleanup_playfield(
    mut commands: Commands,
    query: Query<Entity, With<Playfield>>,
) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}