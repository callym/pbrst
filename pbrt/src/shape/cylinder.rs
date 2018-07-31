use std::cmp::{ min, max };
use std::sync::Arc;
use cgmath::prelude::*;
use cgmath::{ Rad, Deg, Matrix4 };
use num;
use crate::prelude::*;
use crate::math::*;
use crate::math::Transform;
use crate::interaction::SurfaceInteraction;

use super::{ Shape, ShapeData };

#[derive(Clone, Debug)]
pub struct Cylinder {
    radius: Float,
    z_min: Float,
    z_max: Float,
    phi_max: Float,
    shape_data: ShapeData,
}

impl Cylinder {
    pub fn new(radius: Float, z_min: Float, z_max: Float, phi_max: impl Into<Rad<Float>>, data: ShapeData) -> Self {
        let z_min = min(z_min, z_max);
        let z_max = max(z_min, z_max);

        let phi_max: Float = num::clamp(phi_max.into(), Deg(float(0)).into(), Deg(float(360)).into()).0;

        let transform = data.object_to_world.matrix;
        let to_y = Matrix4::from_angle_x(Deg(float(90.0)));
        let transform = Transform::new(transform * to_y);

        let data = ShapeData::new(Arc::new(transform), data.reverse_orientation);

        Self {
            radius,
            z_min,
            z_max,
            phi_max,
            shape_data: data,
        }
    }
}

impl Shape for Cylinder {
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

        let ox = efloat(ray.origin.x, o_err.x);
        let oy = efloat(ray.origin.y, o_err.y);

        let a = dx.powi(2) + dy.powi(2);
        let b = efloat0(2.0) * (dx * ox + dy * oy);
        let c = ox.powi(2) + oy.powi(2) - efloat0(self.radius).powi(2);

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

        // compute cylinder hit point and phi
        let mut p_hit = ray.position(*shape_hit);
        let hit_rad = (p_hit.x.powi(2) + p_hit.y.powi(2)).sqrt();

        p_hit.x *= self.radius / hit_rad;
        p_hit.y *= self.radius / hit_rad;

        let mut test = |p_hit: Point3f| {
            phi = Float::atan2(p_hit.y, p_hit.x);
            if phi < 0.0 {
                phi += float(2.0) * Float::pi();
            }

            p_hit.z < self.z_min ||
            p_hit.z > self.z_max ||
            phi > self.phi_max
        };

        if test(p_hit) {
            if shape_hit == t1 {
                return None;
            }

            shape_hit = t1;

            if t1.upper_bound() > max {
                return None;
            }

            let mut p_hit = ray.position(*shape_hit);

            // refine cylinder intersection point
            let hit_rad = (p_hit.x.powi(2) + p_hit.y.powi(2)).sqrt();
            p_hit.x *= self.radius / hit_rad;
            p_hit.y *= self.radius / hit_rad;

            if test(p_hit) {
                return None;
            }
        }

        // find parametric representation of cylinder hit
        let u = phi / self.phi_max;
        let v = (p_hit.z - self.z_min) / (self.z_max - self.z_min);

        let dpdu = Vector3f::new(
            -self.phi_max * p_hit.y,
            self.phi_max * p_hit.x,
            float(0.0),
        );

        let dpdv = Vector3f::new(
            float(0.0),
            float(0.0),
            self.z_max - self.z_min,
        );

        let d2pduu = Vector3f::new(p_hit.x, p_hit.y, float(0.0)) * -self.phi_max.powi(2);
        let d2pduv = Vector3f::zero();
        let d2pdvv = Vector3f::zero();

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
        let p_err = Vector3f::new(p_hit.x, p_hit.y, float(0.0)).abs() * gammaf(3);

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
        (self.z_max - self.z_min) * self.radius * self.phi_max
    }
}
