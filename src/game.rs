mod ball;
mod brick;
mod hud;
mod paddle;
mod progress;
mod wall;

use super::GameState;
use crate::{
    collision::{self, Collision},
    colors::BreakoutColors,
};
use ball::BallLost;
use bevy::prelude::*;
use progress::Progress;

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameState = GameState::Game)]
enum GamePhase {
    #[default]
    Intro,
    AwaitingFirstInput,
    Playing,
}

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<GamePhase>()
        .add_message::<Collision>()
        .add_message::<BallLost>()
        .add_systems(OnEnter(GameState::Game), setup_resources)
        .add_systems(
            Update,
            (run_intro, ball::serving_ball_follow_paddle)
                .chain()
                .run_if(in_state(GamePhase::Intro)),
        )
        .add_systems(
            Update,
            (hud::bounce_size, start_game, ball::serve)
                .chain()
                .run_if(in_state(GamePhase::AwaitingFirstInput)),
        )
        .add_systems(
            Update,
            (
                ball::serve,
                ball::serving_ball_follow_paddle,
                hud::fade_out,
                hud::speed_update_text,
            )
                .run_if(in_state(GamePhase::Playing)),
        )
        .add_systems(
            FixedUpdate,
            (
                paddle::movement,
                apply_velocity,
                collision::check_for_collisions,
                paddle::bounce,
                ball::bounce,
                brick::hit_by_ball,
                ball::hit_by_killzone,
                ball::hit_by_backboard,
                ball::speed_update,
                handle_ball_lost,
                hud::update_scoreboard,
            )
                .chain()
                .run_if(in_state(GamePhase::Playing)),
        );
}

fn setup_resources(mut commands: Commands) {
    commands.insert_resource(IntroClock(0.));
    commands.insert_resource(Score(0));
    commands.insert_resource(Lives(START_LIVES));
    commands.insert_resource(Progress { ..default() });
}

fn run_intro(
    mut commands: Commands,
    time: Res<Time>,
    mut clock: ResMut<IntroClock>,
    window: Single<&Window>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    lives: Res<Lives>,
    mut next: ResMut<NextState<GamePhase>>,
) {
    let before = clock.0;

    clock.0 += time.delta_secs();

    if before <= WALLS_AT && clock.0 > WALLS_AT {
        wall::spawn(&mut commands, &window);
    }
    if before <= BRICKS_AT && clock.0 > BRICKS_AT {
        brick::spawn(&mut commands, &window);
    }
    if before <= PADDLE_BALL_AT && clock.0 > PADDLE_BALL_AT {
        paddle::spawn(&mut commands);
        ball::spawn_serving_ball(
            &mut commands,
            &mut meshes,
            &mut materials,
            BALL_LIVES_COLORS[lives.0 - 1],
        );
    }
    if before <= SCORE_AT && clock.0 > SCORE_AT {
        hud::spawn_scoreboard(&mut commands);
    }
    if before <= SPACE_HINT_AT && clock.0 > SPACE_HINT_AT {
        hud::spawn_space_hint(&mut commands);
    }
    if clock.0 > INTRO_END {
        next.set(GamePhase::AwaitingFirstInput);
    }
}

const WALLS_AT: f32 = 0.;
const BRICKS_AT: f32 = 1.;
const PADDLE_BALL_AT: f32 = 2.;
const SCORE_AT: f32 = 3.;
const SPACE_HINT_AT: f32 = 4.;
const INTRO_END: f32 = 4.;
const BALL_LIVES_COLORS: [Color; 3] = [Color::BRICK_RED, Color::BRICK_ORANGE, Color::WHITE];
const START_LIVES: usize = 3;

#[derive(Resource)]
struct IntroClock(f32);

#[derive(Component, Default)]
struct Solid;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Resource, Deref, DerefMut, Copy, Clone)]
struct Lives(usize);

fn start_game(keyboard_input: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GamePhase>>) {
    if !keyboard_input.just_pressed(KeyCode::Space) {
        return;
    }
    next.set(GamePhase::Playing);
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, fixed_time: Res<Time<Fixed>>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * fixed_time.delta_secs();
        transform.translation.y += velocity.y * fixed_time.delta_secs();
    }
}

fn handle_ball_lost(
    mut commands: Commands,
    mut ball_lost: MessageReader<BallLost>,
    mut lives: ResMut<Lives>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for _ in ball_lost.read() {
        **lives -= 1;

        if **lives == 0 {
            panic!(); // End game
        }

        ball::spawn_serving_ball(
            &mut commands,
            &mut meshes,
            &mut materials,
            BALL_LIVES_COLORS[lives.0 - 1],
        );
    }
}
