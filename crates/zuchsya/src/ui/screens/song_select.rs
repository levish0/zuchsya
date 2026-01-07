//! Song selection screen

use bevy::prelude::*;
use std::path::PathBuf;
use zuchsya_core::{GameState, ZuchsyaMap};
use zuchsya_play::note::CurrentHitObjects;

pub struct SongSelectPlugin;

impl Plugin for SongSelectPlugin {
    fn build(&self, app: &mut App) {
        app.init_resource::<BeatmapList>()
            .init_resource::<SelectedBeatmap>()
            .add_systems(OnEnter(GameState::SongSelect), (scan_beatmaps, setup_song_select).chain())
            .add_systems(Update, handle_input.run_if(in_state(GameState::SongSelect)))
            .add_systems(OnExit(GameState::SongSelect), cleanup_song_select);
    }
}

/// List of available beatmaps
#[derive(Resource, Default)]
pub struct BeatmapList {
    pub maps: Vec<BeatmapEntry>,
}

/// Entry in the beatmap list
#[derive(Clone)]
pub struct BeatmapEntry {
    pub path: PathBuf,
    pub title: String,
    pub artist: String,
    pub difficulty: String,
}

/// Currently selected beatmap index
#[derive(Resource, Default)]
pub struct SelectedBeatmap {
    pub index: usize,
}

#[derive(Component)]
struct SongSelectScreen;

#[derive(Component)]
struct BeatmapListItem(usize);

/// Scan for .zuchsya files in the beatmaps folder
fn scan_beatmaps(mut beatmap_list: ResMut<BeatmapList>, mut selected: ResMut<SelectedBeatmap>) {
    beatmap_list.maps.clear();
    selected.index = 0;

    // Try multiple possible locations
    let possible_paths = [
        PathBuf::from("beatmaps"),
        PathBuf::from("./beatmaps"),
        std::env::current_exe()
            .ok()
            .and_then(|p| p.parent().map(|p| p.join("beatmaps")))
            .unwrap_or_default(),
    ];

    for base_path in &possible_paths {
        if let Ok(entries) = std::fs::read_dir(base_path) {
            for entry in entries.flatten() {
                let path = entry.path();
                if path.extension().is_some_and(|ext| ext == "zuchsya") {
                    if let Ok(map) = ZuchsyaMap::load(&path) {
                        beatmap_list.maps.push(BeatmapEntry {
                            path: path.clone(),
                            title: map.metadata.title,
                            artist: map.metadata.artist,
                            difficulty: map.metadata.difficulty_name,
                        });
                    }
                }
            }
        }
    }

    // Sort by title
    beatmap_list.maps.sort_by(|a, b| a.title.cmp(&b.title));
}

