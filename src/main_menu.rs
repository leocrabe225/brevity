use super::GameState;
use crate::colors::BreakoutColors;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), setup)
        .add_systems(
            Update,
            (start_game, move_title_animation).run_if(in_state(GameState::MainMenu)),
        );
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(bsn![
        Transform::from_translation(TITLE_START_POSITION)
        Title
        colored_text(
            "BREAKOUT",
            TITLE_FONT_SIZE,
            2,
            &[
                Color::BRICK_YELLOW,
                Color::BRICK_GREEN,
                Color::BRICK_ORANGE,
                Color::BRICK_RED
            ]
        )
    ]);
}

const TITLE_FONT_SIZE: FontSize = FontSize::Px(67.);

const TITLE_START_POSITION: Vec3 = Vec3::new(0., 300., -1.);
const TITLE_END_POSITION: Vec3 = Vec3::new(0., 170., -1.);

#[derive(Component, Clone, Default)]
struct Title;

#[derive(Component)]
struct GameStarting(Timer);

fn colored_text(
    text: &str,
    font_size: FontSize,
    chunk_size: usize,
    colors: &[Color],
) -> impl Scene {
    let chars: Vec<char> = text.chars().collect();
    let spans: Vec<_> = chars
        .chunks(chunk_size)
        .enumerate()
        .map(|(i, pair)| {
            let pair: String = pair.iter().collect();
            let color = colors[i % colors.len()];
            bsn! {
                TextSpan(pair)
                TextColor(color)
                TextFont {font_size}
            }
        })
        .collect();

    bsn! {
        Text2d
        Children [{spans}]
    }
}

fn move_title_animation(
    mut commands: Commands,
    timer: Single<(Entity, &mut GameStarting)>,
    title: Single<&mut Transform, With<Title>>,
    mut next: ResMut<NextState<GameState>>,
    time: Res<Time>,
) {
    let (entity, mut timer) = timer.into_inner();
    timer.0.tick(time.delta());

    let mut transform = title.into_inner();
    let progress: Vec3 = EasingCurve::new(
        TITLE_START_POSITION,
        TITLE_END_POSITION,
        EaseFunction::Linear,
    )
    .sample_clamped(timer.0.fraction());
    transform.translation = progress;

    if timer.0.fraction() >= 1. {
        commands.entity(entity).despawn();
        next.set(GameState::Game);
    }
}

fn start_game(
    mut commands: Commands,
    keys: Res<ButtonInput<KeyCode>>,
    mut last: Local<Option<bool>>,
) {
    if !last.is_some() && keys.just_pressed(KeyCode::Enter) {
        commands.spawn(GameStarting(Timer::from_seconds(2., TimerMode::Once)));
        *last = Some(true);
    }
}
