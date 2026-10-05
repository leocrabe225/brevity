use std::fmt;
use std::ops::{Add, AddAssign, Mul};

#[derive(Clone, Copy, Debug, Default, PartialEq, Eq, PartialOrd, Ord, Hash)]
pub(super) struct Decimal2(u32);

impl Decimal2 {
    pub const ZERO: Self = Self(0);
    pub const ONE: Self = Self(100);

    pub(super) const fn from_hundredths(hundredths: u32) -> Self {
        Self(hundredths)
    }

    pub fn to_f32(self) -> f32 {
        self.0 as f32 / 100.0
    }

    pub fn integer_text(self) -> String {
        (self.0 / 100).to_string()
    }

    pub fn decimal_text(self) -> String {
        format!(".{:02}", self.0 % 100)
    }
}

impl From<u32> for Decimal2 {
    fn from(value: u32) -> Self {
        Self(value * 100)
    }
}

impl Add for Decimal2 {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        Self(self.0 + rhs.0)
    }
}

impl AddAssign for Decimal2 {
    fn add_assign(&mut self, rhs: Self) {
        self.0 += rhs.0;
    }
}

impl Mul<u32> for Decimal2 {
    type Output = Self;
    fn mul(self, rhs: u32) -> Self {
        Self(self.0 * rhs)
    }
}

impl fmt::Display for Decimal2 {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "{}{}", self.integer_text(), self.decimal_text())
    }
}
