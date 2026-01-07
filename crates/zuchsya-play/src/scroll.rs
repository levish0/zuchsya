//! Scroll system for notes

use bevy::prelude::*;
use zuchsya_core::GameState;

pub struct ScrollPlugin;

impl Plugin for ScrollPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(ScrollConfig::default())
            .add_systems(OnEnter(GameState::Playing), reset_game_time)
            .add_systems(
                Update,
                (update_scroll, adjust_scroll_speed).run_if(in_state(GameState::Playing)),
            );
    }
}

/// Scroll configuration
#[derive(Resource)]
pub struct ScrollConfig {
    /// Scroll speed (1-40, like osu!mania)
    pub speed: u8,
    /// Time range in ms (calculated from speed)
    pub time_range_ms: f64,
}

impl Default for ScrollConfig {
    fn default() -> Self {
        Self::from_speed(20) // osu default is 8, but we use 20 for better readability
    }
}

impl ScrollConfig {
    /// Create config from speed value (1-40)
    pub fn from_speed(speed: u8) -> Self {
        let speed = speed.clamp(1, 40);
        // osu!mania formula: MIN_TIME_RANGE = 290ms (speed 40), MAX_TIME_RANGE = 11485ms (speed 1)
        let time_range_ms = Self::calculate_time_range(speed);
        Self {
            speed,
            time_range_ms,
        }
    }

    /// Calculate time range from speed
    /// Based on osu!mania DrawableManiaRuleset
    fn calculate_time_range(speed: u8) -> f64 {
        const MIN_TIME_RANGE: f64 = 290.0;
        const MAX_TIME_RANGE: f64 = 11485.0;

        // Linear interpolation: speed 1 -> MAX, speed 40 -> MIN
        let t = (speed - 1) as f64 / 39.0;
        MAX_TIME_RANGE - t * (MAX_TIME_RANGE - MIN_TIME_RANGE)
    }

    /// Get Y position for a note given its time relative to current time
    /// Returns position from hit target (negative = above, positive = below)
    pub fn time_to_y(&self, time_diff_ms: f64, playfield_height: f32) -> f32 {
        // time_diff_ms: positive = note is in future, negative = note is in past
        let ratio = time_diff_ms / self.time_range_ms;
        (ratio * playfield_height as f64) as f32
    }
}

/// Current game time (for scroll calculations)
#[derive(Resource, Default)]
pub struct GameTime {
    /// Current time in milliseconds
    pub current_ms: f64,
}

fn update_scroll(time: Res<Time>, mut game_time: ResMut<GameTime>) {
    // Update game time
    game_time.current_ms += time.delta_secs_f64() * 1000.0;
}

/// System to reset game time when entering Playing state
pub fn reset_game_time(mut game_time: ResMut<GameTime>) {
    game_time.current_ms = 0.0;
}

/// Adjust scroll speed with F3/F4 keys
fn adjust_scroll_speed(keyboard: Res<ButtonInput<KeyCode>>, mut config: ResMut<ScrollConfig>) {
    let mut new_speed = config.speed as i32;

    if keyboard.just_pressed(KeyCode::F4) {
        new_speed += 2; // Faster
    }
    if keyboard.just_pressed(KeyCode::F3) {
        new_speed -= 2; // Slower
    }

    new_speed = new_speed.clamp(1, 40);

    if new_speed != config.speed as i32 {
        *config = ScrollConfig::from_speed(new_speed as u8);
        info!("Scroll speed: {} (time range: {:.0}ms)", config.speed, config.time_range_ms);
    }
}
