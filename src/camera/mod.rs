use std::sync::{ Arc, RwLock };
use prelude::*;
use math::*;
use sampler::CameraSample;

pub trait Camera {
    fn film(&self) -> Arc<RwLock<Film>>;

    fn generate_ray_differential(&self, camera_sample: &CameraSample) -> (RayDifferential, Float);
}

pub struct Film {
    sample_bounds: Bounds2<i32>,
}

impl Film {
    pub fn sample_bounds(&self) -> Bounds2<i32> {
        self.sample_bounds
    }

    pub fn film_tile(&self, bounds: &Bounds2<i32>) -> FilmTile {
        unimplemented!()
    }

    pub fn merge_film_tile(&mut self, tile: FilmTile) {

    }

    pub fn write_image(&mut self) {

    }
}

pub struct FilmTile {

}

impl FilmTile {
    pub fn add_sample(&mut self, pfilm: (), l: Spectrum, ray_weight: Float) {

    }
}
