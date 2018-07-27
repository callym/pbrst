use std::fmt::Debug;
use std::sync::Arc;
use prelude::*;
use bxdf::{ Bsdf, TransportMode };
use interaction::SurfaceInteraction;

pub mod matte;
pub use self::matte::MatteMaterial;

pub trait Material: Debug {
    fn compute_scattering_functions<'a>(&self, isect: SurfaceInteraction<'a>, arena: &(), mode: TransportMode, allow_multiple_lobes: bool) -> SurfaceInteraction<'a>;
}

pub trait Texture<T> {
    type Output = T;

    fn evaluate(&self, si: &SurfaceInteraction) -> Self::Output;
}

pub fn bump<'a>(si: SurfaceInteraction<'a>, t: &Arc<Texture<Float, Output = Float>>) -> SurfaceInteraction<'a> {
    unimplemented!()
}
