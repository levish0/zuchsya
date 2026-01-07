//! HUD - Score, Combo, Accuracy display

use bevy::prelude::*;
use zuchsya_core::GameState;

use crate::judgement::{JudgementEvent, ScoreState};

pub struct HudPlugin;

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Playing), setup_hud)
            .add_systems(
                Update,
                (update_score_display, update_combo_display, show_judgement_text)
                    .run_if(in_state(GameState::Playing)),
            )
            .add_systems(OnExit(GameState::Playing), cleanup_hud);
    }
}

#[derive(Component)]
struct HudRoot;

#[derive(Component)]
struct ScoreText;

#[derive(Component)]
struct ComboText;

#[derive(Component)]
struct AccuracyText;

#[derive(Component)]
struct JudgementText {
    timer: f32,
}

fn setup_hud(mut commands: Commands) {
    // HUD Root container
    commands
        .spawn((
            HudRoot,
            Node {
                width: Val::Percent(100.0),
                height: Val::Percent(100.0),
                position_type: PositionType::Absolute,
                ..default()
            },
        ))
        .with_children(|parent| {
            // Score (top right)
            parent.spawn((
                ScoreText,
                Text::new("0"),
                TextFont {
                    font_size: 36.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    top: Val::Px(20.0),
                    ..default()
                },
            ));

            // Accuracy (top right, below score)
            parent.spawn((
                AccuracyText,
                Text::new("100.00%"),
                TextFont {
                    font_size: 24.0,
                    ..default()
                },
                TextColor(Color::srgb(0.8, 0.8, 0.8)),
                Node {
                    position_type: PositionType::Absolute,
                    right: Val::Px(20.0),
                    top: Val::Px(60.0),
                    ..default()
                },
            ));

            // Combo (center bottom)
            parent.spawn((
                ComboText,
                Text::new(""),
                TextFont {
                    font_size: 48.0,
                    ..default()
                },
                TextColor(Color::srgb(1.0, 1.0, 0.5)),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    bottom: Val::Px(150.0),
                    ..default()
                },
            ));

            // Judgement text (center)
            parent.spawn((
                JudgementText { timer: 0.0 },
                Text::new(""),
                TextFont {
                    font_size: 32.0,
                    ..default()
                },
                TextColor(Color::WHITE),
                Node {
                    position_type: PositionType::Absolute,
                    left: Val::Percent(50.0),
                    top: Val::Percent(40.0),
                    ..default()
                },
            ));
        });
}

fn update_score_display(score: Res<ScoreState>, mut query: Query<&mut Text, With<ScoreText>>) {
    if !score.is_changed() {
        return;
    }

    for mut text in query.iter_mut() {
        // osu!mania max score is 1,000,000 (7 digits)
        **text = format!("{:07}", score.score());
    }
}

fn update_combo_display(
    score: Res<ScoreState>,
    mut combo_query: Query<(&mut Text, &mut TextColor), (With<ComboText>, Without<AccuracyText>)>,
    mut acc_query: Query<&mut Text, (With<AccuracyText>, Without<ComboText>)>,
) {
    if !score.is_changed() {
        return;
    }

    // Update combo
    for (mut text, mut color) in combo_query.iter_mut() {
        if score.combo > 0 {
            **text = format!("{}x", score.combo);
            // Color based on combo
            let intensity = (score.combo as f32 / 50.0).min(1.0);
            *color = TextColor(Color::srgb(1.0, 1.0 - intensity * 0.5, 0.5 - intensity * 0.5));
        } else {
            **text = String::new();
        }
    }

    // Update accuracy
    for mut text in acc_query.iter_mut() {
        if score.total_notes() > 0 {
            **text = format!("{:.2}%", score.accuracy * 100.0);
        }
    }
}

fn show_judgement_text(
    mut events: MessageReader<JudgementEvent>,
    mut query: Query<(&mut Text, &mut TextColor, &mut JudgementText)>,
    time: Res<Time>,
) {
    // Handle new judgements
    for event in events.read() {
        for (mut text, mut color, mut judgement) in query.iter_mut() {
            let (label, col) = match event.result {
                zuchsya_core::HitResult::Perfect => ("PERFECT", Color::srgb(0.5, 1.0, 1.0)),
                zuchsya_core::HitResult::Great => ("GREAT", Color::srgb(1.0, 1.0, 0.5)),
                zuchsya_core::HitResult::Good => ("GOOD", Color::srgb(0.5, 1.0, 0.5)),
                zuchsya_core::HitResult::Ok => ("OK", Color::srgb(0.5, 0.8, 0.5)),
                zuchsya_core::HitResult::Meh => ("MEH", Color::srgb(0.6, 0.6, 0.6)),
                zuchsya_core::HitResult::Miss => ("MISS", Color::srgb(1.0, 0.3, 0.3)),
            };
            **text = label.to_string();
            *color = TextColor(col);
            judgement.timer = 0.5; // Show for 0.5 seconds
        }
    }

    // Fade out judgement text
    for (mut text, mut color, mut judgement) in query.iter_mut() {
        if judgement.timer > 0.0 {
            judgement.timer -= time.delta_secs();
            let alpha = (judgement.timer / 0.5).clamp(0.0, 1.0);
            if let Color::Srgba(ref mut srgba) = color.0 {
                srgba.alpha = alpha;
            }
            if judgement.timer <= 0.0 {
                **text = String::new();
            }
        }
    }
}

fn cleanup_hud(mut commands: Commands, query: Query<Entity, With<HudRoot>>) {
    for entity in query.iter() {
        commands.entity(entity).despawn();
    }
}
