use std::{
    self,
    cmp::{ PartialEq, PartialOrd, Ordering },
    fmt::{ self, Debug },
    num::FpCategory,
    ops::{ Mul, Div, Rem, Neg, MulAssign, DivAssign, RemAssign },
};

#[cfg(not(feature = "double"))]
use std::f32::consts;

#[cfg(feature = "double")]
use std::f64::consts;

use cgmath::ApproxEq;
use derive_more::{
    Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign,
};
use num;
use num::Float as NumFloat;
use num::traits::{ Bounded, Num, One, Zero, NumCast };
use num::traits::ParseFloatError;
use shrinkwraprs::Shrinkwrap;
use rand::Rng;
use rand::distributions::{ Distribution, Standard };

macro_rules! float_define {
    ($prim:ident, $noisy_ty:ident, $noisy_fn:ident) => {
        #[macro_use]
        pub mod float {
            use noisy_float::types::{ $noisy_ty, $noisy_fn };

            pub(crate) const CONV: fn(FloatPrim) -> FloatNoisy = $noisy_fn;
            pub type FloatNoisy = $noisy_ty;
            pub type FloatPrim = $prim;

            #[macro_export]
            macro_rules! float_const {
                ($fn:ident, $name:ident) => {
                    #[inline(always)]
                    pub fn $fn() -> Self {
                        float(std::$prim::consts::$name)
                    }
                };
            }
        }
    };
}

#[cfg(not(feature = "double"))]
float_define!(f32, N32, n32);

#[cfg(feature = "double")]
float_define!(f64, N64, n64);

pub use self::float::*;

pub fn float(f: impl NumCast) -> Float {
    Float(CONV(num::cast::cast(f).unwrap()))
}

#[derive(
    Copy, Clone, Eq, PartialEq, Ord, PartialOrd,
    Add, Sub, Mul, Div, Rem,
    AddAssign, SubAssign,
    Shrinkwrap
)]
#[shrinkwrap(mutable, unsafe_ignore_visibility)]
pub struct Float(FloatNoisy);

impl Debug for Float {
    fn fmt(&self, f: &mut fmt::Formatter<'_>) -> fmt::Result {
        <FloatPrim as Debug>::fmt(&self.raw(), f)
    }
}

impl Float {
    float_const!(pi, PI);
    float_const!(frac_1_pi, FRAC_1_PI);
    float_const!(frac_pi_2, FRAC_PI_2);
    float_const!(frac_pi_4, FRAC_PI_4);

    #[inline(always)]
    pub fn inv_2_pi() -> Self {
        const FRAC_2_PI: FloatPrim = 1.0 / (2.0 * consts::PI);
        float(FRAC_2_PI)
    }

    #[inline(always)]
    pub fn inv_4_pi() -> Self {
        const FRAC_4_PI: FloatPrim = 1.0 / (4.0 * consts::PI);
        float(FRAC_4_PI)
    }

    pub fn raw(self) -> FloatPrim {
        self.0.raw()
    }

    pub fn pow(self, n: i32) -> Self {
        self.powi(n)
    }

    pub fn lerp(self, other: Self, amount: Self) -> Self {
        (float(1.0) - amount) * self + float(1.0) * other
    }
}

impl Distribution<Float> for Standard {
    fn sample<R: Rng + ?Sized>(&self, rng: &mut R) -> Float {
        float(rng.gen::<FloatPrim>())
    }
}

impl From<FloatPrim> for Float {
    #[inline(always)]
    fn from(f: FloatPrim) -> Self {
        float(f)
    }
}

impl Into<FloatPrim> for Float {
    #[inline(always)]
    fn into(self) -> FloatPrim {
        self.raw()
    }
}

impl From<FloatNoisy> for Float {
    #[inline(always)]
    fn from(f: FloatNoisy) -> Self {
        Float(f)
    }
}

impl Into<FloatNoisy> for Float {
    #[inline(always)]
    fn into(self) -> FloatNoisy {
        self.0
    }
}

impl ApproxEq for Float {
    type Epsilon = Self;

    #[inline(always)]
    fn default_epsilon() -> Self {
        float(<FloatPrim as ApproxEq>::default_epsilon())
    }

    #[inline(always)]
    fn default_max_relative() -> Self {
        float(<FloatPrim as ApproxEq>::default_max_relative())
    }

    #[inline(always)]
    fn default_max_ulps() -> u32 {
        <FloatPrim as ApproxEq>::default_max_ulps()
    }

    #[inline(always)]
    fn relative_eq(&self, other: &Self, epsilon: Self, max_relative: Self) -> bool {
        <FloatPrim as ApproxEq>::relative_eq(&self.0.raw(), &other.0.raw(), epsilon.0.raw(), max_relative.0.raw())
    }

