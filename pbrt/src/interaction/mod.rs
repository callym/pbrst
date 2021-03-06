use cgmath::prelude::*;
use crate::prelude::*;
use crate::bxdf::BxdfType;

mod surface;
pub use self::surface::*;

#[cfg_attr(feature = "cargo-clippy", allow(large_enum_variant))]
pub enum Interactions<'a> {
    Interaction(BaseInteraction),
    SurfaceInteraction(SurfaceInteraction<'a>),
}

impl Interactions<'_> {
    pub fn get_base(&self) -> &BaseInteraction {
        match self {
            Interactions::Interaction(isect) => isect,
            Interactions::SurfaceInteraction(isect) => &**isect,
        }
    }

    pub fn get_surface(&self) -> Option<&SurfaceInteraction<'_>> {
        match self {
            Interactions::Interaction(_) => None,
            Interactions::SurfaceInteraction(isect) => Some(isect),
        }
    }
}

impl From<BaseInteraction> for Interactions<'a> {
    fn from(si: BaseInteraction) -> Self {
        Interactions::Interaction(si)
    }
}

impl From<SurfaceInteraction<'a>> for Interactions<'a> {
    fn from(si: SurfaceInteraction<'a>) -> Self {
        Interactions::SurfaceInteraction(si)
    }
}

impl Into<BaseInteraction> for Interactions<'a> {
    fn into(self) -> BaseInteraction {
        self.get_base().clone()
    }
}

#[derive(Clone, Debug)]
pub struct BaseInteraction {
    pub p: Point3f,
    pub time: Float,
    pub p_err: Vector3f,
    pub wo: Vector3f,
    pub n: Option<Normal>,
    pub medium: Option<()>,
}

impl BaseInteraction {
    pub fn new(p: Point3f, n: Normal, p_err: Vector3f, wo: Vector3f, time: Float, medium: Option<()>) -> Self {
        let wo = wo.normalize();

        Self {
            p,
            time,
            p_err,
            wo,
            n: Some(n),
            medium,
        }
    }

    pub fn is_surface_interaction(&self) -> bool {
        self.n.map_or(false, |n| n != Normal::zero())
    }

    pub fn spawn_ray(&self, dir: &Vector3f) -> Ray {
        let n = self.n.unwrap_or_else(Normal::zero);
        let o = offset_ray_origin(&self.p, &self.p_err, &n, dir);

        let mut ray = Ray::new(o, *dir);
        ray.time = self.time;
        ray
    }

    pub fn spawn_ray_to(&self, p: impl Into<BaseInteraction>) -> Ray {
        let p: BaseInteraction = p.into();
        let n = self.n.unwrap_or_else(Normal::zero);
        let o = offset_ray_origin(&self.p, &self.p_err, &n, &(p.p - self.p));
        let target: Point3f = offset_ray_origin(&p.p, &p.p_err, &p.n.unwrap_or_else(Normal::zero), &(o - p.p));
        let d = target - o;

        let mut ray = Ray::new(o, d);
        ray.max = float(1.0 - SHADOW_EPSILON);
        ray.time = self.time;
        // todo: ray.medium = GetMedium(d);
        ray
    }
}

#[derive(Copy, Clone, Debug)]
pub struct Shading {
    pub n: Normal,
    pub dpdu: Vector3f,
    pub dpdv: Vector3f,
    pub dndu: Normal,
    pub dndv: Normal,
}

#[derive(Copy, Clone, Debug)]
pub struct Sample {
    pub li: Spectrum,
    pub wi: Vector3f,
    pub pdf: Float,
    pub ty: Option<BxdfType>,
}
