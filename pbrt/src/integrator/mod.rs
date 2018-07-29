use std::sync::Arc;
use ::{
    scene::Scene,
    sampler::Sampler,
};

pub mod sampler_integrator;
pub use self::sampler_integrator::{ ParIntegratorData, SamplerIntegrator };

pub mod direct_lighting;
pub use self::direct_lighting::{ DirectLightingIntegrator, LightStrategy };

pub mod whitted;
pub use self::whitted::WhittedIntegrator;

mod utils;

pub trait Integrator {
    fn render(&mut self, scene: Scene);

    fn preprocess(&mut self, _scene: &Scene, _sampler: &mut Box<Sampler + Send>) {

    }
}
