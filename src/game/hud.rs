use std::f32::consts::PI;

use crate::game::GamePhase;

use super::{Score, progress::Progress};
use bevy::{prelude::*, text::TextSection};

const SCOREBOARD_FONT_SIZE: FontSize = FontSize::Px(200.);
const SPEED_FONT_SIZE: FontSize = FontSize::Px(40.);
const SPACE_HINT_FONT_SIZE: FontSize = FontSize::Px(40.);

const SPEED_UPDATE_POS: Vec2 = Vec2::new(0., -300.);
const SCORE_POS: Vec2 = Vec2::new(0., 0.);
const SPACE_HINT_POS: Vec2 = Vec2::new(0., -155.);

#[derive(Component)]
pub(super) struct ScoreboardText;

#[derive(Component)]
pub(super) struct SpaceHintText;

#[derive(Component)]
pub(super) struct SizeBounce {
    clock: f32,
    font_size: FontSize,
}

#[derive(Component)]
pub(super) struct FadeOut(Timer);

pub(super) fn spawn_scoreboard(commands: &mut Commands) {
    commands.spawn((
        Text2d::new("0"),
        TextFont {
            font_size: SCOREBOARD_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform {
            translation: SCORE_POS.extend(-1.),
            ..default()
        },
        ScoreboardText,
    ));
}

pub(super) fn spawn_space_hint(commands: &mut Commands) {
    commands.spawn((
        Text2d::new("PRESS\nSPACE"),
        TextFont {
            font_size: SPACE_HINT_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform {
            translation: SPACE_HINT_POS.extend(-1.),
            ..default()
        },
        SpaceHintText,
        SizeBounce {
            clock: 0.,
            font_size: SPACE_HINT_FONT_SIZE,
        },
        DespawnOnExit(GamePhase::AwaitingFirstInput),
    ));
}

pub(super) fn bounce_size(time: Res<Time>, mut query: Query<(&mut TextFont, &mut SizeBounce)>) {
    for (mut font, mut size_bounce) in &mut query {
        size_bounce.clock += time.delta_secs();
        let progress = 1. + 0.5 * (PI * size_bounce.clock).sin();
        font.font_size = size_bounce.font_size * progress;
    }
}

pub(super) fn fade_out(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeOut, &mut TextColor)>,
) {
    for (entity, mut fade, mut color) in &mut query {
        fade.0.tick(time.delta());
        let progress = EaseFunction::ExponentialIn.sample_clamped(fade.0.fraction());
        color.0.set_alpha(1. - progress);
        if progress >= 1. {
            commands.entity(entity).despawn();
        }
    }
}

pub(super) fn speed_update_text(
    mut commands: Commands,
    progress: Res<Progress>,
    mut last: Local<Option<f32>>,
) {
    let multiplier = progress.speed_multiplier();
    if last.is_some_and(|last| multiplier > last) {
        commands.spawn((
            Text2d::new(format!("x{:.2}", multiplier)),
            FadeOut(Timer::from_seconds(1.5, TimerMode::Once)),
            TextFont {
                font_size: SPEED_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform {
                translation: SPEED_UPDATE_POS.extend(0.),
                ..default()
            },
        ));
    }
    *last = Some(multiplier);
}

pub(super) fn update_scoreboard(
    score: Res<Score>,
    mut text: Single<&mut Text2d, With<ScoreboardText>>,
) {
    let new_text = text.get_text_mut();

    *new_text = score.to_string();
}