fn setup_song_select(
    mut commands: Commands,
    beatmap_list: Res<BeatmapList>,
    selected: Res<SelectedBeatmap>,
) {
    commands
        .spawn((
            SongSelectScreen,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                justify_content: JustifyContent::Center,
                align_items: AlignItems::Center,
                flex_direction: FlexDirection::Column,
                ..default()
            },
            BackgroundColor(Color::srgb(0.08, 0.08, 0.12)),
        ))
        .with_children(|parent| {
            parent.spawn((
                Text::new("Song Select"),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::WHITE),
            ));

            // Beatmap list container
            parent
                .spawn((
                    Node {
                        flex_direction: FlexDirection::Column,
                        margin: UiRect::top(Val::Px(30.0)),
                        padding: UiRect::all(Val::Px(20.0)),
                        min_width: Val::Px(400.0),
                        max_height: Val::Px(400.0),
                        overflow: Overflow::clip(),
                        ..default()
                    },
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.3)),
                ))
                .with_children(|list| {
                    if beatmap_list.maps.is_empty() {
                        list.spawn((
                            Text::new("No beatmaps found in ./beatmaps/"),
                            TextFont {
                                font_size: 20.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.6, 0.6, 0.6)),
                        ));
                        list.spawn((
                            Text::new("Create .zuchsya files to play!"),
                            TextFont {
                                font_size: 16.0,
                                ..default()
                            },
                            TextColor(Color::srgb(0.5, 0.5, 0.5)),
                            Node {
                                margin: UiRect::top(Val::Px(10.0)),
                                ..default()
                            },
                        ));
                    } else {
                        for (i, entry) in beatmap_list.maps.iter().enumerate() {
                            let is_selected = i == selected.index;
                            let bg_color = if is_selected {
                                Color::srgba(0.3, 0.5, 0.8, 0.5)
                            } else {
                                Color::srgba(0.0, 0.0, 0.0, 0.0)
                            };

                            list.spawn((
                                BeatmapListItem(i),
                                Node {
                                    padding: UiRect::all(Val::Px(10.0)),
                                    margin: UiRect::bottom(Val::Px(5.0)),
                                    ..default()
                                },
                                BackgroundColor(bg_color),
                            ))
                            .with_children(|item| {
                                item.spawn((
                                    Text::new(format!(
                                        "{} - {} [{}]",
                                        entry.artist, entry.title, entry.difficulty
                                    )),
                                    TextFont {
                                        font_size: 18.0,
                                        ..default()
                                    },
                                    TextColor(if is_selected {
                                        Color::WHITE
                                    } else {
                                        Color::srgb(0.8, 0.8, 0.8)
                                    }),
                                ));
                            });
                        }
                    }
                });

            // Instructions
            parent.spawn((
                Text::new("UP/DOWN: Select | ENTER: Play | ESC: Back"),
                TextFont {
                    font_size: 18.0,
                    ..default()
                },
                TextColor(Color::srgb(0.5, 0.5, 0.5)),
                Node {
                    margin: UiRect::top(Val::Px(20.0)),
                    ..default()
                },
            ));
        });
}

fn handle_input(
    keyboard: Res<ButtonInput<KeyCode>>,
    beatmap_list: Res<BeatmapList>,
    mut selected: ResMut<SelectedBeatmap>,
    mut next_state: ResMut<NextState<GameState>>,
    mut commands: Commands,
    mut items: Query<(&BeatmapListItem, &mut BackgroundColor, &Children)>,
    mut texts: Query<&mut TextColor>,
) {
    let map_count = beatmap_list.maps.len();

    if map_count > 0 {
        let mut selection_changed = false;

        if keyboard.just_pressed(KeyCode::ArrowUp) {
            if selected.index > 0 {
                selected.index -= 1;
            } else {
                selected.index = map_count - 1;
            }
            selection_changed = true;
        }

        if keyboard.just_pressed(KeyCode::ArrowDown) {
            if selected.index < map_count - 1 {
                selected.index += 1;
            } else {
                selected.index = 0;
            }
            selection_changed = true;
        }

        // Update visual selection
        if selection_changed {
            for (item, mut bg, children) in items.iter_mut() {
                let is_selected = item.0 == selected.index;
                *bg = if is_selected {
                    BackgroundColor(Color::srgba(0.3, 0.5, 0.8, 0.5))
                } else {
                    BackgroundColor(Color::srgba(0.0, 0.0, 0.0, 0.0))
                };

                for child in children.iter() {
                    if let Ok(mut text_color) = texts.get_mut(child) {
                        *text_color = if is_selected {
                            TextColor(Color::WHITE)
                        } else {
                            TextColor(Color::srgb(0.8, 0.8, 0.8))
                        };
                    }
                }
            }
        }

        if keyboard.just_pressed(KeyCode::Enter) {
            if let Some(entry) = beatmap_list.maps.get(selected.index) {
                // Load the selected beatmap
                if let Ok(map) = ZuchsyaMap::load(&entry.path) {
                    commands.insert_resource(CurrentHitObjects {
                        objects: map.hit_objects,
                    });
                    next_state.set(GameState::Playing);
                }
            }
        }
    }

    if keyboard.just_pressed(KeyCode::Escape) {
        next_state.set(GameState::MainMenu);
    }
}

fn cleanup_song_select(mut commands: Commands, query: Query<Entity, With<SongSelectScreen>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
