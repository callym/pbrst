use std::cmp;
use std::sync::Arc;
use cg::prelude::*;
use cg::Vector3;
use num_cpus;
use rayon::prelude::*;
use prelude::*;
use super::utils::*;

use ::{
    camera::Camera,
    math::*,
    sampler::Sampler,
    scene::Scene,
};
use bxdf::{ BxdfType, TransportMode };
use super::{ ParIntegratorData, SamplerIntegrator };

#[derive(Copy, Clone, Debug, PartialEq, Eq)]
pub enum LightStrategy {
    UniformSampleAll,
    UniformSampleOne,
}

pub struct DirectLightingParIntegratorData {
    max_depth: i32,
    light_strategy: LightStrategy,
    n_light_samples: Arc<Vec<u32>>,
}

impl ParIntegratorData for DirectLightingParIntegratorData {
    fn li(&self, mut ray: RayDifferential, scene: &Scene, sampler: &mut Box<Sampler + Send>, arena: &(), depth: i32) -> Spectrum {
        let mut l = Spectrum::new(0.0);

        if let Some(mut isect) = scene.intersect(&mut ray) {
            let mode = TransportMode::Radiance;
            isect.compute_scattering_functions(&ray, &(), mode, false);

            let wo = isect.wo;
            l += isect.le(&wo);

            if scene.lights.len() > 0 {
                // compute direct lighting
                l += match self.light_strategy {
                    LightStrategy::UniformSampleAll => uniform_sample_all_lights(&isect, scene, sampler, arena, &self.n_light_samples, false),
                    LightStrategy::UniformSampleOne => uniform_sample_one_light(&isect, scene, sampler, arena, &self.n_light_samples, false),
                };
            }

            if depth + 1 < self.max_depth {
                l += specular_reflect(self, &ray, &isect, &scene, sampler, &arena, depth);
                l += specular_transmit(self, &ray, &isect, &scene, sampler, &arena, depth);
            }
        } else {
            for light in &*scene.lights {
                l += light.le(&ray);
            }
        }

        l
    }
}

pub struct DirectLightingIntegrator {
    max_depth: i32,
    light_strategy: LightStrategy,
    camera: Arc<Camera + Send + Sync>,
    sampler: Box<Sampler>,
    n_light_samples: Arc<Vec<u32>>,
}

impl DirectLightingIntegrator {
    pub fn new(max_depth: i32, light_strategy: LightStrategy, camera: Arc<Camera + Send + Sync>, sampler: Box<Sampler>) -> Self {
        Self {
            max_depth,
            light_strategy,
            camera,
            sampler,
            n_light_samples: Arc::new(vec![]),
        }
    }
}

impl SamplerIntegrator for DirectLightingIntegrator {
    type ParIntegratorData = DirectLightingParIntegratorData;

    fn camera(&self) -> Arc<Camera + Send + Sync> {
        self.camera.clone()
    }

    fn sampler(&self) -> &Box<Sampler> {
        &self.sampler
    }

    fn sampler_mut(&mut self) -> &mut Box<Sampler> {
        &mut self.sampler
    }

    fn preprocess(&mut self, scene: &Scene, sampler: &mut Box<Sampler + Send>) {
        let mut n_light_samples = vec![];
        if self.light_strategy == LightStrategy::UniformSampleAll {
            // compute number of samples to use for each light
            for light in &*scene.lights {
                n_light_samples.push(sampler.round_count(light.num_samples()));
            }

            // request samples for sampling all lights
            for _ in 0..self.max_depth {
                for j in 0..scene.lights.len() {
                    sampler.request_2d_vec(n_light_samples[j]);
                    sampler.request_2d_vec(n_light_samples[j]);
                }
            }
        }
    }

    fn par_data(&self) -> Self::ParIntegratorData {
        DirectLightingParIntegratorData {
            max_depth: self.max_depth,
            light_strategy: self.light_strategy,
            n_light_samples: self.n_light_samples.clone(),
        }
    }
}
