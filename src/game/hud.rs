use std::f32::consts::PI;

use crate::{
    GameState,
    decimal2::Decimal2,
    game::{FollowEntity, GamePhase, SetupSet, paddle::Paddle},
};

use super::{Score, progress::Progress};
use bevy::{
    prelude::*,
    sprite::Anchor,
    text::{ComputedTextBlock, Text2dUpdateSystems, TextLayoutInfo, TextSection},
};

impl Plugin for HudPlugin {
    fn build(&self, app: &mut App) {
        app.add_observer(spawn_hints);
        app.add_systems(
            OnEnter(GameState::Game),
            spawn_scoreboard.in_set(SetupSet::Spawn),
        );
        app.add_systems(
            Update,
            (
                remove_on_press,
                disappear_hint,
                bounce_size,
                fade_out,
                speed_update_text,
                update_scoreboard,
            )
                .run_if(in_state(GamePhase::Playing)),
        );
        app.add_systems(
            PostUpdate,
            place_decimals
                .after(Text2dUpdateSystems)
                .before(TransformSystems::Propagate),
        );
    }
}

const SCOREBOARD_INTEGER_FONT_SIZE: FontSize = FontSize::Px(200.);
const SCOREBOARD_DECIMAL_FONT_SIZE: FontSize = FontSize::Px(40.);
const SPEED_FONT_SIZE: FontSize = FontSize::Px(40.);
const SPACE_HINT_FONT_SIZE: FontSize = FontSize::Px(40.);
const ARROW_HINT_FONT_SIZE: FontSize = FontSize::Px(60.);

const SPEED_UPDATE_POS: Vec2 = Vec2::new(0., -300.);
const SCORE_POS: Vec2 = Vec2::new(0., 0.);
const SPACE_HINT_Y: f32 = 95.;
const LEFT_ARROW_HINT_X: f32 = -120.;
const RIGHT_ARROW_HINT_X: f32 = 120.;

pub(super) struct HudPlugin;

#[derive(Component)]
pub(super) struct ScoreboardText;

#[derive(Component)]
pub(super) struct ScoreboardDecimals;

#[derive(Component)]
pub(super) struct Hint;

#[derive(Component)]
struct SizeBounce {
    clock: f32,
    font_size: FontSize,
}

#[derive(Component)]
struct RemoveOnPress(Vec<KeyCode>);

#[derive(Component)]
struct DisappearingHint {
    clock: f32,
    start_size: FontSize,
}

#[derive(Component)]
struct FadeOut(Timer);

