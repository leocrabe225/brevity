mod collision;
mod colors;
mod decimal2;
mod game;
mod main_menu;
use bevy::{
    ecs::schedule::{LogLevel, ScheduleBuildSettings},
    prelude::*,
};

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
        .add_plugins((main_menu::plugin, game::plugin))
        .run();
}

#[derive(States, Default, Debug, Clone, Copy, PartialEq, Eq, Hash)]
enum GameState {
    #[default]
    MainMenu,
    Game,
    GameOver,
}

fn spawn_camera(mut commands: Commands) {
    commands.spawn(Camera2d);
}

fn quit(keyboard_input: Res<ButtonInput<KeyCode>>, mut exit: MessageWriter<AppExit>) {
    if keyboard_input.just_pressed(KeyCode::Escape) {
        exit.write(AppExit::Success);
    }
}
