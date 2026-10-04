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
    let top = window.size().y / 2.;
    let bottom = -window.size().y / 2.;
    let left = -window.size().x / 2.;
    let right = window.size().x / 2.;

    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, top + 10., 0.0),
            ..default()
        },
        Solid,
        Static,
        Backboard,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(window.size().x, 20.0)),
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
            shape: Shape::Rectangle(Rectangle::new(window.size().x, 20.0)),
        },
    ));
    commands.spawn((
        Transform {
            translation: Vec3::new(0.0, bottom + 45., 0.0),
            ..default()
        },
        OutOfBounds,
        Collider {
            shape: Shape::Rectangle(Rectangle::new(window.size().x, 90.0)),
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
            shape: Shape::Rectangle(Rectangle::new(20., window.size().x)),
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
            shape: Shape::Rectangle(Rectangle::new(20., window.size().x)),
        },
    ));
}
