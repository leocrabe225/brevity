use super::{Score, Solid, ball::Ball, progress::Progress};
use crate::GameState;
use crate::collision::{Collider, Collision, Shape};
use crate::colors::BreakoutColors;
use crate::decimal2::Decimal2;
use crate::game::{GameSet, SetupSet};
use bevy::prelude::*;

impl Plugin for BrickPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<BrickDestroyed>();
        app.add_systems(OnEnter(GameState::Game), spawn.in_set(SetupSet::Spawn));
        app.add_systems(FixedUpdate, hit_by_ball.in_set(GameSet::Resolution));
    }
}

pub(super) struct BrickPlugin;

const BRICK_ROW_COLORS: [BrickColor; 8] = [
    BrickColor::Red,
    BrickColor::Red,
    BrickColor::Orange,
    BrickColor::Orange,
    BrickColor::Green,
    BrickColor::Green,
    BrickColor::Yellow,
    BrickColor::Yellow,
];
const BRICK_SIZE: Vec2 = Vec2::new(50., 10.);
const BRICK_PADDING: Vec2 = Vec2::splat(2.);
const BRICK_ROWS: u32 = 8;

#[derive(Clone, Copy, PartialEq)]
enum BrickColor {
    Yellow,
    Green,
    Orange,
    Red,
}

#[derive(Component)]
#[require(Solid)]
pub(super) struct Brick {
    color: BrickColor,
}

impl BrickColor {
    const fn color(self) -> Color {
        match self {
            BrickColor::Yellow => Color::BRICK_YELLOW,
            BrickColor::Green => Color::BRICK_GREEN,
            BrickColor::Orange => Color::BRICK_ORANGE,
            BrickColor::Red => Color::BRICK_RED,
        }
    }

    const fn points(self) -> u32 {
        match self {
            BrickColor::Yellow => 1,
            BrickColor::Green => 3,
            BrickColor::Orange => 5,
            BrickColor::Red => 7,
        }
    }
}

#[derive(Message)]
pub(super) struct BrickDestroyed {
    pub(super) color: Color,
    pub(super) points: Decimal2,
    pub(super) position: Vec2,
}

fn spawn(mut commands: Commands, window: Single<&Window>) {
    let top = window.size().y / 2.;
    let left = -window.size().x / 2.;
    let brick_columns_count =
        ((window.size().x - BRICK_PADDING.x) / (BRICK_SIZE.x + BRICK_PADDING.x)) as u32;
    let brick_row_width =
        brick_columns_count as f32 * (BRICK_SIZE.x + BRICK_PADDING.x) - BRICK_PADDING.x;
    let row_start = (window.size().x - brick_row_width) / 2.;

    for row in 0..BRICK_ROWS {
        let y = top
            - (BRICK_PADDING.y + row as f32 * (BRICK_SIZE.y + BRICK_PADDING.y))
            - BRICK_SIZE.y / 2.;
        let brick_color = BRICK_ROW_COLORS[row as usize];
        for column in 0..brick_columns_count {
            let x = left
                + (row_start + column as f32 * (BRICK_PADDING.x + BRICK_SIZE.x))
                + BRICK_SIZE.x / 2.;
            commands.spawn((
                Sprite::from_color(brick_color.color(), Vec2::ONE),
                Transform {
                    translation: Vec3::new(x, y, 0.),
                    scale: BRICK_SIZE.extend(1.),
                    ..default()
                },
                Brick { color: brick_color },
                Solid,
                Collider {
                    shape: Shape::Rectangle(Rectangle::new(BRICK_SIZE.x, BRICK_SIZE.y)),
                },
            ));
        }
    }
}

fn hit_by_ball(
    mut commands: Commands,
    mut collisions: MessageReader<Collision>,
    mut bricks_destroyed: MessageWriter<BrickDestroyed>,
    bricks: Query<&Brick>,
    balls: Query<(), With<Ball>>,
    mut score: ResMut<Score>,
    mut progress: ResMut<Progress>,
) {
    for collision in collisions.read() {
        let (me, other) = (&collision.collisioner, &collision.collisionee);
        if !bricks.contains(me.entity) || !balls.contains(other.entity) {
            continue;
        }
        let Ok(brick) = bricks.get(me.entity) else {
            continue;
        };
        let points = progress.speed_multiplier() * brick.color.points();

        bricks_destroyed.write(BrickDestroyed {
            color: brick.color.color(),
            points,
            position: me.position,
        });

        **score += points;

        progress.bricks_hit += 1;
        if brick.color == BrickColor::Orange {
            progress.orange_reached = true;
        }
        if brick.color == BrickColor::Red {
            progress.red_reached = true;
        }

        commands.entity(collision.collisioner.entity).despawn();
    }
}
