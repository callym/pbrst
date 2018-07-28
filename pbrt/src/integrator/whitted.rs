use std::cmp;
use std::sync::Arc;
use cg::prelude::*;
use cg::Vector3;
use num_cpus;
use rayon::prelude::*;
use prelude::*;

use ::{
    camera::Camera,
    math::*,
    sampler::Sampler,
    scene::Scene,
};
use bxdf::{ BxdfType, TransportMode };
use super::{ ParIntegratorData, SamplerIntegrator };

pub struct WhittedParIntegratorData {
    max_depth: i32,
}

impl ParIntegratorData for WhittedParIntegratorData {
    fn li(&self, mut ray: RayDifferential, scene: &Scene, sampler: &mut Box<Sampler + Send>, arena: &(), depth: i32) -> Spectrum {
        let mut l = Spectrum::new(0.0);

        if let Some(mut isect) = scene.intersect(&mut ray) {
            // initialise common variables for Whitted
            let n = isect.shading.n;
            let wo = isect.wo;

            // compute scattering fn for surface interaction
            isect.compute_scattering_functions(&ray, &arena, TransportMode::Camera, false);

            // compute emitted light if ray hit area light source
            l += isect.le(&wo);

            // add contribution of each light source
            for light in scene.lights.iter() {
                let (sample, visibility) = light.sample_li(&isect, sampler.get_2d());

                if sample.li.is_black() || sample.pdf == 0.0 {
                    continue;
                }

                let bsdf = match &isect.bsdf {
                    Some(bsdf) => bsdf,
                    None => continue,
                };

                let f = bsdf.f(wo, sample.wi, BxdfType::all());

                if !f.is_black() && visibility.map_or(false, |v| v.unoccluded(scene)) {
                    l += f * sample.li * sample.wi.dot(*n).abs() / sample.pdf;
                }
            }

            if depth + 1 < self.max_depth {
                // trace rays for specular reflection & refraction
                l += super::utils::specular_reflect(self, &ray, &isect, &scene, sampler, &arena, depth);
                l += super::utils::specular_transmit(self, &ray, &isect, &scene, sampler, &arena, depth);
            }

        } else {
            for light in scene.lights.iter() {
                l += light.le(&ray);
            }
        }

        l
    }
}

pub struct WhittedIntegrator {
    max_depth: i32,
    camera: Arc<Camera + Send + Sync>,
    sampler: Box<Sampler>,
}

impl WhittedIntegrator {
    pub fn new(max_depth: i32, camera: Arc<Camera + Send + Sync>, sampler: Box<Sampler>) -> Self {
        Self {
            max_depth,
            camera,
            sampler,
        }
    }
}

impl SamplerIntegrator for WhittedIntegrator {
    type ParIntegratorData = WhittedParIntegratorData;

    fn camera(&self) -> Arc<Camera + Send + Sync> {
        self.camera.clone()
    }

    fn sampler(&self) -> &Box<Sampler> {
        &self.sampler
    }

    fn sampler_mut(&mut self) -> &mut Box<Sampler> {
        &mut self.sampler
    }

    fn par_data(&self) -> Self::ParIntegratorData {
        WhittedParIntegratorData {
            max_depth: self.max_depth,
        }
    }
}
