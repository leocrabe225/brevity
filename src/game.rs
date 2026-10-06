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
use ball::BallLost;
use bevy::prelude::*;

#[derive(SubStates, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
#[source(GameState = GameState::Game)]
enum GamePhase {
    #[default]
    Intro,
    Playing,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum GameSet {
    Input,
    Movement,
    Detection,
    Resolution,
    Rules,
}

#[derive(SystemSet, Debug, Clone, PartialEq, Eq, Hash)]
enum SetupSet {
    Reset,
    Spawn,
    Stage,
}

pub(super) fn plugin(app: &mut App) {
    app.add_sub_state::<GamePhase>()
        .add_message::<BallLost>()
        .add_observer(snap_follower)
        .configure_sets(
            OnEnter(GameState::Game),
            (SetupSet::Reset, SetupSet::Spawn, SetupSet::Stage).chain(),
        )
        .configure_sets(
            FixedUpdate,
            (
                GameSet::Input,
                GameSet::Movement,
                GameSet::Detection,
                GameSet::Resolution,
                GameSet::Rules,
            )
                .chain()
                .run_if(in_state(GamePhase::Playing)),
        )
        .configure_sets(FixedUpdate, CollisionSystems.in_set(GameSet::Detection))
        .add_systems(
            OnEnter(GameState::Game),
            setup_resources.in_set(SetupSet::Reset),
        )
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
                apply_velocity.in_set(GameSet::Movement),
                handle_ball_lost.in_set(GameSet::Rules),
                follow.after(GameSet::Resolution).before(GameSet::Rules),
            ),
        );
}

fn setup_resources(mut commands: Commands) {
    commands.init_resource::<IntroTimer>();
    commands.insert_resource(Score(Decimal2::ZERO));
    commands.insert_resource(Lives(START_LIVES));
}

const BALL_LIVES_COLORS: [Color; 3] = [Color::BRICK_RED, Color::BRICK_ORANGE, Color::WHITE];
const START_LIVES: usize = 3;

#[derive(Resource, Default, Deref, DerefMut)]
struct IntroTimer(Timer);

#[derive(Component, Default)]
struct Solid;

#[derive(Component, Default)]
struct Static;

#[derive(Component, Deref, DerefMut)]
struct Velocity(Vec2);

#[derive(Resource, Deref, DerefMut)]
struct Score(Decimal2);

#[derive(Resource, Deref, DerefMut, Copy, Clone)]
struct Lives(usize);

#[derive(Component)]
struct FollowEntity {
    entity: Entity,
    offset: Vec2,
}

fn advance_intro(
    mut timer: ResMut<IntroTimer>,
    time: Res<Time>,
    mut next: ResMut<NextState<GamePhase>>,
) {
    if timer.0.tick(time.delta()).is_finished() {
        next.set(GamePhase::Playing);
    }
}

fn apply_velocity(mut query: Query<(&mut Transform, &Velocity)>, fixed_time: Res<Time>) {
    for (mut transform, velocity) in &mut query {
        transform.translation.x += velocity.x * fixed_time.delta_secs();
        transform.translation.y += velocity.y * fixed_time.delta_secs();
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

fn handle_ball_lost(
    mut commands: Commands,
    mut ball_lost: MessageReader<BallLost>,
    mut lives: ResMut<Lives>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    paddle: Single<Entity, With<paddle::Paddle>>,
) {
    for _ in ball_lost.read() {
        **lives -= 1;

        if **lives == 0 {
            todo!(); // End game
        }

        ball::spawn_serving_ball(
            paddle.entity(),
            &mut commands,
            &mut meshes,
            &mut materials,
            BALL_LIVES_COLORS[lives.0 - 1],
        );
    }
}
