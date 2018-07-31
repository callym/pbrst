use std::fmt::Debug;
use std::sync::Arc;
use crate::prelude::*;
use crate::bxdf::TransportMode;
use crate::interaction::SurfaceInteraction;
use crate::texture::Texture;

mod matte;
pub use self::matte::MatteMaterial;

pub trait Material: Debug {
    fn compute_scattering_functions(&self, isect: SurfaceInteraction<'a>, arena: &(), mode: TransportMode, allow_multiple_lobes: bool) -> SurfaceInteraction<'a>;
}

pub fn bump(_si: &SurfaceInteraction<'a>, _t: &Arc<dyn Texture<Float> + Send + Sync>) -> SurfaceInteraction<'a> {
    unimplemented!()
}
