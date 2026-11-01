use super::GameState;
use crate::{colors::BreakoutColors, sequence::Sequence};
use bevy::prelude::*;

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameState = GameState::MainMenu)]
enum MenuPhase {
    #[default]
    Waiting,
    Outro,
}

impl Sequence for MenuPhase {
    fn next(self) -> Option<Self> {
        match self {
            MenuPhase::Waiting => Some(MenuPhase::Outro),
            MenuPhase::Outro => None,
        }
    }

    fn finish(world: &mut World) {
        todo!(
            "Implement finish and find where the fuck to place Hold (Title means remove_component, not a big fan)",
        )
    }
}

impl Plugin for MainMenuPlugin {
    fn build(&self, app: &mut App) {
        app.add_sub_state::<MenuPhase>()
            .add_systems(OnEnter(GameState::MainMenu), setup)
            .add_systems(
                Update,
                (
                    start_game.run_if(in_state(MenuPhase::Waiting)),
                    move_title_animation.run_if(in_state(MenuPhase::Outro)),
                ),
            );
    }
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(bsn![
        Transform::from_translation(TITLE_START_POSITION)
        Title
        DespawnOnEnter::<GameState>(GameState::MainMenu)
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

pub(super) struct MainMenuPlugin;

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
    mut next: ResMut<NextState<MenuPhase>>,
) {
    if keys.just_pressed(KeyCode::Enter) {
        commands.spawn(GameStarting(Timer::from_seconds(2., TimerMode::Once)));
        next.set(MenuPhase::Outro);
    }
}
