use std::sync::{ Arc, Mutex };
use crate::film::Film;
use crate::math::*;
use crate::sampler::CameraSample;

#[macro_use] mod macros;

mod orthographic;
pub use self::orthographic::OrthographicCamera;

mod perspective;
pub use self::perspective::PerspectiveCamera;

pub trait Camera {
    fn film(&self) -> Arc<Mutex<Film>>;

    fn generate_ray_differential(&self, camera_sample: &CameraSample) -> (Float, RayDifferential);
}
