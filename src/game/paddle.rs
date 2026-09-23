use super::{Solid, Velocity, wall::Wall};
use crate::collision::{Collider, Collision, Shape};
use bevy::prelude::*;

const PADDLE_COLOR: Color = Color::WHITE;
const PADDLE_SIZE: Vec2 = Vec2::new(100., 10.);

#[derive(Component)]
#[require(Solid)]
pub(super) struct Paddle;

pub(super) fn spawn(commands: &mut Commands) {
    commands.spawn((
        Sprite::from_color(PADDLE_COLOR, Vec2::ONE),
        Transform {
            translation: Vec3::new(0.0, -250.0, 0.0),
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

pub(super) fn movement(
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

pub(super) fn bounce(
    mut collisions: MessageReader<Collision>,
    mut paddles: Query<&mut Transform, With<Paddle>>,
    solids: Query<(), With<Solid>>,
    walls: Query<(), With<Wall>>,
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
