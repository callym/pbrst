use std::sync::{ Arc, RwLock };
use cg::prelude::*;
use cg::Matrix4;
use prelude::*;
use math::*;
use math::transform::Transform;
use sampler::CameraSample;

#[macro_use] mod macros;

pub mod orthographic;
pub use self::orthographic::OrthographicCamera;

pub mod perspective;
pub use self::perspective::PerspectiveCamera;

pub trait Camera {
    fn film(&self) -> Arc<Film>;

    fn generate_ray(&self, camera_sample: &CameraSample) -> (Float, Ray);

    fn generate_ray_differential(&self, camera_sample: &CameraSample) -> (Float, RayDifferential);
}

pub struct Film {
    pub sample_bounds: Bounds2<i32>,
    pub full_resolution: Point2i,
}

impl Film {
    pub fn film_tile(&self, bounds: &Bounds2<i32>) -> FilmTile {
        unimplemented!()
    }

    pub fn merge_film_tile(&self, tile: FilmTile) {

    }

    pub fn write_image(&self) {

    }
}

pub struct FilmTile {

}

impl FilmTile {
    pub fn add_sample(&mut self, pfilm: (), l: Spectrum, ray_weight: Float) {

    }
}
