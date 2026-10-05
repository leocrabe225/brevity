use bevy::prelude::*;

use crate::decimal2::Decimal2;

const SPEED_STEP_PER_CHECKPOINT: Decimal2 = Decimal2::from_hundredths(10);
const SPEED_STEP_PER_BACKBOARD_HIT: Decimal2 = Decimal2::from_hundredths(1);
const FIRST_HIT_CHECKPOINT: u32 = 4;
const SECOND_HIT_CHECKPOINT: u32 = 12;

#[derive(Resource, Default)]
pub(super) struct Progress {
    pub(super) bricks_hit: u32,
    pub(super) orange_reached: bool,
    pub(super) red_reached: bool,
    pub(super) backboard_touched: u32,
}
impl Progress {
    pub(super) fn speed_multiplier(&self) -> Decimal2 {
        Decimal2::ONE
            + SPEED_STEP_PER_BACKBOARD_HIT * self.backboard_touched
            + SPEED_STEP_PER_CHECKPOINT
                * [
                    self.bricks_hit >= FIRST_HIT_CHECKPOINT,
                    self.bricks_hit >= SECOND_HIT_CHECKPOINT,
                    self.orange_reached,
                    self.red_reached,
                ]
                .into_iter()
                .map(u32::from)
                .sum::<u32>()
    }
}
