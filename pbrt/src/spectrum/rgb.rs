use itertools::izip;
use super::*;

#[derive(Copy, Clone, Debug, Eq, PartialEq)]
pub struct RgbSpectrum {
    c: [Float; 3],
}

impl Spectrum for RgbSpectrum {
    fn new(value: impl Into<Float>) -> Self {
        Self {
            c: [value.into(); 3],
        }
    }

    fn from_sampled(samples: &[SampledSpectrumData]) -> Self {
        let samples: Vec<_> = if !is_sorted(samples) {
            let mut samples: Vec<_> = samples.to_vec();
            samples.sort_unstable();
            samples
        } else {
            samples.to_vec()
        };

        let mut xyz = [float(0.0); 3];

        for (lambda, x, y, z) in izip!(CIE_LAMBDA.iter(), CIE_X.iter(), CIE_Y.iter(), CIE_Z.iter()) {
            let val = interpolate_spectrum_samples(&samples, float(*lambda));

            xyz[0] += val * float(*x);
            xyz[1] += val * float(*y);
            xyz[2] += val * float(*z);
        }

        let scale = float(CIE_LAMBDA.last().unwrap() - CIE_LAMBDA.first().unwrap()) / float(CIE_Y_INTEGRAL * N_CIE_SAMPLES as f32);

        xyz[0] *= scale;
        xyz[1] *= scale;
        xyz[2] *= scale;

        Self::from_xyz(xyz, SpectrumType::Illumination)
    }

    fn from_rgb(rgb: [Float; 3], _: SpectrumType) -> Self {
        Self {
            c: rgb,
        }
    }

    fn from_xyz(xyz: [Float; 3], _: SpectrumType) -> Self {
        Self {
            c: xyz_to_rgb(xyz),
        }
    }

    #[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
    fn y(&self) -> Float {
        let y = [float(0.212671), float(0.715160), float(0.072169)];
        y[0] * self.c[0] + y[1] * self.c[1] + y[2] * self.c[2]
    }

    fn to_xyz(&self) -> [Float; 3] {
        rgb_to_xyz(self.c)
    }

    fn to_rgb(&self) -> [Float; 3] {
        self.c
    }

    fn to_rgb_spectrum(&self) -> RgbSpectrum {
        *self
    }
}

spectrum_impl!(RgbSpectrum);
