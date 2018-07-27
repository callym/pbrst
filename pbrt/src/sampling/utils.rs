use std::cmp::max;
use prelude::*;

pub fn uniform_sample_hemisphere(u: Point2f) -> Vector3f {
    let z = u[0];
    let r = max(float(0.0), float(1.0) - z.powi(2)).sqrt();
    let phi = float(2.0) * Float::pi() * u[1];

    Vector3f::new(r * phi.cos(), r * phi.sin(), z)
}

#[inline(always)]
pub fn uniform_hemisphere_pdf() -> Float {
    Float::inv_2_pi()
}

pub fn uniform_sample_sphere(u: Point2f) -> Vector3f {
    let z = float(1.0) - float(2.0) * u[0];
    let r = max(float(0.0), float(1.0) - z.powi(2)).sqrt();
    let phi = float(2.0) * Float::pi() * u[1];

    Vector3f::new(r * phi.cos(), r * phi.sin(), z)
}

#[inline(always)]
pub fn uniform_sphere_pdf() -> Float {
    Float::inv_4_pi()
}

pub fn uniform_sample_disk(u: Point2f) -> Point2f {
    let r = u[0].sqrt();
    let theta = float(2.0) * Float::pi() * u[1];

    Point2f::new(r * theta.cos(), r * theta.sin())
}

pub fn concentric_sample_disk(u: Point2f) -> Point2f {
    // map the uniform random numbers provided in `u` to [-1, 1]^2
    let u_offset = u * float(2.0) - Vector2f::new(float(1.0), float(1.0));

    // handles degeneracy at the origin
    if u_offset.x == 0.0 && u_offset.y == 0.0 {
        return Point2f::zero()
    }

    // apply concentric mapping to point
    let (r, theta) = if u_offset.x.abs() > u_offset.y.abs() {
        (
            u_offset.x,
            Float::frac_pi_4() * (u_offset.y / u_offset.x),
        )
    } else {
        (
            u_offset.y,
            Float::frac_pi_2() - Float::frac_pi_4() * (u_offset.x / u_offset.y)
        )
    };

    Point2f::new(theta.cos(), theta.sin()) * r
}

#[inline(always)]
pub fn cosine_sample_hemisphere(u: Point2f) -> Vector3f {
    let d = concentric_sample_disk(u);
    let z = max(float(0.0), float(1.0) - d.x.powi(2) - d.y.powi(2)).sqrt();

    Vector3f::new(d.x, d.y, z)
}

#[inline(always)]
pub fn cosine_hemisphere_pdf(cos_theta: Float) -> Float {
    cos_theta * Float::frac_1_pi()
}

pub fn uniform_sample_cone(u: Point2f, cos_theta_max: Float) -> Vector3f {
    let cos_theta = (float(1.0) - u[0]) + u[0] * cos_theta_max;
    let sin_theta = (float(1.0) - cos_theta.powi(2)).sqrt();
    let phi = u[1] * float(2.0) * Float::pi();

    Vector3f::new(phi.cos() * sin_theta, phi.sin() * sin_theta, cos_theta)
}

#[inline(always)]
pub fn uniform_cone_pdf(cos_theta_max: Float) -> Float {
    float(1.0) / (float(2.0) * Float::pi() * (float(1.0) - cos_theta_max))
}

pub fn uniform_sample_triangle(u: Point2f) -> Point2f {
    let su0 = u[0].sqrt();

    Point2f::new(float(1.0) - su0, u[1] * su0)
}

#[inline(always)]
pub fn uniform_triangle_pdf(area: Float) -> Float {
    float(1.0) / area
}

#[inline(always)]
pub fn balance_heuristic(nf: u8, f_pdf: Float, ng: u8, g_pdf: Float) -> Float {
    (float(nf) * f_pdf) / (float(nf) * f_pdf + float(ng) * g_pdf)
}

#[inline(always)]
pub fn power_heuristic(nf: u8, f_pdf: Float, ng: u8, g_pdf: Float) -> Float {
    let f = float(nf) * f_pdf;
    let g = float(ng) * g_pdf;

    f.powi(2) / (f.powi(2) + g.powi(2))
}
