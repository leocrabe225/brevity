mod death;

use super::{
    Solid, Velocity,
    arena::{Backboard, OutOfBounds},
    paddle::Paddle,
    progress::Progress,
};
use crate::{
    collision::{Collider, Collision, Shape},
    ecs_ext::InsertNewExt,
    game::{FollowEntity, GameSet, OnBoard},
};
use bevy::prelude::*;
use std::f32::consts::FRAC_PI_3;

impl Plugin for BallPlugin {
    fn build(&self, app: &mut App) {
        app.add_plugins(death::BallDeathPlugin);
        app.add_message::<BallDied>();
        app.add_systems(
            FixedUpdate,
            (
                serve.in_set(GameSet::Input),
                (bounce, out_of_bounds, hit_by_backboard).in_set(GameSet::Resolution),
                speed_update.in_set(GameSet::Rules),
            ),
        );
    }
}

const BALL_DIAMETER: f32 = 10.;
const BALL_RADIUS: f32 = BALL_DIAMETER / 2.;
const INITIAL_BALL_DIRECTION: Vec2 = Vec2::new(0.5, 0.5);
const BALL_STARTING_SPEED: f32 = 400.;
const MAX_PADDLE_BOUNCE_ANGLE: f32 = FRAC_PI_3;
const BALL_Z_ORDER: f32 = 3.;

pub(super) struct BallPlugin;

#[derive(Component, Clone, Copy, Default)]
#[require(OnBoard)]
pub(super) struct Ball;

#[derive(Component, Clone, Copy, Default)]
struct Serving;

#[derive(Message)]
pub(super) struct BallDied {
    color: Color,
    position: Vec2,
    velocity: Vec2,
}

fn ball_visuals(color: Color) -> impl Scene {
    bsn! {
        Mesh2d(asset_value(Circle::new(BALL_RADIUS)))
        MeshMaterial2d::<ColorMaterial>(asset_value(ColorMaterial {
            color,
            alpha_mode: bevy::sprite_render::AlphaMode2d::Blend,
            ..default()
        }))
    }
}

pub(super) fn spawn_serving_ball(follows: Entity, commands: &mut Commands, color: Color) {
    commands.spawn_scene(bsn! {
        ball_visuals(color)
        Transform::from_translation(Vec2::ZERO.extend(BALL_Z_ORDER))
        Serving
        Ball
        FollowEntity {
            entity: follows,
            offset: {Vec2::Y * 7.},
        }
    });
}

fn serve(
    mut commands: Commands,
    progress: Res<Progress>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
    serving_balls: Query<Entity, With<Serving>>,
) {
    if !keyboard_input.pressed(KeyCode::Space) {
        return;
    }
    for serving_ball in serving_balls {
        commands
            .entity(serving_ball)
            .remove::<Serving>()
            .remove::<FollowEntity>()
            .insert_new(Solid)
            .insert_new(Velocity(
                INITIAL_BALL_DIRECTION.normalize() * current_speed(&progress),
            ))
            .insert_new(Collider {
                shape: Shape::Circle(Circle::new(BALL_RADIUS)),
            });
    }
}

fn bounce(
    mut collisions: MessageReader<Collision>,
    mut balls: Query<(&mut Transform, &mut Velocity), With<Ball>>,
    progress: Res<Progress>,
    solids: Query<(), With<Solid>>,
    paddles: Query<(), With<Paddle>>,
) {
    for collision in collisions.read() {
        let (me, other) = (&collision.collisioner, &collision.collisionee);
        if !balls.contains(me.entity) || !solids.contains(other.entity) {
            continue;
        }

        let Ok((mut transform, mut velocity)) = balls.get_mut(me.entity) else {
            continue;
        };

        transform.translation +=
            (collision.contact.normal * collision.contact.penetration).extend(0.0);

        if velocity.dot(collision.contact.normal) >= 0. {
            continue;
        }

        match (paddles.contains(other.entity), &other.shape) {
            (true, Shape::Rectangle(paddle)) if collision.contact.normal.y > 0. => {
                let offset =
                    ((me.position.x - other.position.x) / paddle.half_size.x).clamp(-1., 1.);
                let angle = offset * MAX_PADDLE_BOUNCE_ANGLE;
                **velocity = Vec2::new(angle.sin(), angle.cos()) * current_speed(&progress);
            }
            _ => {
                **velocity = velocity.reflect(collision.contact.normal);
            }
        }
    }
}

fn out_of_bounds(
    mut commands: Commands,
    mut collisions: MessageReader<Collision>,
    mut events: MessageWriter<BallDied>,
    materials: Res<Assets<ColorMaterial>>,
    balls: Query<(&MeshMaterial2d<ColorMaterial>, &Transform, &Velocity), With<Ball>>,
    killzone: Query<(), With<OutOfBounds>>,
) {
    for collision in collisions.read() {
        let (me, other) = (&collision.collisioner, &collision.collisionee);
        if !balls.contains(me.entity) || !killzone.contains(other.entity) {
            continue;
        }
        let Ok((material_handle, transform, velocity)) = balls.get(me.entity) else {
            todo!("Handle errors properly.");
        };
        let Some(color_material) = materials.get(material_handle) else {
            todo!("Handle errors properly.");
        };

        commands.entity(me.entity).despawn();
        events.write(BallDied {
            color: color_material.color,
            position: transform.translation.xy(),
            velocity: **velocity,
        });
    }
}

fn hit_by_backboard(
    mut collisions: MessageReader<Collision>,
    balls: Query<(), With<Ball>>,
    backboard: Query<(), With<Backboard>>,
    mut progress: ResMut<Progress>,
) {
    for collision in collisions.read() {
        let (me, other) = (&collision.collisioner, &collision.collisionee);
        if !balls.contains(me.entity) || !backboard.contains(other.entity) {
            continue;
        }

        progress.backboard_touches += 1;
    }
}

fn speed_update(balls: Query<&mut Velocity, With<Ball>>, progress: Res<Progress>) {
    for mut velocity in balls {
        velocity.0 = velocity.normalize() * current_speed(&progress);
    }
}

fn current_speed(progress: &Progress) -> f32 {
    BALL_STARTING_SPEED * progress.speed_multiplier().to_f32()
}
