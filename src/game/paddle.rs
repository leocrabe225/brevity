use super::{Solid, Static, Velocity};
use crate::{
    GameState,
    collision::{Collider, Collision, Shape},
    game::{GameSet, SetupSet},
};
use bevy::prelude::*;

impl Plugin for PaddlePlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn.in_set(SetupSet::Spawn));
        app.add_systems(
            FixedUpdate,
            (
                movement.in_set(GameSet::Input),
                bounce.in_set(GameSet::Resolution),
            ),
        );
    }
}

const PADDLE_COLOR: Color = Color::WHITE;
const PADDLE_SIZE: Vec2 = Vec2::new(100., 10.);
const PADDLE_START_POS: Vec2 = Vec2::new(0., -250.);

pub(super) struct PaddlePlugin;

#[derive(Component)]
#[require(Solid)]
pub(super) struct Paddle;

fn spawn(mut commands: Commands) {
    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: PADDLE_START_POS.extend(0.0),
            scale: PADDLE_SIZE.extend(1.0),
            ..default()
        },
        Velocity(Vec2::ZERO),
        Paddle,
        Solid,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(PADDLE_SIZE.x, PADDLE_SIZE.y)),
        },
    ));
}

fn movement(
    keyboard_input: Res<ButtonInput<KeyCode>>,
    paddle: Single<&mut Velocity, With<Paddle>>,
) {
    const SPEED: f32 = 500.0;

    let mut velocity = paddle.into_inner();
    *velocity = Velocity(Vec2::ZERO);

    if keyboard_input.pressed(KeyCode::ArrowLeft) {
        velocity.x -= SPEED;
    }

    if keyboard_input.pressed(KeyCode::ArrowRight) {
        velocity.x += SPEED;
    }
}

fn bounce(
    mut collisions: MessageReader<Collision>,
    mut paddles: Query<&mut Transform, With<Paddle>>,
    solids: Query<(), With<Solid>>,
    walls: Query<(), With<Static>>,
) {
    for collision in collisions.read() {
        let (me, other) = (&collision.collisioner, &collision.collisionee);
        if !paddles.contains(me.entity) || !solids.contains(other.entity) {
            continue;
        }
        if !walls.contains(collision.collisionee.entity) {
            continue;
        }
        let Ok(mut transform) = paddles.get_mut(collision.collisioner.entity) else {
            continue;
        };
        let push = collision.contact.normal * collision.contact.penetration;
        transform.translation += push.extend(0.0);
    }
}
