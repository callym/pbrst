#![cfg_attr(feature = "cargo-clippy", allow(unreadable_literal, excessive_precision))]

use std::cmp::{ min, max };
use physical_constants::{
    PLANCK_CONSTANT as h,
    SPEED_OF_LIGHT_IN_VACUUM as c,
    BOLTZMANN_CONSTANT as kb,
    WIEN_WAVELENGTH_DISPLACEMENT_LAW_CONSTANT as b,
};

use super::*;
use super::sampled::*;

pub enum SpectrumType {
    Reflectance,
    Illumination,
}

pub struct XyzSampledSpectrums {
    pub x: SampledSpectrum,
    pub y: SampledSpectrum,
    pub z: SampledSpectrum,
}

pub struct RgbSampledSpectrums {
    pub white: SampledSpectrum,
    pub cyan: SampledSpectrum,
    pub magenta: SampledSpectrum,
    pub yellow: SampledSpectrum,
    pub red: SampledSpectrum,
    pub green: SampledSpectrum,
    pub blue: SampledSpectrum,
}

lazy_static! {
    pub static ref XYZ: XyzSampledSpectrums = {
        let num = float(NUM_SAMPLES as f32);
        let start = float(LAMBDA_START as f32);
        let end = float(LAMBDA_END as f32);

        let mut x = SampledSpectrum::new(0.0);
        let mut y = SampledSpectrum::new(0.0);
        let mut z = SampledSpectrum::new(0.0);

        let cie = |v: [f32; N_CIE_SAMPLES]| CIE_LAMBDA.iter().zip(v.iter())
            .map(|(l, v)| (float(*l), float(*v)))
            .map(|(lambda, value)| SampledSpectrumData { lambda, value })
            .collect::<Vec<_>>();

        let cie_x = cie(CIE_X);
        let cie_y = cie(CIE_Y);
        let cie_z = cie(CIE_Z);

        for i in 0..NUM_SAMPLES {
            let wl_0 = start.lerp(end, float(i as f32) / num);
            let wl_1 = start.lerp(end, float(i as f32 + 1.0) / num);

            x.c[i] = average_samples(&cie_x, wl_0, wl_1);
            y.c[i] = average_samples(&cie_y, wl_0, wl_1);
            z.c[i] = average_samples(&cie_z, wl_0, wl_1);
        }

        XyzSampledSpectrums { x, y, z }
    };

    pub static ref RGB_REFL: RgbSampledSpectrums = {
        let num = float(NUM_SAMPLES as f32);
        let start = float(LAMBDA_START as f32);
        let end = float(LAMBDA_END as f32);

        let mut white = SampledSpectrum::new(0.0);
        let mut cyan = SampledSpectrum::new(0.0);
        let mut magenta = SampledSpectrum::new(0.0);
        let mut yellow = SampledSpectrum::new(0.0);
        let mut red = SampledSpectrum::new(0.0);
        let mut green = SampledSpectrum::new(0.0);
        let mut blue = SampledSpectrum::new(0.0);

        let cie = |v: [f32; RGB_TO_SPECTRUM_SAMPLES]| RGB_TO_SPECTRUM_LAMBDA.iter().zip(v.iter())
            .map(|(l, v)| (float(*l), float(*v)))
            .map(|(lambda, value)| SampledSpectrumData { lambda, value })
            .collect::<Vec<_>>();

        let cie_white = cie(RGB_REFL_TO_SPECTRUM_WHITE);
        let cie_cyan = cie(RGB_REFL_TO_SPECTRUM_CYAN);
        let cie_magenta = cie(RGB_REFL_TO_SPECTRUM_MAGENTA);
        let cie_yellow = cie(RGB_REFL_TO_SPECTRUM_YELLOW);
        let cie_red = cie(RGB_REFL_TO_SPECTRUM_RED);
        let cie_green = cie(RGB_REFL_TO_SPECTRUM_GREEN);
        let cie_blue = cie(RGB_REFL_TO_SPECTRUM_BLUE);

        for i in 0..NUM_SAMPLES {
            let wl_0 = start.lerp(end, float(i as f32) / num);
            let wl_1 = start.lerp(end, float(i as f32 + 1.0) / num);

            white.c[i] = average_samples(&cie_white, wl_0, wl_1);
            cyan.c[i] = average_samples(&cie_cyan, wl_0, wl_1);
            magenta.c[i] = average_samples(&cie_magenta, wl_0, wl_1);
            yellow.c[i] = average_samples(&cie_yellow, wl_0, wl_1);
            red.c[i] = average_samples(&cie_red, wl_0, wl_1);
            green.c[i] = average_samples(&cie_green, wl_0, wl_1);
            blue.c[i] = average_samples(&cie_blue, wl_0, wl_1);
        }

        RgbSampledSpectrums { white, cyan, magenta, yellow, red, green, blue }
    };

    pub static ref RGB_ILLUM: RgbSampledSpectrums = {
        let num = float(NUM_SAMPLES as f32);
        let start = float(LAMBDA_START as f32);
        let end = float(LAMBDA_END as f32);

        let mut white = SampledSpectrum::new(0.0);
        let mut cyan = SampledSpectrum::new(0.0);
        let mut magenta = SampledSpectrum::new(0.0);
        let mut yellow = SampledSpectrum::new(0.0);
        let mut red = SampledSpectrum::new(0.0);
        let mut green = SampledSpectrum::new(0.0);
        let mut blue = SampledSpectrum::new(0.0);

        let cie = |v: [f32; RGB_TO_SPECTRUM_SAMPLES]| RGB_TO_SPECTRUM_LAMBDA.iter().zip(v.iter())
            .map(|(l, v)| (float(*l), float(*v)))
            .map(|(lambda, value)| SampledSpectrumData { lambda, value })
            .collect::<Vec<_>>();

        let cie_white = cie(RGB_ILLUM_TO_SPECTRUM_WHITE);
        let cie_cyan = cie(RGB_ILLUM_TO_SPECTRUM_CYAN);
        let cie_magenta = cie(RGB_ILLUM_TO_SPECTRUM_MAGENTA);
        let cie_yellow = cie(RGB_ILLUM_TO_SPECTRUM_YELLOW);
        let cie_red = cie(RGB_ILLUM_TO_SPECTRUM_RED);
        let cie_green = cie(RGB_ILLUM_TO_SPECTRUM_GREEN);
        let cie_blue = cie(RGB_ILLUM_TO_SPECTRUM_BLUE);

        for i in 0..NUM_SAMPLES {
            let wl_0 = start.lerp(end, float(i as f32) / num);
            let wl_1 = start.lerp(end, float(i as f32 + 1.0) / num);

            white.c[i] = average_samples(&cie_white, wl_0, wl_1);
            cyan.c[i] = average_samples(&cie_cyan, wl_0, wl_1);
            magenta.c[i] = average_samples(&cie_magenta, wl_0, wl_1);
            yellow.c[i] = average_samples(&cie_yellow, wl_0, wl_1);
            red.c[i] = average_samples(&cie_red, wl_0, wl_1);
            green.c[i] = average_samples(&cie_green, wl_0, wl_1);
            blue.c[i] = average_samples(&cie_blue, wl_0, wl_1);
        }

        RgbSampledSpectrums { white, cyan, magenta, yellow, red, green, blue }
    };
}

