#[cfg(not(feature = "double"))]
use hexf::{ hexf32, hexf32_impl };

#[cfg(feature = "double")]
use hexf::{ hexf64, hexf64_impl };

use crate::prelude::*;

#[cfg(not(feature = "double"))]
pub const ONE_MINUS_EPSILON: FloatPrim = hexf32!("0x1.fffffep-1");

#[cfg(feature = "double")]
pub const ONE_MINUS_EPSILON: FloatPrim = hexf64!("0x1.fffffffffffffp-1");

mod base;
pub use self::base::*;

mod stratified;
pub use self::stratified::StratifiedSampler;

pub trait Sampler {
    fn create_new(&self, seed: i32) -> Box<dyn Sampler + Send + 'static>;

    fn samples_per_pixel(&self) -> u64;

    fn start_pixel(&mut self, pixel: Point2i);

    fn get_camera_sample(&mut self, pixel: Point2i) -> CameraSample {
        CameraSample {
            film: pixel.map(|p| float(p as f32)) + self.get_2d().into_vector(),
            time: self.get_1d(),
            lens: self.get_2d(),
        }
    }

    fn set_sample_number(&mut self, n: u64) -> bool;

    fn start_next_sample(&mut self) -> bool;

    fn get_1d(&mut self) -> Float;

    fn get_2d(&mut self) -> Point2f;

    fn request_1d_vec(&mut self, n: u32);

    fn request_2d_vec(&mut self, n: u32);

    fn get_1d_vec(&mut self, n: u32) -> Option<Vec<Float>>;

    fn get_2d_vec(&mut self, n: u32) -> Option<Vec<Point2f>>;

    fn round_count(&self, n: u32) -> u32 {
        n
    }
}

#[derive(Copy, Clone, Debug)]
pub struct CameraSample {
    pub lens: Point2f,
    pub film: Point2f,
    pub time: Float,
}

