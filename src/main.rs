mod collision;
mod colors;
mod decimal2;
mod ecs_ext;
mod game;
mod main_menu;
mod sequence;
use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
};

use crate::sequence::{Sequence, advance};

fn main() {
    let schedule_build_settings = ScheduleBuildSettings {
        ambiguity_detection: LogLevel::Warn,
        auto_insert_apply_deferred: false,
        ..default()
    };
    App::new()
        .configure_schedules(schedule_build_settings)
        .add_plugins(DefaultPlugins)
        .init_state::<GameState>()
        .add_systems(Startup, spawn_camera)
        .add_systems(Update, quit)
        .add_systems(FixedUpdate, advance::<GameState>)
        .add_plugins((main_menu::MainMenuPlugin, game::GamePlugin))
        .run();
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    MainMenu,
    Game,
}

impl Sequence for GameState {
    fn next(self) -> Option<Self> {
        Some(match self {
            GameState::MainMenu => GameState::Game,
            GameState::Game => GameState::MainMenu,
        })
    }

    fn finish(_: &mut World) {
        unreachable!("GameState loops forever, it never finishes");
    }
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn quit(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
