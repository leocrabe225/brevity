use std::f32::consts::{FRAC_PI_2, PI};

use bevy::prelude::*;

use crate::game::{AngularVelocity, GameSet, Velocity, ball::ball_visuals};

use super::{BALL_DIAMETER, BALL_RADIUS, BallDied};

impl Plugin for BallDeathPlugin {
    fn build(&self, app: &mut bevy::app::App) {
        app.add_systems(
            FixedUpdate,
            (
                (death_animation, tick_laser, tick_lasered).in_set(GameSet::Rules),
                handle_ball_died.in_set(GameSet::Resolution),
            ),
        );
    }
}

const LASER_ORIGIN: Vec2 = Vec2::new(0., -400.);
const LASER_HEIGHT: f32 = 0.5;

const LASER_TIME: f32 = 0.2;
const DEATH_TIMER: f32 = 0.6;
const FRAGMENT_MIN_VELOCITY: f32 = 10.;
const FRAGMENT_MAX_VELOCITY: f32 = 500.;
const FRAGMENT_MIN_ANGULAR_SPEED: f32 = 0.5 * PI_2;
const FRAGMENT_MAX_ANGULAR_SPEED: f32 = 2. * PI_2;
const PI_2: f32 = PI * 2.;
const MIN_FRAGMENT_ANGLE: f32 = (PI_2) / 36.;
const MAX_FRAGMENT_ANGLE: f32 = (PI_2) / 10.;

pub(super) struct BallDeathPlugin;

#[derive(Component)]
struct Dying {
    timer: Timer,
    start_velocity: Vec2,
    start_angular_velocity: f32,
}

#[derive(Component, Clone, Default)]
#[require(Transform)]
pub(super) struct BeingLasered {
    timer: Timer,
}

impl BeingLasered {
    pub(super) fn new() -> Self {
        Self {
            timer: Timer::from_seconds(LASER_TIME, TimerMode::Once),
        }
    }
}

#[derive(Component, Clone, FromTemplate)]
struct Laser {
    timer: Timer,
    target: Entity,
}

fn handle_ball_died(mut commands: Commands, mut events: MessageReader<BallDied>) {
    for ball_died in events.read() {
        let translation = ball_died.position.extend(5.);
        let entity = commands
            .spawn_scene(bsn! {
                ball_visuals(ball_died.color)
                Transform::from_translation(translation)
                Velocity({ball_died.velocity})
                BeingLasered::new()
            })
            .id();
        commands.spawn_scene(bsn! {
            Mesh2d(asset_value(Rectangle::new(1., 1.)))
            MeshMaterial2d::<ColorMaterial>(asset_value(ColorMaterial {
                color: Color::srgb_u8(255, 0, 0),
                alpha_mode: bevy::sprite_render::AlphaMode2d::Blend,
                ..default()
            }))
            template_value(get_laser_transform(ball_died.position))
            Laser {
                timer: Timer::from_seconds(LASER_TIME, TimerMode::Once),
                target: entity,
            }
        });
    }
}

