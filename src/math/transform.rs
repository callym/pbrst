use std::sync::Arc;
use std::cmp::max;
use cg::prelude::*;
use cg::Matrix4;
use cg::Matrix3;
use cg::Quaternion;
use cg::Transform as _;
use num;
use prelude::*;
use interaction::SurfaceInteraction;
use super::TermsOfMotion;

type Matrix4f = Matrix4<Float>;
type Quaternionf = Quaternion<Float>;

#[derive(Copy, Clone, Debug, PartialEq)]
pub struct Transform {
    pub matrix: Matrix4f,
    pub inverse: Matrix4f,
}

impl Transform {
    pub fn identity() -> Self {
        Self {
            matrix: Matrix4::identity(),
            inverse: Matrix4::identity(),
        }
    }

    pub fn new(matrix: Matrix4f) -> Self {
        let inverse = matrix.inverse_transform().unwrap();

        Self {
            matrix,
            inverse,
        }
    }

    pub fn inverse(&self) -> Self {
        Self {
            matrix: self.inverse,
            inverse: self.matrix,
        }
    }

    pub fn is_identity(&self) -> bool {
        self.matrix.is_identity()
    }

    pub fn swaps_handedness(&self) -> bool {
        let m = Matrix3 {
            x: self.matrix.x.xyz(),
            y: self.matrix.y.xyz(),
            z: self.matrix.z.xyz(),
        };

        m.determinant() < 0.0
    }

    pub fn transform_point(&self, point: Point3f) -> Point3f {
        self.matrix.transform_point(point)
    }

    pub fn transform_point_with_abs_error(&self, _: Point3f, _: Vector3f) -> (Point3f, Vector3f) {
        unimplemented!()
    }

    pub fn transform_vector(&self, vector: Vector3f) -> Vector3f {
        self.matrix.transform_vector(vector)
    }

    pub fn transform_normal(&self, normal: Normal) -> Normal {
        let transpose = self.inverse.transpose();
        transpose.transform_vector(*normal).into()
    }

    pub fn transform_ray_data(&self, ray: RayData) -> RayData {
        let origin = self.transform_point(ray.origin);
        let direction = self.transform_vector(ray.direction);
        // Offset ray origin to the edge of error bounds and compute tMax
        RayData {
            origin,
            direction,
        }
    }

    pub fn transform_ray_data_with_error(&self, _: RayData, _: Vector3f, _: Vector3f) -> (RayData, Vector3f, Vector3f) {
        unimplemented!()
    }

    pub fn transform_ray(&self, mut ray: Ray) -> Ray {
        ray.ray = self.transform_ray_data(ray.ray);
        ray
    }

    pub fn transform_ray_with_error(&self, _: Ray, _: Vector3f, _: Vector3f) -> (Ray, Vector3f, Vector3f) {
        unimplemented!()
    }

    pub fn transform_ray_differential(&self, mut ray: RayDifferential) -> RayDifferential {
        ray.ray = self.transform_ray(ray.ray);
        ray.x = ray.x.map(|x| self.transform_ray_data(x));
        ray.y = ray.y.map(|y| self.transform_ray_data(y));
        ray
    }

    pub fn transform_ray_differential_with_error(&self, _: RayDifferential, _: Vector3f, _: Vector3f) -> (RayDifferential, Vector3f, Vector3f) {
        unimplemented!()
    }

    // todo - this can be more efficient
    pub fn transform_bounds(&self, bounds: Bounds3f) -> Bounds3f {
        let p = |x, y, z| self.transform_point(Point3f::new(x, y, z));
        let ret = Bounds3f::from_point(self.transform_point(bounds.min));

        let ret = ret.union_p(p(bounds.max.x, bounds.min.y, bounds.min.z));
        let ret = ret.union_p(p(bounds.min.x, bounds.max.y, bounds.min.z));
        let ret = ret.union_p(p(bounds.min.x, bounds.min.y, bounds.max.z));

        let ret = ret.union_p(p(bounds.min.x, bounds.max.y, bounds.max.z));
        let ret = ret.union_p(p(bounds.max.x, bounds.max.y, bounds.min.z));
        let ret = ret.union_p(p(bounds.max.x, bounds.min.y, bounds.max.z));

        let ret = ret.union_p(p(bounds.max.x, bounds.max.y, bounds.max.z));

        ret
    }

    pub fn transform_surface_interaction(&self, si: &SurfaceInteraction) -> SurfaceInteraction {
        let mut ret = si.clone();

        let (p, p_err) = self.transform_point_with_abs_error(ret.p, ret.p_err);
        ret.p = p;
        ret.p_err = p_err;

        ret.n = ret.n.map(|n| self.transform_normal(n).normalize());
        ret.interaction.wo = self.transform_vector(ret.interaction.wo).normalize();

        ret.dpdu = self.transform_vector(ret.dpdu);
        ret.dpdv = self.transform_vector(ret.dpdv);
        ret.dndu = self.transform_normal(ret.dndu);
        ret.dndv = self.transform_normal(ret.dndv);

        ret.shading.n = self.transform_normal(ret.shading.n).normalize();
        ret.shading.dpdu = self.transform_vector(ret.shading.dpdu);
        ret.shading.dpdv = self.transform_vector(ret.shading.dpdv);
        ret.shading.dndu = self.transform_normal(ret.shading.dndu);
        ret.shading.dndv = self.transform_normal(ret.shading.dndv);

        if let Some(n) = ret.n {
            ret.shading.n = ret.shading.n.face_forward(n);
        }

        ret
    }

