use prelude::*;

pub trait Sampler {
    fn create_new(&self, seed: i32) -> Box<Sampler>;

    fn samples_per_pixel(&self) -> i32;

    fn start_pixel(&mut self, pixel: &Point2i);

    fn get_camera_sample(&mut self, pixel: &Point2i) -> CameraSample;

    fn start_next_sample(&self) -> bool;

    fn get_2d(&self) -> Point2f;
}

pub struct CameraSample {

}
