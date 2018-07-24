use num;
use prelude::*;

#[inline(always)]
pub fn spherical_direction(sin: Float, cos: Float, phi: Float) -> Vector3f {
    Vector3f::new(sin * phi.cos() , sin * phi.sin(), cos)
}

#[inline(always)]
pub fn spherical_direction_from_axis(sin: Float, cos: Float, phi: Float, x: Vector3f, y: Vector3f, z: Vector3f) -> Vector3f {
    x * sin * phi.cos() +
    y * sin * phi.sin() +
    z * cos
}

/// Calculates the Spherical Theta (θ) from a **normalised** `Vector3f`.
#[inline(always)]
pub fn spherical_theta(v: Vector3f) -> Float {
    num::clamp(v.z, float(-1.0), float(1.0)).acos()
}

/// Calculates the Spherical Phi (φ) from a **normalised** `Vector3f`.
#[inline(always)]
pub fn spherical_phi(v: Vector3f) -> Float {
    let p = v.y.atan2(v.x);
    if p < 0.0 { p + float(2.0) * Float::pi() } else { p }
}
