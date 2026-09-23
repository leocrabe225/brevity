use bevy::prelude::*;

pub(super) trait BreakoutColors {
    const BRICK_RED: Color;
    const BRICK_ORANGE: Color;
    const BRICK_GREEN: Color;
    const BRICK_YELLOW: Color;
}

impl BreakoutColors for Color {
    const BRICK_RED: Color = Color::srgb_u8(166, 30, 10);
    const BRICK_ORANGE: Color = Color::srgb_u8(196, 134, 10);
    const BRICK_GREEN: Color = Color::srgb_u8(10, 134, 51);
    const BRICK_YELLOW: Color = Color::srgb_u8(196, 196, 41);
}
