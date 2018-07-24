use std::sync::Arc;
use cg::prelude::*;
use prelude::*;
use math::*;

use bsdf::Bsdf;
use shape::Shape;

#[derive(Copy, Clone, Debug)]
pub struct Shading {
    pub n: Normal,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,
    pub dndu: Normal,
    pub dndv: Normal,
}

#[derive(Clone, Debug)]
pub struct Interaction {
    pub p: Point3f,
    pub time: Float,
    pub p_err: Vector3f,
    pub wo: Vector3f,
    pub n: Option<Normal>,
}

impl Interaction {
    pub fn new(p: Point3f, n: Normal, p_err: Vector3f, wo: Vector3f, time: Float) -> Self {
        Self {
            p,
            time,
            p_err,
            wo,
            n: Some(n),
        }
    }

    pub fn is_surface_interaction(&self) -> bool {
        self.n.is_some()
    }

    pub fn spawn_ray(&self, dir: &Vector3f) -> Ray {
        let n = self.n.unwrap_or(Normal::zero());
        let o = offset_ray_origin(&self.p, &self.p_err, &n, dir);

        let mut ray = Ray::new(o, *dir);
        ray.time = self.time;
        ray
    }

    pub fn spawn_ray_to(&self, p: &Point3f) -> Ray {
        let n = self.n.unwrap_or(Normal::zero());
        let o = offset_ray_origin(&self.p, &self.p_err, &n, &(p - self.p));
        let d = p - o;

        let mut ray = Ray::new(o, d);
        ray.max = Some(float(1.0 - SHADOW_EPSILON));
        ray.time = self.time;
        // todo: ray.medium = GetMedium(d);
        ray
    }
}

#[derive(Clone, Debug, Shrinkwrap)]
#[shrinkwrap(mutable)]
pub struct SurfaceInteraction<'a> {
    #[shrinkwrap(main_field)]
    pub interaction: Interaction,
    pub uv: Point2f,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,
    pub dndu: Normal,
    pub dndv: Normal,
    pub shape: Option<&'a Shape>,
    pub shading: Shading,
    pub wo: Ray,
    pub bsdf: Arc<Bsdf>,
}

impl<'a> SurfaceInteraction<'a> {
    pub fn new(
        p: Point3f,
        p_err: Vector3f,
        uv: Point2f,
        wo: Vector3f,
        dpdu: Vector3f,
        dpdv: Vector3f,
        dndu: Normal,
        dndv: Normal,
        time: Float,
        shape: Option<&'a Shape>
    ) -> Self {
        let mut n: Normal = dpdu.cross(dpdv).normalize().into();

        // adjust normal based on orientation & handiness
        if shape.map_or(false, |s| s.reverse_orientation() ^ s.transform_swaps_handedness()) {
            *n *= float(-1.0);
        }

        let interaction = Interaction::new(p, n, p_err, wo, time);

        // init shading geom from true geom
        let shading = Shading {
            n,
            dpdu,
            dpdv,
            dndu,
            dndv,
        };

        unimplemented!();
/*
        Self {
            interaction,
            uv,
            dpdu,
            dpdv,
            dndu,
            dndv,
            shape,
            shading,
        }*/
    }

    pub fn set_shading_geometry(&mut self, dpdus: Vector3f, dpdvs: Vector3f, dndus: Normal, dndvs: Normal, orientation_is_authoritative: bool) {
        // compute shading.n for Self
        let mut n: Normal = dpdus.cross(dpdvs).normalize().into();

        // adjust normal based on orientation & handiness
        if self.shape.as_ref().map_or(false, |s| s.reverse_orientation() ^ s.transform_swaps_handedness()) {
            *n *= float(-1.0);
        }

        if orientation_is_authoritative {
            n = n.face_forward(self.shading.n);
        } else {
            self.shading.n = self.shading.n.face_forward(n);
        }

        // initialize shading partial derivative values
        self.shading.dpdu = dpdus;
        self.shading.dpdv = dpdvs;
        self.shading.dndu = dndus;
        self.shading.dndv = dndvs;
    }

    pub fn compute_scattering_functions(&self, ray: &Ray, arena: &()) {
        unimplemented!()
    }

    pub fn le(&self, ray: &Ray) -> Spectrum {
        Spectrum::new(0.0)
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Sample {
    pub li: Spectrum,
    pub wi: Ray,
    pub pdf: Float,
}
