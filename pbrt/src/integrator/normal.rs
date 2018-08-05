use std::sync::Arc;
use itertools::izip;
use crate::prelude::*;
use super::utils::*;

use crate::camera::Camera;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use crate::bxdf::TransportMode;
use crate::spectrum::utils::*;
use super::{ ParIntegratorData, SamplerIntegrator };

pub struct NormalParIntegratorData;

impl ParIntegratorData for NormalParIntegratorData {
    fn li(&self, mut ray: RayDifferential, scene: &Scene, sampler: &mut dyn Sampler, arena: &(), depth: i32) -> Spectrum {
        let mut l = Spectrum::new(1.0);

        if let Some(isect) = scene.intersect(&mut ray) {
            let n = isect.n.unwrap();
            let n = Spectrum::from_rgb([n.x, n.y, n.z], SpectrumType::Reflectance);

            l = n;
        }

        l
    }
}

pub struct NormalIntegrator {
    camera: Arc<dyn Camera + Send + Sync>,
    sampler: Box<dyn Sampler>,
}

impl NormalIntegrator {
    pub fn new(camera: Arc<dyn Camera + Send + Sync>, sampler: Box<dyn Sampler>) -> Self {
        Self {
            camera,
            sampler,
        }
    }
}

impl SamplerIntegrator for NormalIntegrator {
    type ParIntegratorData = NormalParIntegratorData;

    fn camera(&self) -> Arc<dyn Camera + Send + Sync> {
        self.camera.clone()
    }

    fn sampler(&self) -> &dyn Sampler {
        self.sampler.as_ref()
    }

    fn sampler_mut(&mut self) -> &mut dyn Sampler {
        self.sampler.as_mut()
    }

    fn par_data(&self) -> Self::ParIntegratorData {
        NormalParIntegratorData
    }
}
