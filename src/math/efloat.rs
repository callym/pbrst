use std::{
    self,
    cmp::{ PartialEq, PartialOrd, Ordering },
    num::FpCategory,
    ops::{ Mul, Div, Rem, Neg, MulAssign, DivAssign, RemAssign },
};
use cg::ApproxEq;
use noisy_float::types::{ R32, r32 };
use num;
use num::Float as NumFloat;
use num::traits::{ Bounded, Num, One, Zero };
use num::traits::ParseFloatError;
use prelude::*;

#[derive(
    Copy, Clone, Debug, Eq, PartialEq, Ord, PartialOrd,
    Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign,
    Shrinkwrap
)]
#[shrinkwrap(mutable, unsafe_ignore_visibility)]
pub struct Efloat(Float);

pub fn efloat0(f: impl Into<Float>) -> Efloat {
    efloat(f, 0.0)
}

pub fn efloat(f: impl Into<Float>, _err: impl Into<Float>) -> Efloat {
    let f: Float = f.into();
    Efloat(f)
}


impl Efloat {
    pub fn lower_bound(&self) -> Self {
        unimplemented!()
    }

    pub fn upper_bound(&self) -> Self {
        unimplemented!()
    }

    pub fn pow(self, n: i32) -> Self {
        self.powi(n)
    }

    pub fn lerp(self, o: Self, t: Self) -> Self {
        (efloat(1.0, 0.0) - t) * self + efloat(1.0, 0.0) * o
    }
}

impl From<FloatPrim> for Efloat {
    #[inline(always)]
    fn from(f: FloatPrim) -> Self {
        efloat(f, 0.0)
    }
}


impl ApproxEq for Efloat {
    type Epsilon = Self;

    #[inline(always)]
    fn default_epsilon() -> Self {
        efloat(<FloatPrim as ApproxEq>::default_epsilon(), 0.0)
    }

    #[inline(always)]
    fn default_max_relative() -> Self {
        efloat(<FloatPrim as ApproxEq>::default_max_relative(), 0.0)
    }

    #[inline(always)]
    fn default_max_ulps() -> u32 {
        <FloatPrim as ApproxEq>::default_max_ulps()
    }

    #[inline(always)]
    fn relative_eq(&self, other: &Self, epsilon: Self, max_relative: Self) -> bool {
        <FloatPrim as ApproxEq>::relative_eq(&(*self).raw(), &other.0.raw(), epsilon.0.raw(), max_relative.0.raw())
    }

    #[inline(always)]
    fn ulps_eq(&self, other: &Self, epsilon: Self, max_ulps: u32) -> bool {
        <FloatPrim as ApproxEq>::ulps_eq(&(*self).raw(), &other.0.raw(), epsilon.0.raw(), max_ulps)
    }
}

impl Num for Efloat {
    type FromStrRadixErr = ParseFloatError;
    #[inline(always)]
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        <FloatPrim as Num>::from_str_radix(src, radix)
            .map(|f| efloat(f, 0.0))
    }
}

impl One for Efloat {
    #[inline(always)]
    fn one() -> Self {
        efloat(1.0, 0.0)
    }
}

impl Zero for Efloat {
    #[inline(always)]
    fn zero() -> Self {
        efloat(0.0, 0.0)
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        (*self).is_zero()
    }
}

impl Bounded for Efloat {
    #[inline(always)]
    fn min_value() -> Self {
        efloat(<FloatPrim as Bounded>::min_value(), 0.0)
    }

    #[inline(always)]
    fn max_value() -> Self {
        efloat(<FloatPrim as Bounded>::max_value(), 0.0)
    }
}

impl num::ToPrimitive for Efloat {
    #[inline(always)]
    fn to_i64(&self) -> Option<i64> {
        <FloatPrim as num::ToPrimitive>::to_i64(&(*self).raw())
    }

    #[inline(always)]
    fn to_u64(&self) -> Option<u64> {
        <FloatPrim as num::ToPrimitive>::to_u64(&(*self).raw())
    }
}

impl num::NumCast for Efloat {
    #[inline(always)]
    fn from<T: num::ToPrimitive>(n: T) -> Option<Self> {
        <FloatPrim as num::NumCast>::from(n).map(|n| efloat(n, 0.0))
    }
}

impl PartialEq<FloatPrim> for Efloat {
    #[inline(always)]
    fn eq(&self, other: &FloatPrim) -> bool {
        self.eq(&efloat(*other, 0.0))
    }
}

impl PartialOrd<FloatPrim> for Efloat {
    #[inline(always)]
    fn partial_cmp(&self, other: &FloatPrim) -> Option<Ordering> {
        self.partial_cmp(&efloat(*other, 0.0))
    }
}

impl Mul for Efloat {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        efloat(self.0 * rhs.0, 0.0)
    }
}

impl MulAssign for Efloat {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for Efloat {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        efloat(self.0 / rhs.0, 0.0)
    }
}

impl DivAssign for Efloat {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Rem for Efloat {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        efloat(self.0 % rhs.0, 0.0)
    }
}

impl RemAssign for Efloat {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl Neg for Efloat {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        efloat(-(*self).raw(), 0.0)
    }
}

impl NumFloat for Efloat {
    #[inline(always)]
    fn nan() -> Self {
        efloat(FloatPrim::nan(), 0.0)
    }

    #[inline(always)]
    fn infinity() -> Self {
        efloat(FloatPrim::infinity(), 0.0)
    }

    #[inline(always)]
    fn neg_infinity() -> Self {
        efloat(FloatPrim::neg_infinity(), 0.0)
    }

