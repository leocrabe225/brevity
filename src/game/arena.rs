use super::{Solid, Static};
use crate::{
    GameState,
    collision::{Collider, Shape},
    game::SetupSet,
};
use bevy::prelude::*;

impl Plugin for ArenaPlugin {
    fn build(&self, app: &mut App) {
        app.add_systems(OnEnter(GameState::Game), spawn.in_set(SetupSet::Spawn));
    }
}

pub(super) struct ArenaPlugin;

#[derive(Component)]
pub(super) struct Backboard;

#[derive(Component)]
pub(super) struct OutOfBounds;

fn spawn(mut commands: Commands, window: Single<&Window>) {
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
        Solid,
        Static,
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
        Solid,
        Static,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(window_size.x as f32, 20.0)),
        },
    ));
    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, bottom + 45., 0.0),
            ..default()
        },
        OutOfBounds,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(window_size.x as f32, 90.0)),
        },
    ));
    commands.spawn((
        Transform {
            translation: Vec3::new(left - 10., 0.0, 0.0),
            ..default()
        },
        Solid,
        Static,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(20., window_size.x as f32)),
        },
    ));
    commands.spawn((
        Transform {
            translation: Vec3::new(right + 10., 0.0, 0.0),
            ..default()
        },
        Solid,
        Static,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(20., window_size.x as f32)),
        },
    ));
}
