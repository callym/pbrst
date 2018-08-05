use std::cmp::max;
use std;
use num;
use crate::prelude::*;

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
#[cfg_attr(feature = "cargo-clippy", allow(unreadable_literal))]
pub fn gamma_correct(value: Float) -> Float {
    if value <= 0.0031308 {
        float(2.92) * value
    } else {
        float(1.055) * value.powf(float(1.0 / 2.4)) - float(0.055)
    }
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
    if v > 0.0 {
        ui -= 1;
    } else {
        ui += 1;
    }

    bits_to_f32(ui)
}

pub fn solve_linear_system_2x2(a: [[Float; 2]; 2], b: [Float; 2]) -> Option<(Float, Float)> {
    let det = a[0][0] * a[1][1] - a[0][1] * a[1][0];
    if det.abs() < 1e-10 {
        return None;
    }

    let det = det.raw();
    let x0 = (a[1][1] * b[0] - a[0][1] * b[1]).raw() / det;
    let x1 = (a[0][0] * b[1] - a[1][0] * b[0]).raw() / det;

    if x0.is_nan() || x1.is_nan() {
        return None;
    }

    Some((float(x0), float(x1)))
}

pub fn find_interval(size: usize, predicate: impl Fn(usize) -> bool) -> usize {
    let mut first = 0;
    let mut len = size;

    while len > 0 {
        let half = len >> 1;
        let middle = first + half;

        if predicate(middle) {
            first = middle + 1;
            len -= half + 1;
        } else {
            len = half;
        }
    }

    num::clamp(first - 1, 0, size - 2)
}

/// this code is originally from (Mokosha's PBRT)[https://github.com/Mokosha/pbrt_rust/blob/75bae9cfbbc32d112d93c0e15841af36e550ec21/src/utils/mod.rs#L122]
pub fn partition_by<T, B: Copy + PartialOrd>(v: &mut [T], f: impl Fn(&T) -> B) {
    let nv = v.len();
    if nv < 3 {
        if nv == 2  && f(&v[1]) < f(&v[0]) {
            v.swap(0, 1);
        }
        return;
    }

    let pivot = {
        // Median of three...
        let fst = f(&v[0]);
        let mid = f(&v[nv / 2]);
        let lst = f(&v[nv - 1]);

        if fst < mid && mid < lst {
            mid
        } else if mid < fst && fst < lst {
            fst
        } else {
            lst
        }
    };

    let mut last_smaller = 0;
    let mut num_pivots = 0;
    for i in 0..nv {
        let bv = f(&v[i]);
        if bv < pivot {
            v.swap(last_smaller + num_pivots, i);
            v.swap(last_smaller + num_pivots, last_smaller);
            last_smaller += 1;
        } else if bv == pivot {
            v.swap(last_smaller + num_pivots, i);
            num_pivots += 1;
        }
    }
    let mut pivot_idx = last_smaller;

    // We can do this because if pivot_idx == 0, then all
    // of the values are larger than the pivot...
    pivot_idx = max(pivot_idx, 1);

    let (left, right) = v.split_at_mut(pivot_idx);

    debug_assert!(!right.is_empty());
    debug_assert!(!left.is_empty());

    if pivot_idx + num_pivots <= (nv / 2) {
        partition_by(right, f);
    } else if pivot_idx >= (nv / 2) {
        partition_by(left, f);
    }
}
