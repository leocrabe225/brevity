use bevy::prelude::*;

use crate::{
    GameState,
    game::{
        IntroTimer, SetupSet,
        ball::Ball,
        brick::Brick,
        hud::{Hint, ScoreboardText},
        paddle::Paddle,
    },
};

impl Plugin for IntroPlugin {
    fn build(&self, app: &mut App) {
        app.insert_resource(IntroTimer(Timer::from_seconds(INTRO_END, TimerMode::Once)));
        app.add_systems(OnEnter(GameState::Game), setup.in_set(SetupSet::Stage));
        app.add_systems(Update, reveal);
    }
}

pub(super) struct IntroPlugin;

#[derive(Component)]
struct RevealAt(f32);

const BRICKS_AT: f32 = 1.;
const PADDLE_BALL_AT: f32 = 2.;
const SCORE_AT: f32 = 3.;
const HINTS_AT: f32 = 4.;
const INTRO_END: f32 = 4.;

fn setup(
    mut commands: Commands,
    bricks: Query<Entity, With<Brick>>,
    paddle: Single<Entity, With<Paddle>>,
    balls: Query<Entity, With<Ball>>,
    scoreboard: Single<Entity, With<ScoreboardText>>,
    hints: Query<Entity, With<Hint>>,
) {
    for brick in bricks {
        commands
            .entity(brick)
            .insert((Visibility::Hidden, RevealAt(BRICKS_AT)));
    }
    commands
        .entity(paddle.entity())
        .insert((Visibility::Hidden, RevealAt(PADDLE_BALL_AT)));
    for ball in balls {
        commands
            .entity(ball)
            .insert((Visibility::Hidden, RevealAt(PADDLE_BALL_AT)));
    }
    commands
        .entity(scoreboard.entity())
        .insert((Visibility::Hidden, RevealAt(SCORE_AT)));
    for hint in hints {
        commands
            .entity(hint)
            .insert((Visibility::Hidden, RevealAt(HINTS_AT)));
    }
}

fn reveal(mut commands: Commands, timer: Res<IntroTimer>, query: Query<(Entity, &RevealAt)>) {
    for (entity, reveal_at) in query {
        if timer.elapsed_secs() >= reveal_at.0 {
            commands
                .entity(entity)
                .remove::<RevealAt>()
                .insert(Visibility::Inherited);
        }
    }
}