fn spawn_scoreboard(mut commands: Commands) {
    commands.spawn((
        Text2d::new(Decimal2::ZERO.integer_text()),
        TextFont {
            font_size: SCOREBOARD_INTEGER_FONT_SIZE,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform {
            translation: SCORE_POS.extend(-1.),
            ..default()
        },
        ScoreboardText,
        children![(
            Text2d::new(Decimal2::ZERO.decimal_text()),
            TextFont {
                font_size: SCOREBOARD_DECIMAL_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Anchor::BOTTOM_LEFT,
            ScoreboardDecimals,
        )],
    ));
}

fn spawn_hints(add: On<Add, Paddle>, mut commands: Commands) {
    spawn_space_hint(&mut commands, add.entity);
    spawn_arrows_hint(&mut commands, add.entity);
}

fn spawn_space_hint(commands: &mut Commands, follow: Entity) {
    spawn_hint(
        commands,
        Vec2::ZERO,
        "PRESS\nSPACE",
        SPACE_HINT_FONT_SIZE,
        vec![KeyCode::Space],
        FollowEntity {
            entity: follow,
            offset: Vec2::new(0., SPACE_HINT_Y),
        },
    );
}

fn spawn_arrows_hint(commands: &mut Commands, follow: Entity) {
    spawn_hint(
        commands,
        Vec2::ZERO,
        "<",
        ARROW_HINT_FONT_SIZE,
        vec![KeyCode::ArrowLeft, KeyCode::ArrowRight],
        FollowEntity {
            entity: follow,
            offset: Vec2::new(LEFT_ARROW_HINT_X, 0.),
        },
    );
    spawn_hint(
        commands,
        Vec2::ZERO,
        ">",
        ARROW_HINT_FONT_SIZE,
        vec![KeyCode::ArrowRight, KeyCode::ArrowLeft],
        FollowEntity {
            entity: follow,
            offset: Vec2::new(RIGHT_ARROW_HINT_X, 0.),
        },
    );
}

fn spawn_hint(
    commands: &mut Commands,
    pos: Vec2,
    text: &str,
    font_size: FontSize,
    remove_keys: Vec<KeyCode>,
    extra: impl Bundle,
) {
    commands.spawn((
        Hint,
        Text2d::new(text),
        TextFont {
            font_size,
            ..default()
        },
        TextColor(Color::WHITE),
        Transform {
            translation: pos.extend(-1.),
            ..default()
        },
        SizeBounce {
            clock: 0.,
            font_size,
        },
        RemoveOnPress(remove_keys),
        extra,
    ));
}

fn remove_on_press(
    mut commands: Commands,
    query: Query<(&RemoveOnPress, &TextFont, Entity)>,
    keyboard_input: Res<ButtonInput<KeyCode>>,
) {
    for (remove, font, entity) in query {
        if keyboard_input.any_just_pressed(remove.0.clone()) {
            commands.entity(entity).insert(DisappearingHint {
                clock: 0.,
                start_size: font.font_size,
            });
            commands
                .entity(entity)
                .remove::<RemoveOnPress>()
                .remove::<SizeBounce>();
        }
    }
}

fn disappear_hint(
    mut commands: Commands,
    query: Query<(&mut DisappearingHint, &mut TextFont, Entity)>,
    time: Res<Time>,
) {
    const LENGTH: f32 = 0.3;
    for (mut disappearing, mut font, entity) in query {
        disappearing.clock += time.delta_secs();
        if disappearing.clock >= LENGTH {
            commands.entity(entity).despawn();
            continue;
        }
        let progress = EaseFunction::BackIn.sample_clamped(disappearing.clock * (1. / LENGTH));
        font.font_size = disappearing.start_size * (1. - progress);
    }
}

fn bounce_size(time: Res<Time>, mut query: Query<(&mut TextFont, &mut SizeBounce)>) {
    for (mut font, mut size_bounce) in &mut query {
        size_bounce.clock += time.delta_secs();
        let progress = 1. + 0.15 * (PI * size_bounce.clock / 1.5).sin();
        font.font_size = size_bounce.font_size * progress;
    }
}

fn fade_out(
    time: Res<Time>,
    mut commands: Commands,
    mut query: Query<(Entity, &mut FadeOut, &mut TextColor)>,
) {
    for (entity, mut fade, mut color) in &mut query {
        fade.0.tick(time.delta());
        let progress = EaseFunction::ExponentialIn.sample_clamped(fade.0.fraction());
        color.0.set_alpha(1. - progress);
        if progress >= 1. {
            commands.entity(entity).despawn();
        }
    }
}

fn speed_update_text(
    mut commands: Commands,
    progress: Res<Progress>,
    mut last: Local<Option<Decimal2>>,
) {
    let multiplier = progress.speed_multiplier();
    if last.is_some_and(|last| multiplier > last) {
        commands.spawn((
            Text2d::new(format!("x{multiplier}")),
            FadeOut(Timer::from_seconds(1.5, TimerMode::Once)),
            TextFont {
                font_size: SPEED_FONT_SIZE,
                ..default()
            },
            TextColor(Color::WHITE),
            Transform {
                translation: SPEED_UPDATE_POS.extend(0.),
                ..default()
            },
        ));
    }
    *last = Some(multiplier);
}

fn update_scoreboard(
    score: Res<Score>,
    mut integer: Single<&mut Text2d, (With<ScoreboardText>, Without<ScoreboardDecimals>)>,
    mut decimal: Single<&mut Text2d, (With<ScoreboardDecimals>, Without<ScoreboardText>)>,
) {
    let new_integer = integer.get_text_mut();
    let new_decimal = decimal.get_text_mut();

    *new_integer = score.integer_text();
    *new_decimal = score.decimal_text();
}

fn first_baseline(block: &ComputedTextBlock, layout: &TextLayoutInfo) -> Option<f32> {
    Some(block.buffer().get(0)?.metrics().baseline / layout.scale_factor)
}

fn place_decimals(
    integers: Query<(&TextLayoutInfo, &ComputedTextBlock, &Children), With<ScoreboardText>>,
    mut decimals: Query<
        (&mut Transform, &TextLayoutInfo, &ComputedTextBlock),
        With<ScoreboardDecimals>,
    >,
) {
    for (layout, block, children) in &integers {
        let Some(baseline) = first_baseline(block, layout) else {
            continue;
        };
        for child in children.iter() {
            let Ok((mut transform, decimal_layout, decimal_block)) = decimals.get_mut(child) else {
                continue;
            };
            let Some(decimal_baseline) = first_baseline(decimal_block, decimal_layout) else {
                continue;
            };
            let padding = layout.size.y - baseline;
            let decimal_padding = decimal_layout.size.y - decimal_baseline;
            transform.translation.x = layout.size.x / 2.;
            transform.translation.y = -layout.size.y / 2. + padding - decimal_padding;
        }
    }
}
