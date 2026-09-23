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

pub(super) fn plugin(app: &mut App) {
    app.add_message::<Collision>()
        .add_message::<BallLost>()
        .add_systems(OnEnter(GameState::Game), (setup_resources, setup).chain())
        .add_systems(
            Update,
            (ball::serve, hud::fade_out, hud::speed_update_text).run_if(in_state(GameState::Game)),
        )
        .add_systems(
            FixedUpdate,
            (
                paddle::movement,
                apply_velocity,
                ball::serving_ball_follow_paddle,
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
                .run_if(in_state(GameState::Game)),
        );
}

fn setup_resources(mut commands: Commands) {
    commands.insert_resource(Score(0));
    commands.insert_resource(Lives(START_LIVES));
    commands.insert_resource(Progress { ..default() });
}

fn setup(
    mut commands: Commands,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    lives: Res<Lives>,
    window: Single<&Window>,
) {
    paddle::spawn(&mut commands);
    ball::spawn_serving_ball(
        &mut commands,
        &mut meshes,
        &mut materials,
        BALL_LIVES_COLORS[lives.0 - 1],
    );
    brick::spawn(&mut commands, &window);
    wall::spawn(&mut commands, &window);
    hud::spawn_scoreboard(&mut commands);
}

const BALL_LIVES_COLORS: [Color; 3] = [Color::BRICK_RED, Color::BRICK_ORANGE, Color::WHITE];
const START_LIVES: usize = 3;

#[derive(Component, Default)]
struct Solid;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource, Deref, DerefMut)]
struct Score(usize);

#[derive(Resource, Deref, DerefMut, Copy, Clone)]
struct Lives(usize);

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
