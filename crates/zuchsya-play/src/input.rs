//! Input handling for gameplay

use bevy::prelude::*;
use crate::app::state::GameState;

pub struct InputPlugin;

impl Plugin for InputPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(KeyBindings::default())
            .insert_resource(KeyState::default())
            .add_systems(
                Update,
                update_key_state.run_if(in_state(GameState::Playing)),
            );
    }
}

/// Key bindings for each column
#[derive(Resource)]
pub struct KeyBindings {
    pub keys: Vec<KeyCode>,
}

impl Default for KeyBindings {
    fn default() -> Self {
        // Default 4K bindings
        Self {
            keys: vec![KeyCode::KeyD, KeyCode::KeyF, KeyCode::KeyJ, KeyCode::KeyK],
        }
    }
}

impl KeyBindings {
    /// Get bindings for specific key count
    pub fn for_key_count(key_count: u8) -> Self {
        let keys = match key_count {
            1 => vec![KeyCode::Space],
            2 => vec![KeyCode::KeyF, KeyCode::KeyJ],
            3 => vec![KeyCode::KeyF, KeyCode::Space, KeyCode::KeyJ],
            4 => vec![KeyCode::KeyD, KeyCode::KeyF, KeyCode::KeyJ, KeyCode::KeyK],
            5 => vec![
                KeyCode::KeyD,
                KeyCode::KeyF,
                KeyCode::Space,
                KeyCode::KeyJ,
                KeyCode::KeyK,
            ],
            6 => vec![
                KeyCode::KeyS,
                KeyCode::KeyD,
                KeyCode::KeyF,
                KeyCode::KeyJ,
                KeyCode::KeyK,
                KeyCode::KeyL,
            ],
            7 => vec![
                KeyCode::KeyS,
                KeyCode::KeyD,
                KeyCode::KeyF,
                KeyCode::Space,
                KeyCode::KeyJ,
                KeyCode::KeyK,
                KeyCode::KeyL,
            ],
            8 => vec![
                KeyCode::KeyA,
                KeyCode::KeyS,
                KeyCode::KeyD,
                KeyCode::KeyF,
                KeyCode::KeyJ,
                KeyCode::KeyK,
                KeyCode::KeyL,
                KeyCode::Semicolon,
            ],
            _ => vec![KeyCode::KeyD, KeyCode::KeyF, KeyCode::KeyJ, KeyCode::KeyK],
        };
        Self { keys }
    }
}

/// Current key state
#[derive(Resource, Default)]
pub struct KeyState {
    /// Currently pressed keys (by column index)
    pub pressed: Vec<bool>,
    /// Keys that were just pressed this frame
    pub just_pressed: Vec<bool>,
    /// Keys that were just released this frame
    pub just_released: Vec<bool>,
}

impl KeyState {
    pub fn new(key_count: u8) -> Self {
        Self {
            pressed: vec![false; key_count as usize],
            just_pressed: vec![false; key_count as usize],
            just_released: vec![false; key_count as usize],
        }
    }
}

fn update_key_state(
    keyboard: Res<ButtonInput<KeyCode>>,
    bindings: Res<KeyBindings>,
    mut state: ResMut<KeyState>,
) {
    // Ensure state vectors are correct size
    let key_count = bindings.keys.len();
    if state.pressed.len() != key_count {
        *state = KeyState::new(key_count as u8);
    }

    for (i, key) in bindings.keys.iter().enumerate() {
        state.pressed[i] = keyboard.pressed(*key);
        state.just_pressed[i] = keyboard.just_pressed(*key);
        state.just_released[i] = keyboard.just_released(*key);
    }
}