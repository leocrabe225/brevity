use super::Solid;
use crate::collision::{Collider, Shape};
use bevy::prelude::*;

#[derive(Component)]
#[require(Solid)]
pub(super) struct Wall;
#[derive(Component)]
pub(super) struct Backboard;

#[derive(Component)]
pub(super) struct Killzone;

pub(super) fn spawn(commands: &mut Commands, window: &Window) {
    let window_size = window.resolution.physical_size();
    let top = (window_size.y as f32) / 2.;
    let bottom = -(window_size.y as f32) / 2.;
    let left = -(window_size.x as f32) / 2.;
    let right = (window_size.x as f32) / 2.;

    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, top + 10., 0.0),
            ..default()
        },
        Wall,
        Backboard,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(window_size.x as f32, 20.0)),
        },
    ));
    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, bottom - 10., 0.0),
            ..default()
        },
        Wall,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(window_size.x as f32, 20.0)),
        },
    ));
    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, bottom + 45., 0.0),
            ..default()
        },
        Killzone,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(window_size.x as f32, 90.0)),
        },
    ));
    commands.spawn((
        Transform {
            translation: Vec3::new(left - 10., 0.0, 0.0),
            ..default()
        },
        Wall,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(20., window_size.x as f32)),
        },
    ));
    commands.spawn((
        Transform {
            translation: Vec3::new(right + 10., 0.0, 0.0),
            ..default()
        },
        Wall,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(20., window_size.x as f32)),
        },
    ));
}