    pub fn transform_point_inverse(&self, point: Point3f) -> Point3f {
        self.inverse.transform_point(point)
    }

    pub fn transform_vector_inverse(&self, vector: Vector3f) -> Vector3f {
        self.inverse.transform_vector(vector)
    }

    pub fn transform_normal_inverse(&self, normal: Normal) -> Normal {
        let transpose = self.matrix.transpose();
        transpose.transform_vector(*normal).into()
    }

    pub fn transform_ray_data_inverse(&self, ray: RayData) -> RayData {
        let origin = self.transform_point_inverse(ray.origin);
        let direction = self.transform_vector_inverse(ray.direction);
        // Offset ray origin to the edge of error bounds and compute tMax
        RayData {
            origin,
            direction,
        }
    }

    pub fn transform_ray_inverse(&self, mut ray: Ray) -> Ray {
        ray.ray = self.transform_ray_data_inverse(ray.ray);
        ray
    }

    pub fn transform_ray_differential_inverse(&self, mut ray: RayDifferential) -> RayDifferential {
        ray.ray = self.transform_ray(ray.ray);
        ray.x = ray.x.map(|x| self.transform_ray_data_inverse(x));
        ray.y = ray.y.map(|y| self.transform_ray_data_inverse(y));
        ray
    }
}

/// Represents a decomposition from a *Transform* into `M = TRS`
#[derive(Copy, Clone, Debug)]
pub struct Decomposed {
    pub translate: Vector3f,
    pub rotate: Quaternionf,
    pub scale: Matrix4f,
}

impl Into<Decomposed> for Transform {
    fn into(self) -> Decomposed {
        let m = self.matrix;
        // extract translation T
        let t = Vector3f::new(m[3][0], m[3][1], m[3][2]);

        // compute M without translation
        let m_orig = m;
        let mut m = m;
        for i in 0..3 {
            m[3][i] = float(0.0);
            m[i][3] = float(0.0);
        }
        m[3][3] = float(1.0);

        // extract rotation R
        let mut norm = float(0.0);
        let mut count = 0;
        let mut r = m;

        while count < 100 && norm > 0.0001 {
            let mut r_next = Matrix4::zero();
            let r_it = r.transpose().inverse_transform().unwrap();

            for i in 0..4 {
                for j in 0..4 {
                    r_next[i][j] = float(0.5) * (r[i][j] * r_it[i][j]);
                }
            }

            norm = float(0.0);
            for i in 0..3 {
                let n = (r[0][i] - r_next[0][i]).abs() +
                        (r[1][i] - r_next[1][i]).abs() +
                        (r[2][i] - r_next[2][i]).abs();
                norm = max(norm, n);
            }

            r = r_next;

            count += 1;
        };

        // convert r into a quaternion
        let trace = r[0][0] + r[1][1] + r[2][2];
        let r_q = if trace > 0.0 {
            // compute w from matrix trace, then xyz
            let s = (trace + float(1.0)).sqrt();
            let w = s / float(2.0);
            let s = float(0.5) / s;

            Quaternion {
                v: Vector3f::new(
                    r[1][2] - r[2][1] * s,
                    r[2][0] - r[0][2] * s,
                    r[0][1] - r[1][0] * s,
                ),
                s: w,
            }
        } else {
            let next = [1usize, 2, 0];
            let mut q = [float(0.0); 3];

            let i: usize = if r[1][1] > r[0][0] {
                1
            } else if r[2][2] > r[1][1] {
                2
            } else {
                0
            };
            let j = next[i];
            let k = next[j];

            let mut s = ((r[i][i] - (r[j][j] + r[k][k])) + float(1.0)).sqrt();
            q[i] = s * float(0.5);
            if s != 0.0 {
                s = float(0.5) / s;
            }
            let w = (r[j][k] - r[k][j]) * s;
            q[j] = (r[i][j] + r[j][i]) * s;
            q[k] = (r[i][k] + r[k][i]) * s;

            Quaternion {
                v: Vector3f::new(q[0], q[1], q[2]),
                s: w,
            }
        };

        // compute scale S using rotation and original
        let s = r.inverse_transform().unwrap() * m;

        Decomposed {
            translate: t,
            rotate: r_q,
            scale: s,
        }
    }
}

#[derive(Clone, Debug)]
pub struct AnimatedTransform {
    pub start: Arc<Transform>,
    pub start_time: Float,
    pub end: Arc<Transform>,
    pub end_time: Float,
    decomposed: (Decomposed, Decomposed),
    terms_of_motion: TermsOfMotion,
}