pub fn xyz_to_rgb(xyz: [Float; 3]) -> [Float; 3] {
    let xyz = [xyz[0].raw(), xyz[1].raw(), xyz[2].raw()];
    let mut rgb = [0.0; 3];

    rgb[0] = 3.240479 * xyz[0] - 1.537150 * xyz[1] - 0.498535 * xyz[2];
    rgb[1] = -0.969256 * xyz[0] + 1.875991 * xyz[1] + 0.041556 * xyz[2];
    rgb[2] = 0.055648 * xyz[0] - 0.204043 * xyz[1] + 1.057311 * xyz[2];

    [float(rgb[0]), float(rgb[1]), float(rgb[2])]
}

pub fn rgb_to_xyz(rgb: [Float; 3]) -> [Float; 3] {
    let rgb = [rgb[0].raw(), rgb[1].raw(), rgb[2].raw()];
    let mut xyz = [0.0; 3];

    xyz[0] = 0.412453 * rgb[0] + 0.357580 * rgb[1] + 0.180423 * rgb[2];
    xyz[1] = 0.212671 * rgb[0] + 0.715160 * rgb[1] + 0.072169 * rgb[2];
    xyz[2] = 0.019334 * rgb[0] + 0.119193 * rgb[1] + 0.950227 * rgb[2];

    [float(xyz[0]), float(xyz[1]), float(xyz[2])]
}

