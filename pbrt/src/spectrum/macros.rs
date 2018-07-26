macro_rules! spectrum_impl {
    ($ty:ident) => {
const __SPECTRUM_IMPL: () = {
use std::ops::{
    Add, AddAssign,
    Div, DivAssign,
    Mul, MulAssign,
    Sub, SubAssign,
    Deref, DerefMut,
};
use prelude::*;

impl Deref for $ty {
    type Target = [Float];
    fn deref(&self) -> &Self::Target {
        &self.c
    }
}

impl DerefMut for $ty {
    fn deref_mut(&mut self) -> &mut Self::Target {
        &mut self.c
    }
}

impl Add for $ty {
    type Output = Self;
    fn add(mut self, rhs: Self) -> Self {
        for (c1, c2) in self.iter_mut().zip(rhs.iter()) {
            *c1 += *c2;
        }
        self
    }
}

impl AddAssign for $ty {
    fn add_assign(&mut self, rhs: Self) {
        for (c1, c2) in self.iter_mut().zip(rhs.iter()) {
            *c1 += *c2;
        }
    }
}

impl Mul for $ty {
    type Output = Self;
    fn mul(mut self, rhs: Self) -> Self {
        for (c1, c2) in self.iter_mut().zip(rhs.iter()) {
            *c1 *= *c2;
        }
        self
    }
}

impl MulAssign for $ty {
    fn mul_assign(&mut self, rhs: Self) {
        for (c1, c2) in self.iter_mut().zip(rhs.iter()) {
            *c1 *= *c2;
        }
    }
}

impl Div for $ty {
    type Output = Self;
    fn div(mut self, rhs: Self) -> Self {
        for (c1, c2) in self.iter_mut().zip(rhs.iter()) {
            *c1 /= *c2;
        }
        self
    }
}

impl DivAssign for $ty {
    fn div_assign(&mut self, rhs: Self) {
        for (c1, c2) in self.iter_mut().zip(rhs.iter()) {
            *c1 /= *c2;
        }
    }
}

impl Sub for $ty {
    type Output = Self;
    fn sub(mut self, rhs: Self) -> Self {
        for (c1, c2) in self.iter_mut().zip(rhs.iter()) {
            *c1 -= *c2;
        }
        self
    }
}

impl SubAssign for $ty {
    fn sub_assign(&mut self, rhs: Self) {
        for (c1, c2) in self.iter_mut().zip(rhs.iter()) {
            *c1 -= *c2;
        }
    }
}

impl Add<Float> for $ty {
    type Output = Self;
    fn add(mut self, rhs: Float) -> Self {
        for c1 in self.iter_mut() {
            *c1 += rhs;
        }
        self
    }
}

impl AddAssign<Float> for $ty {
    fn add_assign(&mut self, rhs: Float) {
        for c1 in self.iter_mut() {
            *c1 += rhs;
        }
    }
}

impl Sub<Float> for $ty {
    type Output = Self;
    fn sub(mut self, rhs: Float) -> Self {
        for c1 in self.iter_mut() {
            *c1 -= rhs;
        }
        self
    }
}

impl SubAssign<Float> for $ty {
    fn sub_assign(&mut self, rhs: Float) {
        for c1 in self.iter_mut() {
            *c1 -= rhs;
        }
    }
}

impl Mul<Float> for $ty {
    type Output = Self;
    fn mul(mut self, rhs: Float) -> Self {
        for c1 in self.iter_mut() {
            *c1 *= rhs;
        }
        self
    }
}

impl MulAssign<Float> for $ty {
    fn mul_assign(&mut self, rhs: Float) {
        for c1 in self.iter_mut() {
            *c1 *= rhs;
        }
    }
}

impl Div<Float> for $ty {
    type Output = Self;
    fn div(mut self, rhs: Float) -> Self {
        let rhs = float(1.0) / rhs;
        for c1 in self.iter_mut() {
            *c1 *= rhs;
        }
        self
    }
}

impl DivAssign<Float> for $ty {
    fn div_assign(&mut self, rhs: Float) {
        let rhs = float(1.0) / rhs;
        for c1 in self.iter_mut() {
            *c1 *= rhs;
        }
    }
}
};
    };
}