impl AnimatedTransform {
    pub fn new(start: Arc<Transform>, start_time: Float, end: Arc<Transform>, end_time: Float) -> Self {
        // decompose start and end
        let start_d: Decomposed = (*start).into();
        let mut end_d: Decomposed = (*end).into();

        // flip R[1] if needed to select the shortest path
        if start_d.rotate.dot(end_d.rotate) < 0.0 {
            end_d.rotate = -end_d.rotate;
        }

        // compute terms of motion derivative function
        let terms_of_motion = TermsOfMotion::new(start_d, end_d);

        Self {
            start,
            end,
            start_time,
            end_time,
            decomposed: (start_d, end_d),
            terms_of_motion,
        }
    }

    pub fn animated(&self) -> bool {
        self.start != self.end
    }

    fn start(&self) -> &Decomposed {
        &self.decomposed.0
    }

    fn end(&self) -> &Decomposed {
        &self.decomposed.1
    }

    pub fn has_rotation(&self) -> bool {
        self.start().rotate.dot(self.end().rotate) < 0.9995
    }

    pub fn interpolate(&self, time: Float) -> Transform {
        if !self.animated() {
            return *self.start;
        }

        if time <= self.start_time {
            return *self.start;
        }

        if time >= self.end_time {
            return *self.end;
        }

        let dt = (time - self.start_time) / (self.end_time - self.start_time);

        // interpolate translation at dt
        let trans = self.start().translate * (float(1.0) - dt) + self.end().translate * dt;
        let trans = Matrix4f::from_translation(trans);

        // interpolate rotation at dt
        let rotate = self.start().rotate.slerp(self.end().rotate, dt);
        let rotate: Matrix4f = rotate.into();

        // interpolate scale at dt
        let mut scale = Matrix4f::identity();
        for i in 0..3 {
            for j in 0..3 {
                scale[i][j] = self.start().scale[i][j].lerp(self.end().scale[i][j], dt);
            }
        }

        // compute interpolated matrix
        let m = trans * rotate * scale;

        Transform::new(m)
    }

    pub fn transform_point(&self, time: Float, p: Point3f) -> Point3f {
        if !self.animated() {
            self.start.transform_point(p)
        } else if time <= self.start_time {
            self.start.transform_point(p)
        } else if time >= self.end_time {
            self.end.transform_point(p)
        } else {
            let t = self.interpolate(time);
            t.transform_point(p)
        }
    }

    pub fn transform_vector(&self, time: Float, p: Vector3f) -> Vector3f {
        if !self.animated() {
            self.start.transform_vector(p)
        } else if time <= self.start_time {
            self.start.transform_vector(p)
        } else if time >= self.end_time {
            self.end.transform_vector(p)
        } else {
            let t = self.interpolate(time);
            t.transform_vector(p)
        }
    }

    pub fn transform_ray_data(&self, time: Float, p: RayData) -> RayData {
        if !self.animated() {
            self.start.transform_ray_data(p)
        } else if time <= self.start_time {
            self.start.transform_ray_data(p)
        } else if time >= self.end_time {
            self.end.transform_ray_data(p)
        } else {
            let t = self.interpolate(time);
            t.transform_ray_data(p)
        }
    }

    pub fn transform_ray(&self, time: Float, p: Ray) -> Ray {
        if !self.animated() {
            self.start.transform_ray(p)
        } else if time <= self.start_time {
            self.start.transform_ray(p)
        } else if time >= self.end_time {
            self.end.transform_ray(p)
        } else {
            let t = self.interpolate(time);
            t.transform_ray(p)
        }
    }

    pub fn transform_ray_differential(&self, time: Float, p: RayDifferential) -> RayDifferential {
        if !self.animated() {
            self.start.transform_ray_differential(p)
        } else if time <= self.start_time {
            self.start.transform_ray_differential(p)
        } else if time >= self.end_time {
            self.end.transform_ray_differential(p)
        } else {
            let t = self.interpolate(time);
            t.transform_ray_differential(p)
        }
    }

    pub fn motion_bounds(&self, bounds: Bounds3f) -> Bounds3f {
        if !self.animated() {
            return self.start.transform_bounds(bounds);
        }

        if !self.has_rotation() {
            let start = self.start.transform_bounds(bounds);
            let end = self.end.transform_bounds(bounds);
            return start.union(end);
        }

        let mut bounds_new = Bounds3f::empty();
        for corner in 0..8 {
            bounds_new = bounds_new.union(self.bound_point_motion(bounds.corner(corner)))
        }

        bounds_new
    }

    pub fn bound_point_motion(&self, p: Point3f) -> Bounds3f {
        let mut bounds = Bounds3f::new(self.start.transform_point(p), self.end.transform_point(p));

        let cos_theta = self.start().rotate.dot(self.end().rotate);
        let theta = num::clamp(cos_theta, float(-1.0), float(1.0)).acos();

        for c in 0..3 {
            let (num_zero, zeros) = self.terms_of_motion.interval_find_zeros(c, p, theta, Interval::new(float(0.0), float(1.0)), 8);

            for i in 0..num_zero {
                let pz = self.transform_point(zeros[i].lerp(self.start_time, self.end_time), p);
                bounds = bounds.union_p(pz);
            }
        }

        bounds
    }
}
