use super::GameState;
use crate::colors::BreakoutColors;
use bevy::prelude::*;

pub(super) fn plugin(app: &mut App) {
    app.add_systems(OnEnter(GameState::MainMenu), setup)
        .add_systems(Update, start_game.run_if(in_state(GameState::MainMenu)));
}

fn setup(mut commands: Commands) {
    commands.spawn_scene(bsn![
            Node {
                width: percent(100),
                height: percent(100),
                align_items: AlignItems::Center,
                justify_content: JustifyContent::Center,
            }
            Children [
                Node {
                    top: px(100),
                    position_type: PositionType::Absolute,
                }
                colored_text("BREAKOUT", FontSize::Px(67.), 2, &[Color::BRICK_YELLOW, Color::BRICK_GREEN, Color::BRICK_ORANGE, Color::BRICK_RED])
            ]
        ]);
}

fn colored_text(
    text: &str,
    font_size: FontSize,
    chunk_size: usize,
    colors: &[Color],
) -> impl Scene {
    let chars: Vec<char> = text.chars().collect();
    let spans: Vec<_> = chars
        .chunks(chunk_size)
        .enumerate()
        .map(|(i, pair)| {
            let pair: String = pair.iter().collect();
            let color = colors[i % colors.len()];
            bsn! {
                TextSpan(pair)
                TextColor(color)
                TextFont {font_size}
            }
        })
        .collect();

    bsn! {
        Text
        Children [{spans}]
    }
}

fn start_game(keys: Res<ButtonInput<KeyCode>>, mut next: ResMut<NextState<GameState>>) {
    if keys.just_pressed(KeyCode::Enter) {
        next.set(GameState::Game);
    }
}
