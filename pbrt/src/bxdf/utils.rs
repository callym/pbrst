use std::cmp::max;
use cgmath::prelude::*;
use num;
use crate::prelude::*;

#[inline(always)]
pub fn cos_theta(w: Vector3f) -> Float {
    w.z
}

#[inline(always)]
pub fn cos_2_theta(w: Vector3f) -> Float {
    w.z.powi(2)
}

#[inline(always)]
pub fn cos_theta_abs(w: Vector3f) -> Float {
    w.z.abs()
}

#[inline(always)]
pub fn sin_2_theta(w: Vector3f) -> Float {
    max(float(0.0), float(1.0) - cos_2_theta(w))
}

#[inline(always)]
pub fn sin_theta(w: Vector3f) -> Float {
    sin_2_theta(w).sqrt()
}

#[inline(always)]
pub fn tan_theta(w: Vector3f) -> Float {
    sin_theta(w) / cos_theta(w)
}

#[inline(always)]
pub fn tan_2_theta(w: Vector3f) -> Float {
    sin_2_theta(w) / cos_2_theta(w)
}

#[inline(always)]
pub fn cos_phi(w: Vector3f) -> Float {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        float(1.0)
    } else {
        num::clamp(w.x / sin_theta, float(-1.0), float(1.0))
    }
}

#[inline(always)]
pub fn sin_phi(w: Vector3f) -> Float {
    let sin_theta = sin_theta(w);
    if sin_theta == 0.0 {
        float(1.0)
    } else {
        num::clamp(w.y / sin_theta, float(-1.0), float(1.0))
    }
}

#[inline(always)]
pub fn cos_2_phi(w: Vector3f) -> Float {
    cos_phi(w).powi(2)
}

#[inline(always)]
pub fn sin_2_phi(w: Vector3f) -> Float {
    sin_phi(w).powi(2)
}

#[inline(always)]
pub fn cos_delta_phi(wa: Vector3f, wb: Vector3f) -> Float {
    num::clamp(
        (wa.x * wb.x + wa.y * wb.y) /
        ((wa.x.powi(2) + wa.y.powi(2)) * (wa.x.powi(2) + wa.y.powi(2))).sqrt(),
        float(-1.0),
        float(1.0)
    )
}

#[inline(always)]
pub fn reflect(wo: Vector3f, n: Vector3f) -> Vector3f {
    let Vector3f { x, y, z } = -wo.add_element_wise(float(2.0)) * wo.dot(n);
    Vector3f::new(x * n.x, y * n.y, z * n.z)
}

/// This function calculates the refracted `Vector3f`, and returns `None` if
/// it is totally internally refracted.
#[inline(always)]
pub fn refract(wi: Vector3f, n: Normal, eta: Float) -> Option<Vector3f> {
    let cos_theta_i = (*n).dot(wi);
    let sin_2_theta_i = max(float(0.0), float(1.0) - cos_theta_i.powi(2));
    let sin_2_theta_t = eta.powi(2) * sin_2_theta_i;

    if sin_2_theta_t >= 1.0 {
        return None;
    }

    let cos_theta_t = (float(1.0) - sin_2_theta_t).sqrt();

    let Vector3f { x, y, z } = (-wi * eta).add_element_wise(eta * cos_theta_i - cos_theta_t);

    Some(Vector3f::new(x * n.x, y * n.y, z * n.z))
}

#[inline(always)]
pub fn same_hemisphere(w: Vector3f, wp: Vector3f) -> bool {
    w.z * wp.z > 0.0
}