    #[inline(always)]
    fn ulps_eq(&self, other: &Self, epsilon: Self, max_ulps: u32) -> bool {
        <FloatPrim as ApproxEq>::ulps_eq(&self.0.raw(), &other.0.raw(), epsilon.0.raw(), max_ulps)
    }
}

impl Num for Float {
    type FromStrRadixErr = ParseFloatError;
    #[inline(always)]
    fn from_str_radix(src: &str, radix: u32) -> Result<Self, Self::FromStrRadixErr> {
        <FloatPrim as Num>::from_str_radix(src, radix)
            .map(float)
    }
}

impl One for Float {
    #[inline(always)]
    fn one() -> Float {
        float(1.0)
    }
}

impl Zero for Float {
    #[inline(always)]
    fn zero() -> Float {
        float(0.0)
    }

    #[inline(always)]
    fn is_zero(&self) -> bool {
        self.0.is_zero()
    }
}

impl Bounded for Float {
    #[inline(always)]
    fn min_value() -> Self {
        float(<FloatPrim as Bounded>::min_value())
    }

    #[inline(always)]
    fn max_value() -> Self {
        float(<FloatPrim as Bounded>::max_value())
    }
}

impl num::ToPrimitive for Float {
    #[inline(always)]
    fn to_i64(&self) -> Option<i64> {
        <FloatPrim as num::ToPrimitive>::to_i64(&self.0.raw())
    }

    #[inline(always)]
    fn to_u64(&self) -> Option<u64> {
        <FloatPrim as num::ToPrimitive>::to_u64(&self.0.raw())
    }
}

impl num::NumCast for Float {
    #[inline(always)]
    fn from<T: num::ToPrimitive>(n: T) -> Option<Self> {
        <FloatPrim as num::NumCast>::from(n).map(float)
    }
}

impl PartialEq<FloatPrim> for Float {
    #[inline(always)]
    fn eq(&self, other: &FloatPrim) -> bool {
        self.eq(&float(*other))
    }
}

impl PartialOrd<FloatPrim> for Float {
    #[inline(always)]
    fn partial_cmp(&self, other: &FloatPrim) -> Option<Ordering> {
        self.partial_cmp(&float(*other))
    }
}

impl Mul for Float {
    type Output = Self;
    #[inline(always)]
    fn mul(self, rhs: Self) -> Self {
        Float(self.0 * rhs.0)
    }
}

impl MulAssign for Float {
    #[inline(always)]
    fn mul_assign(&mut self, rhs: Self) {
        self.0 *= rhs.0;
    }
}

impl Div for Float {
    type Output = Self;
    #[inline(always)]
    fn div(self, rhs: Self) -> Self {
        Float(self.0 / rhs.0)
    }
}

impl DivAssign for Float {
    #[inline(always)]
    fn div_assign(&mut self, rhs: Self) {
        self.0 /= rhs.0;
    }
}

impl Rem for Float {
    type Output = Self;
    #[inline(always)]
    fn rem(self, rhs: Self) -> Self {
        Float(self.0 % rhs.0)
    }
}

impl RemAssign for Float {
    #[inline(always)]
    fn rem_assign(&mut self, rhs: Self) {
        self.0 %= rhs.0;
    }
}

impl Neg for Float {
    type Output = Self;
    #[inline(always)]
    fn neg(self) -> Self {
        float(-self.0.raw())
    }
}

impl NumFloat for Float {
    #[inline(always)]
    fn nan() -> Self {
        float(FloatPrim::nan())
    }

    #[inline(always)]
    fn infinity() -> Self {
        float(FloatPrim::infinity())
    }

    #[inline(always)]
    fn neg_infinity() -> Self {
        float(FloatPrim::neg_infinity())
    }

    #[inline(always)]
    fn neg_zero() -> Self {
        float(FloatPrim::zero())
    }

    #[inline(always)]
    fn min_value() -> Self {
        float(<FloatPrim as Bounded>::min_value())
    }

    #[inline(always)]
    fn min_positive_value() -> Self {
        float(FloatPrim::min_positive_value())
    }

    #[inline(always)]
    fn max_value() -> Self {
        float(<FloatPrim as Bounded>::max_value())
    }

    #[inline(always)]
    fn is_nan(self) -> bool {
        self.0.is_nan()
    }

    #[inline(always)]
    fn is_infinite(self) -> bool {
        self.0.is_infinite()
    }

    #[inline(always)]
    fn is_finite(self) -> bool {
        self.0.is_finite()
    }

    #[inline(always)]
    fn is_normal(self) -> bool {
        self.0.is_normal()
    }

