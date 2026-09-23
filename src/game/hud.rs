use super::{Score, progress::Progress};
use bevy::{prelude::*, text::TextSection};

const SCOREBOARD_FONT_SIZE: FontSize = FontSize::Px(200.);
const SPEED_FONT_SIZE: FontSize = FontSize::Px(40.);

#[derive(Component)]
pub(super) struct ScoreboardText;

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
            translation: Vec3::new(0., 0., 0.),
            ..default()
        },
        ScoreboardText,
    ));
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
                translation: Vec3::new(0., -300., 0.),
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
