use crate::scene::Scene;
use crate::sampler::Sampler;

mod sampler_integrator;
pub use self::sampler_integrator::{ ParIntegratorData, SamplerIntegrator };

mod normal;
pub use self::normal::{ NormalIntegrator };

mod direct_lighting;
pub use self::direct_lighting::{ DirectLightingIntegrator, LightStrategy };

mod whitted;
pub use self::whitted::WhittedIntegrator;

pub mod utils;

pub trait Integrator {
    fn render(&mut self, scene: Scene);

    fn preprocess(&mut self, _scene: &Scene, _sampler: &mut dyn Sampler) {

    }
}