    #[inline(always)]
    fn classify(self) -> FpCategory {
        self.0.classify()
    }

    #[inline(always)]
    fn floor(self) -> Self {
        Float(self.0.floor())
    }

    #[inline(always)]
    fn ceil(self) -> Self {
        Float(self.0.ceil())
    }

    #[inline(always)]
    fn round(self) -> Self {
        Float(self.0.round())
    }

    #[inline(always)]
    fn trunc(self) -> Self {
        Float(self.0.trunc())
    }

    #[inline(always)]
    fn fract(self) -> Self {
        Float(self.0.fract())
    }

    #[inline(always)]
    fn abs(self) -> Self {
        Float(self.0.abs())
    }

    #[inline(always)]
    fn signum(self) -> Self {
        Float(self.0.signum())
    }

    #[inline(always)]
    fn is_sign_positive(self) -> bool {
        self.0.is_sign_positive()
    }

    #[inline(always)]
    fn is_sign_negative(self) -> bool {
        self.0.is_sign_negative()
    }

    #[inline(always)]
    fn mul_add(self, a: Self, b: Self) -> Self {
        Float(self.0.mul_add(a.0, b.0))
    }

    #[inline(always)]
    fn recip(self) -> Self {
        Float(self.0.recip())
    }

    #[inline(always)]
    fn powi(self, n: i32) -> Self {
        Float(self.0.powi(n))
    }

    #[inline(always)]
    fn powf(self, n: Self) -> Self {
        Float(self.0.powf(n.0))
    }

    #[inline(always)]
    fn sqrt(self) -> Self {
        Float(self.0.sqrt())
    }

    #[inline(always)]
    fn exp(self) -> Self {
        Float(self.0.exp())
    }

    #[inline(always)]
    fn exp2(self) -> Self {
        Float(self.0.exp2())
    }

    #[inline(always)]
    fn ln(self) -> Self {
        Float(self.0.ln())
    }

    #[inline(always)]
    fn log(self, base: Self) -> Self {
        Float(self.0.log(base.0))
    }

    #[inline(always)]
    fn log2(self) -> Self {
        Float(self.0.log2())
    }

    #[inline(always)]
    fn log10(self) -> Self {
        Float(self.0.log10())
    }

    #[inline(always)]
    fn max(self, other: Self) -> Self {
        Float(self.0.max(other.0))
    }

    #[inline(always)]
    fn min(self, other: Self) -> Self {
        Float(self.0.min(other.0))
    }

    #[inline(always)]
    fn abs_sub(self, other: Self) -> Self {
        Float(self.0.abs_sub(other.0))
    }

    #[inline(always)]
    fn cbrt(self) -> Self {
        Float(self.0.cbrt())
    }

    #[inline(always)]
    fn hypot(self, other: Self) -> Self {
        Float(self.0.hypot(other.0))
    }

    #[inline(always)]
    fn sin(self) -> Self {
        Float(self.0.sin())
    }

    #[inline(always)]
    fn cos(self) -> Self {
        Float(self.0.cos())
    }

    #[inline(always)]
    fn tan(self) -> Self {
        Float(self.0.tan())
    }

    #[inline(always)]
    fn asin(self) -> Self {
        Float(self.0.asin())
    }

    #[inline(always)]
    fn acos(self) -> Self {
        Float(self.0.acos())
    }

    #[inline(always)]
    fn atan(self) -> Self {
        Float(self.0.atan())
    }

    #[inline(always)]
    fn atan2(self, other: Self) -> Self {
        Float(self.0.atan2(other.0))
    }

    #[inline(always)]
    fn sin_cos(self) -> (Self, Self) {
        let (a, b) = self.0.sin_cos();
        (Float(a), Float(b))
    }

    #[inline(always)]
    fn exp_m1(self) -> Self {
        Float(self.0.exp_m1())
    }

    #[inline(always)]
    fn ln_1p(self) -> Self {
        Float(self.0.ln_1p())
    }

    #[inline(always)]
    fn sinh(self) -> Self {
        Float(self.0.sinh())
    }

    #[inline(always)]
    fn cosh(self) -> Self {
        Float(self.0.cosh())
    }

    #[inline(always)]
    fn tanh(self) -> Self {
        Float(self.0.tanh())
    }

    #[inline(always)]
    fn asinh(self) -> Self {
        Float(self.0.asinh())
    }

    #[inline(always)]
    fn acosh(self) -> Self {
        Float(self.0.acosh())
    }

    #[inline(always)]
    fn atanh(self) -> Self {
      Float(self.0.atanh())
    }

    #[inline(always)]
    fn integer_decode(self) -> (u64, i16, i8) {
        self.0.integer_decode()
    }
}
