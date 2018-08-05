use std::sync::Arc;
use num;
use crate::prelude::*;
use super::Material;
use crate::interaction::SurfaceInteraction;
use crate::bxdf::{ Bsdf, LambertianReflection, TransportMode };
use crate::texture::Texture;

#[derive(Clone, Debug)]
pub struct MatteMaterial {
    kd: Arc<dyn Texture<Spectrum> + Send + Sync>,
    sigma: Arc<dyn Texture<Float> + Send + Sync>,
    bump: Option<Arc<dyn Texture<Float> + Send + Sync>>,
}

impl MatteMaterial {
    pub fn new(kd: Arc<dyn Texture<Spectrum> + Send + Sync>, sigma: Arc<dyn Texture<Float> + Send + Sync>, bump: Option<Arc<dyn Texture<Float> + Send + Sync>>) -> Self {
        Self { kd, sigma, bump }
    }
}

impl Material for MatteMaterial {
    fn compute_scattering_functions(&self, isect: SurfaceInteraction<'a>, _arena: &(), _mode: TransportMode, _allow_multiple_lobes: bool) -> SurfaceInteraction<'a> {
        let isect = match &self.bump {
            Some(bump) => super::bump(&isect, bump),
            None => isect,
        };

        let mut isect = isect.clone();
        let mut bsdf = Bsdf::new(&isect, None);

        let r = self.kd.evaluate(&isect).clamp(None, None);
        let sig = num::clamp(self.sigma.evaluate(&isect), float(0.0), float(90.0));

        if !r.is_black() {
            if sig == 0.0 {
                bsdf.add(Arc::new(LambertianReflection::new(r)));
            } else {
                unimplemented!()
            }
        }

        isect.bsdf = Some(bsdf);
        isect
    }
}