fn tick_laser(
    mut commands: Commands,
    time: Res<Time>,
    lasers: Query<(&mut Laser, &mut Transform, Entity)>,
    lasereds: Query<&Transform, (With<BeingLasered>, Without<Laser>)>,
) {
    for (mut laser, mut transform, entity) in lasers {
        laser.timer.tick(time.delta());

        let Ok(lasered_transform) = lasereds.get(laser.target) else {
            todo!("Handle errors properly.");
        };

        *transform = get_laser_transform(lasered_transform.translation.xy());

        if laser.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}

fn tick_lasered(
    mut commands: Commands,
    time: Res<Time>,
    lasereds: Query<(
        &mut BeingLasered,
        &MeshMaterial2d<ColorMaterial>,
        &Transform,
        Entity,
    )>,
    mut meshes: ResMut<Assets<Mesh>>,
    mut materials: ResMut<Assets<ColorMaterial>>,
) {
    for (mut lasered, material_handle, transform, entity) in lasereds {
        lasered.timer.tick(time.delta());
        if !lasered.timer.is_finished() {
            continue;
        }
        let Some(material) = materials.get(material_handle) else {
            todo!("Handle errors properly.");
        };
        let color = material.color;
        let position = transform.translation.xy();
        spawn_ball_fragments(
            &mut commands,
            &mut meshes,
            &mut materials,
            color,
            position,
            half_circle_circular_sections_width(),
            get_laser_rotation(position),
        );
        spawn_ball_fragments(
            &mut commands,
            &mut meshes,
            &mut materials,
            color,
            position,
            half_circle_circular_sections_width(),
            get_laser_rotation(position) * Rot2::PI,
        );
        commands.entity(entity).despawn();
    }
}

fn half_circle_circular_sections_width() -> Vec<f32> {
    let mut widths: Vec<f32> = vec![];
    let mut cursor: f32 = 0.;
    while cursor < PI - MAX_FRAGMENT_ANGLE {
        let width: f32 = match cursor {
            x if ((PI - MIN_FRAGMENT_ANGLE - MAX_FRAGMENT_ANGLE)..PI).contains(&x) => {
                rand::random_range(MIN_FRAGMENT_ANGLE..(PI - x - MIN_FRAGMENT_ANGLE))
            }
            _ => rand::random_range(MIN_FRAGMENT_ANGLE..MAX_FRAGMENT_ANGLE),
        };
        widths.push(width);
        cursor += width;
    }
    widths.push(PI - cursor);
    widths
}

fn spawn_ball_fragments(
    commands: &mut Commands<'_, '_>,
    meshes: &mut ResMut<'_, Assets<Mesh>>,
    materials: &mut ResMut<'_, Assets<ColorMaterial>>,
    color: Color,
    position: Vec2,
    widths: Vec<f32>,
    start_rotation: Rot2,
) {
    let mut rotation_cursor = start_rotation;
    let extra_velocity = start_rotation * Rot2::FRAC_PI_2 * Vec2::X * 50.;
    for width in widths {
        let center = rotation_cursor * Rot2::radians(width / 2.);
        spawn_ball_fragment(
            commands,
            meshes,
            materials,
            color,
            position,
            width,
            center,
            center
                * Vec2::X
                * (EaseFunction::ExponentialIn.sample_clamped(rand::random_range((0.)..1.))
                    * (FRAGMENT_MAX_VELOCITY - FRAGMENT_MIN_VELOCITY)
                    + FRAGMENT_MIN_VELOCITY)
                + extra_velocity,
        );
        rotation_cursor *= Rot2::radians(width);
    }
}

fn spawn_ball_fragment(
    commands: &mut Commands,
    meshes: &mut Assets<Mesh>,
    materials: &mut Assets<ColorMaterial>,
    color: Color,
    position: Vec2,
    angle: f32,
    rotation: Rot2,
    velocity: Vec2,
) {
    let alpha = angle / 2.;
    let d = (BALL_DIAMETER * alpha.sin()) / (alpha * 3.);
    let mesh = Mesh::from(CircularSector::from_radians(BALL_RADIUS, angle));
    let mesh_handle = meshes.add(
        mesh.translated_by((Vec2::NEG_Y * d).extend(0.))
            .rotated_by(Quat::from_rotation_z(-FRAC_PI_2)),
    );
    let angular_velocity =
        rand::random_range(FRAGMENT_MIN_ANGULAR_SPEED..FRAGMENT_MAX_ANGULAR_SPEED)
            * if rand::random_bool(0.5) { 1. } else { -1. };
    commands.spawn((
        Mesh2d(mesh_handle),
        MeshMaterial2d(materials.add(ColorMaterial {
            color,
            alpha_mode: bevy::sprite_render::AlphaMode2d::Blend,
            ..default()
        })),
        Transform {
            translation: (position + rotation * Vec2::X * d).extend(5.),
            rotation: Quat::from_rotation_z(rotation.as_radians()),
            ..default()
        },
        Velocity(velocity),
        AngularVelocity(angular_velocity),
        Dying {
            timer: Timer::from_seconds(DEATH_TIMER, TimerMode::Once),
            start_velocity: velocity,
            start_angular_velocity: angular_velocity,
        },
    ));
}

fn get_laser_size(target_position: Vec2) -> Vec2 {
    Vec2::new(target_position.distance(LASER_ORIGIN), LASER_HEIGHT)
}

fn get_laser_position(target_position: Vec2) -> Vec2 {
    LASER_ORIGIN + (target_position - LASER_ORIGIN) / 2.
}

fn get_laser_rotation(target_position: Vec2) -> Rot2 {
    Rot2::radians((target_position - LASER_ORIGIN).to_angle())
}

fn get_laser_transform(target_position: Vec2) -> Transform {
    Transform {
        translation: get_laser_position(target_position).extend(5.),
        rotation: Quat::from_rotation_z(get_laser_rotation(target_position).as_radians()),
        scale: get_laser_size(target_position).extend(1.),
    }
}

fn death_animation(
    mut commands: Commands,
    time: Res<Time>,
    mut materials: ResMut<Assets<ColorMaterial>>,
    query: Query<(
        &mut Dying,
        &mut Velocity,
        &mut AngularVelocity,
        &MeshMaterial2d<ColorMaterial>,
        Entity,
    )>,
) {
    for (mut dying, mut velocity, mut angular_velocity, material_handle, entity) in query {
        const SPLIT: f32 = 0.8;
        const INVERSE_RATIO: f32 = 1. / (1. - SPLIT);
        dying.timer.tick(time.delta());
        let speed_progress = EaseFunction::Linear.sample_clamped(dying.timer.fraction());

        let Some(mut material) = materials.get_mut(material_handle) else {
            todo!("Handle errors properly.");
        };

        if dying.timer.fraction() > SPLIT {
            let alpha_progress = EaseFunction::Linear
                .sample_clamped((dying.timer.fraction() - SPLIT) * INVERSE_RATIO);
            material.color.set_alpha(1. - alpha_progress);
        }
        **velocity = dying.start_velocity * (1. - speed_progress / 1.5);
        **angular_velocity = dying.start_angular_velocity * (1. - speed_progress / 1.5);

        if dying.timer.is_finished() {
            commands.entity(entity).despawn();
        }
    }
}
