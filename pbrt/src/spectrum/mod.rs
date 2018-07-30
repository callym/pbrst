use std::cmp::Ordering;
use std::ops::{ Deref, DerefMut, Mul, Add };
use std::slice::{ Iter, IterMut };
use num;
use prelude::*;

#[macro_use] mod macros;

mod rgb;
pub use self::rgb::RgbSpectrum;

mod rgb_consts;
pub use self::rgb_consts::*;

mod sampled;
pub use self::sampled::SampledSpectrum;

pub mod utils;
pub use self::utils::*;

mod xyz_consts;
pub use self::xyz_consts::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct SampledSpectrumData {
    pub lambda: Float,
    pub value: Float,
}

impl Ord for SampledSpectrumData {
    fn cmp(&self, other: &Self) -> Ordering {
        self.lambda.cmp(&other.lambda)
    }
}

impl PartialOrd for SampledSpectrumData {
    fn partial_cmp(&self, other: &Self) -> Option<Ordering> {
        self.lambda.partial_cmp(&other.lambda)
    }
}

pub trait Spectrum: Deref<Target = [Float]> + DerefMut {
    fn new(value: impl Into<Float>) -> Self;

    fn from_sampled(samples: &[SampledSpectrumData]) -> Self;

    fn from_rgb(rgb: [Float; 3], ty: SpectrumType) -> Self;

    fn from_xyz(xyz: [Float; 3], ty: SpectrumType) -> Self;

    fn y(&self) -> Float;

    fn to_xyz(&self) -> [Float; 3];

    fn to_rgb(&self) -> [Float; 3] {
        let xyz = self.to_xyz();
        xyz_to_rgb(xyz)
    }

    fn to_rgb_spectrum(&self) -> RgbSpectrum;

    fn is_black(&self) -> bool {
        for c in self.deref() {
            if *c != 0.0 {
                return false;
            }
        }

        true
    }

    fn iter(&self) -> SpectrumIter {
        SpectrumIter(self.deref().into_iter())
    }

    fn iter_mut(&mut self) -> SpectrumIterMut {
        SpectrumIterMut(self.deref_mut().into_iter())
    }

    fn sqrt(mut self) -> Self where Self: Sized {
        for c1 in self.iter_mut() {
            *c1 = c1.sqrt();
        }
        self
    }

    fn powi(mut self, n: i32) -> Self where Self: Sized {
        for c1 in self.iter_mut() {
            *c1 = c1.powi(n);
        }
        self
    }

    fn powf(mut self, n: Float) -> Self where Self: Sized {
        for c1 in self.iter_mut() {
            *c1 = c1.powf(n);
        }
        self
    }

    fn exp(mut self) -> Self where Self: Sized {
        for c1 in self.iter_mut() {
            *c1 = c1.exp();
        }
        self
    }

    fn lerp(self, other: Self, t: Float) -> Self where
        Self: Sized + Mul<Float, Output = Self> + Add<Output = Self>
    {
        self * (float(1.0) - t) + other * t
    }

    fn clamp(mut self, low: Option<Float>, high: Option<Float>) -> Self where Self: Sized {
        let low = low.unwrap_or_else(|| float(0.0));
        let high = high.unwrap_or_else(Float::infinity);

        for c1 in self.iter_mut() {
            *c1 = num::clamp(*c1, low, high);
        }
        self
    }
}

pub struct SpectrumIter<'a>(Iter<'a, Float>);

impl<'a> Iterator for SpectrumIter<'a> {
    type Item = &'a Float;

    fn next(&mut self) -> Option<&'a Float> {
        self.0.next()
    }
}

pub struct SpectrumIterMut<'a>(IterMut<'a, Float>);

impl<'a> Iterator for SpectrumIterMut<'a> {
    type Item = &'a mut Float;

    fn next(&mut self) -> Option<&'a mut Float> {
        self.0.next()
    }
}