pub fn blackbody(lambda: &[Float], temperature: Float) -> Vec<SampledSpectrumData> {
    let temperature = f64::from(temperature.raw());
    lambda.iter()
        .map(|l| {
            let l_o = *l;
            // convert nm to m
            let l = f64::from(l.raw()) * 1e-9;

            let l_5 = l.powi(5);
            let v = (2.0 * h * c.powi(2)) /
                (l_5 * ((h * c) / (l * kb * temperature)).exp() - 1.0);

            SampledSpectrumData {
                lambda: l_o,
                value: float(v),
            }
        })
        .collect()
}

pub fn blackbody_normalized(lambda: &[Float], temperature: Float) -> Vec<SampledSpectrumData> {
    let temperature_r = f64::from(temperature.raw());

    // Wein's displacement law gives λ in m,
    // this converts to nm for `blackbody()`
    let lambda_max = (b / temperature_r) * 1e9;
    let max_l = blackbody(&[float(lambda_max)], temperature);

    assert!(max_l.len() == 1);
    let max_l = max_l[0];

    blackbody(lambda, temperature)
        .into_iter()
        .map(|mut s| {
            s.value /= max_l.value;
            s
        })
        .collect()
}

pub fn interpolate_spectrum_samples(samples: &[SampledSpectrumData], lambda: Float) -> Float {
    assert!(!samples.is_empty());

    let first = samples.first().unwrap();
    let last = samples.last().unwrap();

    if lambda <= first.lambda {
        return first.value;
    }

    if lambda >= last.lambda {
        return last.value;
    }

    let offset = samples.binary_search_by(|p| p.lambda.cmp(&lambda));

    // because in error case it gives us the index where an element
    // could be placed and preserve order, should also be the closest
    // index?
    let offset = match offset {
        Ok(s) => s,
        Err(s) => if s <= samples.len() { s } else { samples.len() },
    };

    let sample = samples[offset];
    let sample_1 = samples.get(offset + 1).unwrap_or(last);

    let t = (lambda - sample.lambda) / (sample_1.lambda - sample.lambda);

    sample.value.lerp(sample_1.value, t)
}

pub fn average_samples(samples: &[SampledSpectrumData], lambda_start: Float, lambda_end: Float) -> Float {
    assert!(!samples.is_empty());

    let first = samples.first().unwrap();
    let last = samples.last().unwrap();

    if samples.len() == 1 {
        return first.value;
    }

    if lambda_end <= samples.first().unwrap().lambda {
        return first.value;
    }

    if lambda_start >= samples.last().unwrap().lambda {
        return last.value;
    }

    let mut sum = float(0.0);

    // add contributions of constant segments before/after samples
    if lambda_start < first.lambda {
        sum += first.value * (first.lambda - lambda_start);
    }

    if lambda_end > last.lambda {
        sum += last.value * (lambda_end - last.lambda);
    }

    // advance to first relevant w/v segment
    let mut i = 0;
    while lambda_start > samples[i + 1].lambda {
        i += 1;
    };

    // loop over segments and add contributions
    let interpolate = |w: Float, i0: &SampledSpectrumData, i1: &SampledSpectrumData| {
        i0.value.lerp(i1.value, (w - i0.lambda) / (i1.lambda - i0.lambda))
    };

    for i in samples.windows(2) {
        if let [i0, i1] = i {
            if lambda_end < i1.lambda {
                break;
            }

            let seg_start = max(lambda_start, i0.lambda);
            let seg_end = min(lambda_end, i1.lambda);
            sum += float(0.5) *
                (interpolate(seg_start, i0, i1) + interpolate(seg_end, i0, i1)) *
                (seg_end - seg_start);
        }
    }

    sum / (lambda_start - lambda_end)
}

pub fn is_sorted(samples: &[SampledSpectrumData]) -> bool {
    for i in samples.windows(2) {
        if let [i1, i2] = i {
            if i1.lambda > i2.lambda {
                return false;
            }
        }
    }
    true
}
