use bevy::prelude::*;

const SPEED_STEP_PER_CHECKPOINT: f32 = 0.10;
const SPEED_STEP_PER_BACKBOARD_HIT: f32 = 0.01;
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
    pub(super) fn speed_multiplier(&self) -> f32 {
        1. + SPEED_STEP_PER_BACKBOARD_HIT * self.backboard_touched as f32
            + SPEED_STEP_PER_CHECKPOINT
                * [
                    self.bricks_hit >= FIRST_HIT_CHECKPOINT,
                    self.bricks_hit >= SECOND_HIT_CHECKPOINT,
                    self.orange_reached,
                    self.red_reached,
                ]
                .into_iter()
                .map(f32::from)
                .sum::<f32>()
    }
}
