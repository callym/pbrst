use std::fmt;
use prelude::*;

use super::*;
use super::Spectrum;

pub const LAMBDA_START: u32 = 400;
pub const LAMBDA_END: u32 = 700;

pub const NUM_SAMPLES: usize = 60;

#[derive(Copy, Clone)]
pub struct SampledSpectrum {
    pub c: [Float; NUM_SAMPLES],
}

impl Spectrum for SampledSpectrum {
    fn new(value: impl Into<Float>) -> Self {
        Self {
            c: [value.into(); NUM_SAMPLES],
        }
    }

    fn from_sampled(samples: &[SampledSpectrumData]) -> Self {
        let samples: Vec<_> = if !is_sorted(samples) {
            let mut samples: Vec<_> = samples.iter().map(|i| i.clone()).collect();
            samples.sort_unstable();
            samples
        } else {
            samples.iter().map(|i| i.clone()).collect()
        };

        let mut r = Self::new(0.0);
        for (i, c) in r.iter_mut().enumerate() {
            // compute average of given SPD over 'ith' sample's range
            let lambda_start = float(LAMBDA_START as f32)
                .lerp(float(LAMBDA_END as f32), float(i as f32 / NUM_SAMPLES as f32));
            let lambda_end = float(LAMBDA_START as f32)
                .lerp(float(LAMBDA_END as f32), float((i + 1) as f32 / NUM_SAMPLES as f32));
            *c = average_samples(&samples[..], lambda_start, lambda_end);
        }
        r
    }

    fn from_rgb(rgb: [Float; 3], ty: SpectrumType) -> Self {
        let mut r = SampledSpectrum::new(0.0);

        let rgb_const = match ty {
            SpectrumType::Reflectance => &*RGB_REFL,
            SpectrumType::Illumination => &*RGB_ILLUM,
        };

        if rgb[0] <= rgb[1] && rgb[0] <= rgb[2] {
            // red smallest
            r += rgb_const.white * rgb[0];
            if rgb[1] <= rgb[2] {
                // green 2nd smallest
                r += rgb_const.cyan * (rgb[1] - rgb[0]);
                r += rgb_const.blue * (rgb[2] - rgb[1]);
            } else {
                // blue 2nd smallest
                r += rgb_const.cyan * (rgb[2] - rgb[0]);
                r += rgb_const.green * (rgb[1] - rgb[2]);
            }
        } else if rgb[1] <= rgb[0] && rgb[1] <= rgb[2] {
            // green smallest
            r += rgb_const.white * rgb[1];
            if rgb[0] <= rgb[2] {
                // red 2nd smallest
                r += rgb_const.magenta * (rgb[0] - rgb[1]);
                r += rgb_const.blue * (rgb[2] - rgb[0]);
            } else {
                // blue 2nd smallest
                r += rgb_const.magenta * (rgb[2] - rgb[0]);
                r += rgb_const.red * (rgb[1] - rgb[2]);
            }
        } else {
            // blue smallest
            r += rgb_const.white * rgb[2];
            if rgb[1] <= rgb[2] {
                // red 2nd smallest
                r += rgb_const.yellow * (rgb[0] - rgb[2]);
                r += rgb_const.green * (rgb[1] - rgb[0]);
            } else {
                // green 2nd smallest
                r += rgb_const.yellow * (rgb[1] - rgb[2]);
                r += rgb_const.red * (rgb[0] - rgb[1]);
            }
        }

        r.clamp(None, None)
    }

    fn from_xyz(xyz: [Float; 3], ty: SpectrumType) -> Self {
        let rgb = xyz_to_rgb(xyz);
        Self::from_rgb(rgb, ty)
    }

    fn y(&self) -> Float {
        let mut yy = float(0.0);

        for (c, y) in izip!(self.iter(), XYZ.y.iter()) {
            yy += *y * *c;
        }

        let scale = float(LAMBDA_END - LAMBDA_START) /
            float(CIE_Y_INTEGRAL * NUM_SAMPLES as f32);

        yy * scale
    }

    fn to_xyz(&self) -> [Float; 3] {
        let mut xyz = [float(0.0); 3];

        let XyzSampledSpectrums { x, y, z } = &*XYZ;

        for (c, x, y, z) in izip!(self.iter(), x.iter(), y.iter(), z.iter()) {
            xyz[0] += *x * *c;
            xyz[1] += *y * *x;
            xyz[2] += *z * *c;
        }

        let scale = float(LAMBDA_END - LAMBDA_START) /
            float(CIE_Y_INTEGRAL * NUM_SAMPLES as f32);

        xyz[0] *= scale;
        xyz[1] *= scale;
        xyz[2] *= scale;

        xyz
    }

    fn to_rgb_spectrum(&self) -> RgbSpectrum {
        unimplemented!()
    }
}

impl fmt::Debug for SampledSpectrum {
    fn fmt(&self, f: &mut fmt::Formatter) -> fmt::Result {
        write!(f, "SampledSpectrum {{ ... }}")
    }
}

impl PartialEq for SampledSpectrum {
    fn eq(&self, other: &Self) -> bool {
        **self == **other
    }
}

spectrum_impl!(SampledSpectrum);
