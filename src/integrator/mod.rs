use std::sync::Arc;
use ::{
    scene::Scene,
    sampler::Sampler,
};

mod sampler_integrator;
pub use self::sampler_integrator::SamplerIntegrator;

mod whitted;

mod utils;

pub trait Integrator {
    fn render(&mut self, scene: &Scene);

    fn preprocess(&mut self, _scene: &Scene, _sampler: &mut Box<Sampler>) {

    }
}
