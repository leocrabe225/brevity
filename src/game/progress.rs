use std::mem::discriminant;

use bevy::prelude::*;

use crate::{decimal2::Decimal2, game::GameSet};

impl Plugin for ProgressPlugin {
    fn build(&self, app: &mut App) {
        app.add_message::<SpeedUp>();
        app.insert_resource(Progress { ..default() });
        app.add_systems(
            FixedUpdate,
            detect_speedups
                .run_if(resource_changed::<Progress>)
                .in_set(GameSet::Rules),
        );
    }
}

const SPEED_STEP_PER_CHECKPOINT: Decimal2 = Decimal2::from_hundredths(10);
const SPEED_STEP_PER_BACKBOARD_HIT: Decimal2 = Decimal2::from_hundredths(1);
const FIRST_HIT_CHECKPOINT: u32 = 4;
const SECOND_HIT_CHECKPOINT: u32 = 12;

pub(super) struct ProgressPlugin;

#[derive(Message)]
pub(super) struct SpeedUp {
    pub(super) reason: SpeedBonus,
    pub(super) gained: Decimal2,
}

#[derive(PartialEq, Copy, Clone)]
pub(super) enum SpeedBonus {
    FirstHitCheckpoint,
    SecondHitCheckpoint,
    OrangeRow,
    RedRow,
    BackboardTouched(u32),
}

#[derive(Resource, Default, Copy, Clone)]
pub(super) struct Progress {
    pub(super) bricks_hit: u32,
    pub(super) orange_reached: bool,
    pub(super) red_reached: bool,
    pub(super) backboard_touches: u32,
}

impl SpeedBonus {
    fn bonus(self) -> Decimal2 {
        match self {
            SpeedBonus::FirstHitCheckpoint
            | SpeedBonus::SecondHitCheckpoint
            | SpeedBonus::OrangeRow
            | SpeedBonus::RedRow => SPEED_STEP_PER_CHECKPOINT,
            SpeedBonus::BackboardTouched(touches) => SPEED_STEP_PER_BACKBOARD_HIT * touches,
        }
    }
}

impl Progress {
    pub(super) fn speed_bonuses(&self) -> impl Iterator<Item = SpeedBonus> {
        [
            (self.bricks_hit >= FIRST_HIT_CHECKPOINT).then_some(SpeedBonus::FirstHitCheckpoint),
            (self.bricks_hit >= SECOND_HIT_CHECKPOINT).then_some(SpeedBonus::SecondHitCheckpoint),
            self.orange_reached.then_some(SpeedBonus::OrangeRow),
            self.red_reached.then_some(SpeedBonus::RedRow),
            (self.backboard_touches > 0)
                .then_some(SpeedBonus::BackboardTouched(self.backboard_touches)),
        ]
        .into_iter()
        .flatten()
    }
    pub(super) fn speed_multiplier(&self) -> Decimal2 {
        Decimal2::ONE + self.speed_bonuses().map(SpeedBonus::bonus).sum()
    }
}

fn detect_speedups(
    progress: Res<Progress>,
    mut previous: Local<Vec<SpeedBonus>>,
    mut speedups: MessageWriter<SpeedUp>,
) {
    let current: Vec<SpeedBonus> = progress.speed_bonuses().collect();
    speedups.write_batch(
        current
            .iter()
            .filter(|bonus| !previous.contains(bonus))
            .map(|&reason| {
                let before = previous
                    .iter()
                    .find(|old| discriminant(*old) == discriminant(&reason))
                    .map_or(Decimal2::ZERO, |&old| old.bonus());
                SpeedUp {
                    reason,
                    gained: reason.bonus() - before,
                }
            }),
    );

    *previous = current;
}
