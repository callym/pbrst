use std::ops::{
    Add, AddAssign,
    Div, DivAssign,
    Mul, MulAssign,
    Sub, SubAssign,
};
use num::traits::Num;

use prelude::*;

#[derive(Copy, Clone, Debug)]
pub struct Spectrum {

}

impl Spectrum {
    pub fn new<N: Num>(num: N) -> Self {
        unimplemented!()
    }

    pub fn has_nans(&self) -> bool {
        unimplemented!()
    }

    pub fn y(&self) -> Float {
        unimplemented!()
    }

    pub fn is_black(&self) -> bool {
        unimplemented!()
    }
}

impl Add for Spectrum {
    type Output = Self;
    fn add(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl AddAssign for Spectrum {
    fn add_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Mul for Spectrum {
    type Output = Self;
    fn mul(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl MulAssign for Spectrum {
    fn mul_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Div for Spectrum {
    type Output = Self;
    fn div(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl DivAssign for Spectrum {
    fn div_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Sub for Spectrum {
    type Output = Self;
    fn sub(self, rhs: Self) -> Self {
        unimplemented!()
    }
}

impl SubAssign for Spectrum {
    fn sub_assign(&mut self, rhs: Self) {
        unimplemented!()
    }
}

impl Mul<Float> for Spectrum {
    type Output = Self;
    fn mul(self, rhs: Float) -> Self {
        unimplemented!()
    }
}

impl MulAssign<Float> for Spectrum {
    fn mul_assign(&mut self, rhs: Float) {
        unimplemented!()
    }
}

impl Div<Float> for Spectrum {
    type Output = Self;
    fn div(self, rhs: Float) -> Self {
        unimplemented!()
    }
}

impl DivAssign<Float> for Spectrum {
    fn div_assign(&mut self, rhs: Float) {
        unimplemented!()
    }
}
