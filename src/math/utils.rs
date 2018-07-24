use std;
use prelude::*;

pub const SHADOW_EPSILON: f32 = 0.0001;

pub const MACHINE_EPSILON: f32 = std::f32::EPSILON * 0.5;

#[inline(always)]
pub const fn gamma(n: i32) -> f32 {
    (n as f32 * MACHINE_EPSILON) / (1.0 - n as f32 * MACHINE_EPSILON)
}

#[inline(always)]
pub fn gammaf(n: i32) -> Float {
    float(gamma(n))
}

#[inline(always)]
pub fn f32_to_bits(f: impl Into<f32>) -> u32 {
    let f: f32 = f.into();
    f.to_bits()
}

#[inline(always)]
pub fn bits_to_f32(u: u32) -> f32 {
    f32::from_bits(u)
}

#[inline(always)]
pub fn next_float_up_f(v: impl Into<f32>) -> Float {
    float(next_float_up(v))
}

#[inline(always)]
pub fn next_float_down_f(v: impl Into<f32>) -> Float {
    float(next_float_down(v))
}

#[inline(always)]
pub fn next_float_up(v: impl Into<f32>) -> f32 {
    let mut v: f32 = v.into();

    if v.is_infinite() && v > 0.0 {
        return v;
    }

    if v == -0.0 {
        v = 0.0;
    }

    let mut ui = f32_to_bits(v);
    if v >= 0.0 {
        ui += 1;
    } else {
        ui -= 1;
    }

    bits_to_f32(ui)
}

#[inline(always)]
pub fn next_float_down(v: impl Into<f32>) -> f32 {
    let mut v: f32 = v.into();

    if v.is_infinite() && v < 0.0 {
        return v;
    }

    if v == 0.0 {
        v = -0.0;
    }

    let mut ui = f32_to_bits(v);
    if v >= 0.0 {
        ui -= 1;
    } else {
        ui += 1;
    }

    bits_to_f32(ui)
}