    #[inline(always)]
    fn neg_zero() -> Self {
        efloat(FloatPrim::zero(), 0.0)
    }

    #[inline(always)]
    fn min_value() -> Self {
        efloat(<FloatPrim as Bounded>::min_value(), 0.0)
    }

    #[inline(always)]
    fn min_positive_value() -> Self {
        efloat(FloatPrim::min_positive_value(), 0.0)
    }

    #[inline(always)]
    fn max_value() -> Self {
        efloat(<FloatPrim as Bounded>::max_value(), 0.0)
    }

    #[inline(always)]
    fn is_nan(self) -> bool {
        (*self).is_nan()
    }

    #[inline(always)]
    fn is_infinite(self) -> bool {
        (*self).is_infinite()
    }

    #[inline(always)]
    fn is_finite(self) -> bool {
        (*self).is_finite()
    }

    #[inline(always)]
    fn is_normal(self) -> bool {
        (*self).is_normal()
    }

    #[inline(always)]
    fn classify(self) -> FpCategory {
        (*self).classify()
    }

    #[inline(always)]
    fn floor(self) -> Self {
        efloat((*self).floor(), 0.0)
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        efloat((*self).ceil(), 0.0)
    }

    #[inline(always)]
    fn round(self) -> Self {
        efloat((*self).round(), 0.0)
    }

    #[inline(always)]
    fn trunc(self) -> Self {
        efloat((*self).trunc(), 0.0)
    }

    #[inline(always)]
    fn fract(self) -> Self {
        efloat((*self).fract(), 0.0)
    }

    #[inline(always)]
    fn abs(self) -> Self {
        efloat((*self).abs(), 0.0)
    }

    #[inline(always)]
    fn signum(self) -> Self {
        efloat((*self).signum(), 0.0)
    }

    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        (*self).is_sign_positive()
    }

    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        (*self).is_sign_negative()
    }

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        efloat((*self).mul_add(a.0, b.0), 0.0)
    }

    #[inline(always)]
    fn recip(self) -> Self {
        efloat((*self).recip(), 0.0)
    }

    #[inline(always)]
    fn powi(self, n: i32) -> Self {
        efloat((*self).powi(n), 0.0)
    }

    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        efloat((*self).powf(*n), 0.0)
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        efloat((*self).sqrt(), 0.0)
    }

    #[inline(always)]
    fn exp(self) -> Self {
        efloat((*self).exp(), 0.0)
    }

    #[inline(always)]
    fn exp2(self) -> Self {
        efloat((*self).exp2(), 0.0)
    }

    #[inline(always)]
    fn ln(self) -> Self {
        efloat((*self).ln(), 0.0)
    }

    #[inline(always)]
    fn log(self, base: Self) -> Self {
        efloat((*self).log(*base), 0.0)
    }

    #[inline(always)]
    fn log2(self) -> Self {
        efloat((*self).log2(), 0.0)
    }

    #[inline(always)]
    fn log10(self) -> Self {
        efloat((*self).log10(), 0.0)
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        efloat((*self).raw().max(other.0.raw()), 0.0)
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        efloat((*self).raw().min(other.0.raw()), 0.0)
    }

    #[inline(always)]
    fn abs_sub(self, other: Self) -> Self {
        efloat((*self).abs_sub(*other), 0.0)
    }

    #[inline(always)]
    fn cbrt(self) -> Self {
        efloat((*self).cbrt(), 0.0)
    }

    #[inline(always)]
    fn hypot(self, other: Self) -> Self {
        efloat((*self).hypot(*other), 0.0)
    }

    #[inline(always)]
    fn sin(self) -> Self {
        efloat((*self).sin(), 0.0)
    }

    #[inline(always)]
    fn cos(self) -> Self {
        efloat((*self).cos(), 0.0)
    }

    #[inline(always)]
    fn tan(self) -> Self {
        efloat((*self).tan(), 0.0)
    }

    #[inline(always)]
    fn asin(self) -> Self {
        efloat((*self).asin(), 0.0)
    }

    #[inline(always)]
    fn acos(self) -> Self {
        efloat((*self).acos(), 0.0)
    }

    #[inline(always)]
    fn atan(self) -> Self {
        efloat((*self).atan(), 0.0)
    }

    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        efloat((*self).atan2(*other), 0.0)
    }

    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        let (a, b) = (*self).sin_cos();
        (efloat(a, 0.0), efloat(b, 0.0))
    }

    #[inline(always)]
    fn exp_m1(self) -> Self {
        efloat((*self).exp_m1(), 0.0)
    }

    #[inline(always)]
    fn ln_1p(self) -> Self {
        efloat((*self).ln_1p(), 0.0)
    }

    #[inline(always)]
    fn sinh(self) -> Self {
        efloat((*self).sinh(), 0.0)
    }

    #[inline(always)]
    fn cosh(self) -> Self {
        efloat((*self).cosh(), 0.0)
    }

    #[inline(always)]
    fn tanh(self) -> Self {
        efloat((*self).tanh(), 0.0)
    }

    #[inline(always)]
    fn asinh(self) -> Self {
        efloat((*self).asinh(), 0.0)
    }

    #[inline(always)]
    fn acosh(self) -> Self {
        efloat((*self).acosh(), 0.0)
    }

    #[inline(always)]
    fn atanh(self) -> Self {
      efloat((*self).atanh(), 0.0)
    }

    #[inline(always)]
    fn integer_decode(self) -> (u64, i16, i8) {
        (*self).integer_decode()
    }
}
