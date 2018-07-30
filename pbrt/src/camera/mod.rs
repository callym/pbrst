use std::sync::{ Arc, Mutex };
use film::Film;
use math::*;
use sampler::CameraSample;

#[macro_use] mod macros;

mod orthographic;
pub use self::orthographic::OrthographicCamera;

mod perspective;
pub use self::perspective::PerspectiveCamera;

pub trait Camera {
    fn film(&self) -> Arc<Mutex<Film>>;

    fn generate_ray(&self, camera_sample: &CameraSample) -> (Float, Ray);

    fn generate_ray_differential(&self, camera_sample: &CameraSample) -> (Float, RayDifferential);
}
