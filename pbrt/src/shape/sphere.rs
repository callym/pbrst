use std::cmp::{ min, max };

use cgmath::prelude::*;
use cgmath::{ Rad, Deg };
use num;
use crate::prelude::*;
use crate::math::*;
use crate::interaction::SurfaceInteraction;

use super::{ Shape, ShapeData };

#[derive(Clone, Debug)]
pub struct Sphere {
    radius: Float,
    z_min: Float,
    z_max: Float,
    theta_min: Float,
    theta_max: Float,
    phi_max: Float,
    shape_data: ShapeData,
}

impl Sphere {
    pub fn new(radius: Float, z_min: Float, z_max: Float, phi_max: impl Into<Rad<Float>>, data: ShapeData) -> Self {
        let z_min = num::clamp(min(z_min, z_max), -radius, radius);
        let z_max = num::clamp(max(z_min, z_max), -radius, radius);

        let theta_min = num::clamp(z_min / radius, float(-1.0), float(1.0)).acos();
        let theta_max = num::clamp(z_max / radius, float(-1.0), float(1.0)).acos();

        let phi_max: Float = num::clamp(phi_max.into(), Deg(float(0)).into(), Deg(float(360)).into()).0;

        Self {
            radius,
            z_min,
            z_max,
            theta_min,
            theta_max,
            phi_max,
            shape_data: data,
        }
    }
}

impl Shape for Sphere {
    fn data(&self) -> &ShapeData { &self.shape_data }

    fn object_bounds(&self) -> Bounds3f {
        // todo - this can be tighter
        Bounds3f::new(
            Point3f::new(-self.radius, -self.radius, self.z_min),
            Point3f::new( self.radius,  self.radius, self.z_max),
        )
    }

    #[allow(non_snake_case)]
    #[cfg_attr(feature = "cargo-clippy", allow(many_single_char_names))]
    fn intersect(&'a self, ray: &Ray, _: bool) -> Option<(Float, SurfaceInteraction<'a>)> {
        let mut phi = float(0.0);

        let (ray, o_err, d_err) = self.shape_data.world_to_object.transform_ray_with_error(*ray);

        let dx = efloat(ray.direction.x, d_err.x);
        let dy = efloat(ray.direction.y, d_err.y);
        let dz = efloat(ray.direction.z, d_err.z);
        let ox = efloat(ray.origin.x, o_err.x);
        let oy = efloat(ray.origin.y, o_err.y);
        let oz = efloat(ray.origin.z, o_err.z);

        let a = dx.powi(2) + dy.powi(2) + dz.powi(2);
        let b = efloat0(2.0) * (dx * ox + dy * oy + dz * oz);
        let c = ox.powi(2) + oy.powi(2) + oz.powi(2) - efloat0(self.radius).powi(2);

        let (t0, t1) = match quadratic(a, b, c) {
            Some((t0, t1)) => (t0, t1),
            None => return None,
        };

        let max = ray.max.unwrap_or_else(Float::infinity);

        if t0.upper_bound() > max || t1.lower_bound() <= 0.0 {
            return None;
        }

        let mut shape_hit = t0;
        if shape_hit.lower_bound() <= 0.0 {
            shape_hit = t1;

            if shape_hit.upper_bound() > max {
                return None;
            }
        }

        let mut p_hit = ray.position(*shape_hit);

        p_hit *= self.radius / Point3f::zero().distance(p_hit);

        if p_hit.x == 0.0 && p_hit.y == 0.0 {
            p_hit.x = float(1e-5) * self.radius;
        }

        let mut test = |p_hit: Point3f| {
            phi = Float::atan2(p_hit.y, p_hit.x);
            if phi < 0.0 {
                phi += float(2.0) * Float::pi();
            }

            (self.z_min > -self.radius && p_hit.z < self.z_min) ||
            (self.z_max <  self.radius && p_hit.z > self.z_max) ||
            phi > self.phi_max
        };

        if test(p_hit) {
            if shape_hit == t1 {
                return None;
            }

            if t1.upper_bound() > max {
                return None;
            }

            shape_hit = t1;
            let mut p_hit = ray.position(*shape_hit);

            // refine sphere intersection point
            p_hit *= self.radius / Point3f::zero().distance(p_hit);

            if p_hit.x == 0.0 && p_hit.y == 0.0 {
                p_hit.x = float(1e-5) * self.radius;
            }

            if test(p_hit) {
                return None;
            }
        }

        let u = phi / self.phi_max;
        let theta = num::clamp(p_hit.z / self.radius, float(-1.0), float(1.0)).acos();
        let v = (theta - self.theta_min) / (self.theta_max - self.theta_min);

        let z_radius = (p_hit.x.powi(2) + p_hit.y.powi(2)).sqrt();
        let inv_z_radius = float(1.0) / z_radius;
        let cos_phi = p_hit.x * inv_z_radius;
        let sin_phi = p_hit.y * inv_z_radius;

        let dpdu = Vector3f::new(
            -self.phi_max * p_hit.y,
            self.phi_max * p_hit.x,
            float(0.0),
        );

        let dpdv = Vector3f::new(
            p_hit.z * cos_phi,
            p_hit.z * sin_phi,
            -self.radius * theta.sin(),
        ) * (self.theta_max - self.theta_min);

        let d2pduu = Vector3f::new(p_hit.x, p_hit.y, float(0.0)) * -self.phi_max.powi(2);
        let d2pduv = Vector3f::new(-sin_phi, cos_phi, float(0.0)) *
            (self.theta_max - self.theta_min) * p_hit.z * self.phi_max;
        let d2pdvv = p_hit.into_vector() * -(self.theta_max - self.theta_min).powi(2);

        let E = dpdu.dot(dpdu);
        let F = dpdu.dot(dpdv);
        let G = dpdv.dot(dpdv);
        let N = dpdu.cross(dpdv).normalize();
        let e = N.dot(d2pduu);
        let f = N.dot(d2pduv);
        let g = N.dot(d2pdvv);

        let invEGF2 = float(1.0) / (E * G - F.powi(2));
        let dndu: Normal = (dpdu * (f * F - e * G) * invEGF2 +
                            dpdv * (e * F - f * E) * invEGF2).into();
        let dndv: Normal = (dpdu * (g * F - f * G) * invEGF2 +
                            dpdv * (f * F - g * E) * invEGF2).into();

        // compute error bounds
        let p_err = p_hit.abs().into_vector() * gammaf(5);

        let interaction = SurfaceInteraction::new(
            p_hit,
            p_err,
            Point2f::new(u, v),
            -ray.direction,
            dpdu,
            dpdv,
            dndu,
            dndv,
            ray.time,
            Some(self),
            None,
        );
        let interaction = self.shape_data.object_to_world.transform_surface_interaction(&interaction);

        Some((*shape_hit, interaction))
    }

    fn area(&self) -> Float {
        self.phi_max * self.radius * (self.z_max - self.z_min)
    }
}
