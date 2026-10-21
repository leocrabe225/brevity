mod arena;
mod ball;
mod brick;
mod hud;
mod intro;
mod paddle;
mod progress;

use super::GameState;
use crate::{
    collision::{self, CollisionSystems},
    colors::BreakoutColors,
    decimal2::Decimal2,
};
use bevy::prelude::*;

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameState = GameState::Game)]
enum GamePhase {
    #[default]
    Intro,
    Playing,
    GameOver,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
    Detection,
    Resolution,
    Rules,
    Consequences,
}

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<GamePhase>()
        .init_resource::<IntroTimer>()
        .init_per_game::<Score>()
        .init_per_game::<Lives>()
        .add_observer(snap_follower)
        .add_observer(serve_on_new_paddle)
        .configure_sets(
            FixedUpdate,
            (
                GameSet::Input,
                GameSet::Movement,
                GameSet::Detection,
                GameSet::Resolution,
                GameSet::Rules,
                GameSet::Consequences,
            )
                .chain()
                .run_if(in_state(GamePhase::Playing)),
        )
        .configure_sets(FixedUpdate, CollisionSystems.in_set(GameSet::Detection))
        .add_systems(Update, advance_intro.run_if(in_state(GamePhase::Intro)))
        .add_plugins((
            intro::IntroPlugin,
            arena::ArenaPlugin,
            ball::BallPlugin,
            brick::BrickPlugin,
            hud::HudPlugin,
            paddle::PaddlePlugin,
            collision::CollisionPlugin,
            progress::ProgressPlugin,
        ))
        .add_systems(
            FixedUpdate,
            (
                (apply_velocity, apply_angular_velocity).in_set(GameSet::Movement),
                follow.after(GameSet::Resolution).before(GameSet::Rules),
                handle_ball_died.in_set(GameSet::Consequences),
            ),
        )
        .add_systems(
            FixedPostUpdate,
            game_over.run_if(in_state(GamePhase::GameOver)),
        );
}

const BALL_LIVES_COLORS: [Color; 3] = [Color::BRICK_RED, Color::BRICK_ORANGE, Color::WHITE];

#[derive(Component, Default, Clone)]
#[require(DespawnOnExit<GamePhase> = DespawnOnExit(GamePhase::GameOver))]
struct OnBoard;

#[derive(Resource, Default, Deref, DerefMut)]
struct IntroTimer(Timer);

#[derive(Component, Default)]
struct Solid;

#[derive(Component, Default)]
struct Static;

#[derive(Component, Deref, DerefMut, Clone, Default)]
struct Velocity(Vec2);

#[derive(Component, Deref, DerefMut)]
struct AngularVelocity(f32);

#[derive(Resource, Deref, DerefMut)]
struct Score(Decimal2);
impl PerGame for Score {
    const START: Self = Score(Decimal2::ZERO);
}

#[derive(Resource, Deref, DerefMut, Copy, Clone)]
struct Lives(usize);
impl PerGame for Lives {
    const START: Self = Lives(3);
}

#[derive(Component, Clone, Copy, FromTemplate)]
struct FollowEntity {
    entity: Entity,
    offset: Vec2,
}

trait PerGame: Resource {
    const START: Self;
}

fn reset<T: PerGame>(mut commands: Commands) {
    commands.insert_resource(T::START);
}

trait PerGameExt {
    fn init_per_game<T: PerGame>(&mut self) -> &mut Self;
}

impl PerGameExt for App {
    fn init_per_game<T: PerGame>(&mut self) -> &mut Self {
        self.insert_resource(T::START)
            .add_systems(OnExit(GameState::Game), reset::<T>)
    }
}

fn advance_intro(
    mut timer: ResMut<IntroTimer>,
    time: Res<Time>,
    mut next: ResMut<NextState<GamePhase>>,
) {
    if timer.0.tick(time.delta()).just_finished() {
        next.set(GamePhase::Playing);
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * time.delta_secs();
        transform.translation.y += velocity.y * time.delta_secs();
    }
}

fn apply_angular_velocity(mut query: Query<(&mut Transform, &AngularVelocity)>, time: Res<Time>) {
    for (mut transform, angular_velocity) in &mut query {
        transform.rotation *= Quat::from_rotation_z(**angular_velocity * time.delta_secs());
    }
}

fn snap_follower(
    insert: On<Insert, FollowEntity>,
    followers: Query<&FollowEntity>,
    mut transforms: Query<&mut Transform>,
) {
    let Ok(follow_entity) = followers.get(insert.entity) else {
        return;
    };
    let Ok(target) = transforms
        .get(follow_entity.entity)
        .map(|transform| transform.translation)
    else {
        return;
    };
    let Ok(mut follower) = transforms.get_mut(insert.entity) else {
        return;
    };
    apply_follow(&mut follower, follow_entity.offset, target.xy());
}

fn follow(followers: Query<(Entity, &FollowEntity)>, mut transforms: Query<&mut Transform>) {
    for (entity, follow_entity) in followers {
        let Ok(target) = transforms
            .get(follow_entity.entity)
            .map(|transform| transform.translation)
        else {
            continue;
        };
        let Ok(mut follower) = transforms.get_mut(entity) else {
            continue;
        };
        apply_follow(&mut follower, follow_entity.offset, target.xy());
    }
}

fn apply_follow(follower: &mut Transform, offset: Vec2, target: Vec2) {
    follower.translation = (target + offset).extend(follower.translation.z);
}

fn handle_ball_died(
    mut commands: Commands,
    mut events: MessageReader<ball::BallDied>,
    mut lives: ResMut<Lives>,
    paddle: Single<Entity, With<paddle::Paddle>>,
    mut next: ResMut<NextState<GamePhase>>,
) {
    for _ in events.read() {
        **lives -= 1;

        if **lives == 0 {
            next.set(GamePhase::GameOver);
            return;
        }

        serve_next_ball(paddle.entity(), &mut commands, *lives);
    }
}

fn game_over(mut next: ResMut<NextState<GameState>>) {
    next.set(GameState::MainMenu);
}

fn serve_on_new_paddle(add: On<Add, paddle::Paddle>, mut commands: Commands, lives: Res<Lives>) {
    serve_next_ball(add.entity, &mut commands, *lives);
}

fn serve_next_ball(entity: Entity, commands: &mut Commands, lives: Lives) {
    ball::spawn_serving_ball(entity, commands, BALL_LIVES_COLORS[lives.0 - 1]);
}
