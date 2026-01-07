//! UI systems and screens

use bevy::prelude::*;

pub mod screens;

use screens::{loading, main_menu, song_select};

/// UI plugin
pub struct UiPlugin;

impl Plugin for UiPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins((
            loading::LoadingPlugin,
            main_menu::MainMenuPlugin,
            song_select::SongSelectPlugin,
        ));
    }
}
