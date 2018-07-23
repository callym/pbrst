use std::cmp::{ min, max };
use std::mem;
use std::ops::*;
use prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Interval {
    pub low: Float,
    pub high: Float,
}

impl Interval {
    pub fn point(v0: Float) -> Self {
        Interval::new(v0, v0)
    }

    pub fn new(v0: Float, v1: Float) -> Self {
        Self {
            low: min(v0, v1),
            high: max(v0, v1),
        }
    }

    pub fn sin(self) -> Self {
        debug_assert!(self.low >= 0.0);
        debug_assert!(self.high <= float(2.0001) * Float::pi());

        let mut sin_low = self.low.sin();
        let mut sin_high = self.high.sin();

        let pi_half = Float::pi() / float(2.0);
        let pi_3_2 = float(3.0 / 2.0) * Float::pi();

        if sin_low > sin_high {
            mem::swap(&mut sin_low, &mut sin_high);
        }

        if self.low < pi_half && self.high > pi_half {
            sin_high = float(1.0);
        }

        if self.low < pi_3_2 && self.high > pi_3_2 {
            sin_low = float(-1.0);
        }

        Interval::new(sin_low, sin_high)
    }

    pub fn cos(self) -> Self {
        debug_assert!(self.low >= 0.0);
        debug_assert!(self.high <= float(2.0001) * Float::pi());

        let mut cos_low = self.low.cos();
        let mut cos_high = self.high.cos();

        if cos_low > cos_high {
            mem::swap(&mut cos_low, &mut cos_high);
        }

        if self.low < Float::pi() && self.high > Float::pi() {
            cos_low = float(-1.0);
        }

        Interval::new(cos_low, cos_high)
    }
}

impl Add for Interval {
    type Output = Self;

    fn add(self, rhs: Self) -> Self {
        Interval::new(self.low + rhs.low, self.high + rhs.high)
    }
}

impl Sub for Interval {
    type Output = Self;

    fn sub(self, rhs: Self) -> Self {
        Interval::new(self.low - rhs.high, self.high - rhs.low)
    }
}

impl Mul for Interval {
    type Output = Self;

    fn mul(self, rhs: Self) -> Self {
        let min =   min(min(self.low * rhs.low, self.high * rhs.low),
                        min(self.low * rhs.high, self.high * rhs.high));
        let max =   max(max(self.low * rhs.low, self.high * rhs.low),
                        max(self.low * rhs.high, self.high * rhs.high));
        Interval::new(min, max)
    }
}
