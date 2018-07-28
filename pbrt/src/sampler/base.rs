use std::default::Default;
use rand::Rng;
use xoshiro::Xoroshiro128StarStar;

use prelude::*;

#[derive(Debug)]
pub struct BaseSamplerData {
    current_pixel: Point2i,
    current_pixel_sample_index: u64,
    samples_array_1d_sizes: Vec<u32>,
    samples_array_2d_sizes: Vec<u32>,
    samples_array_1d: Vec<Vec<Float>>,
    samples_array_2d: Vec<Vec<Point2f>>,
    offset_1d: usize,
    offset_2d: usize,
    samples_per_pixel: u64,
}

impl BaseSamplerData {
    pub fn new(samples_per_pixel: u64) -> Self {
        Self {
            current_pixel: Point2i::zero(),
            current_pixel_sample_index: 0,
            samples_array_1d_sizes: vec![],
            samples_array_2d_sizes: vec![],
            samples_array_1d: vec![],
            samples_array_2d: vec![],
            offset_1d: 0,
            offset_2d: 0,
            samples_per_pixel,
        }
    }

    pub fn samples_array_1d_mut(&mut self) -> &mut Vec<Vec<Float>> {
        &mut self.samples_array_1d
    }

    pub fn samples_array_2d_mut(&mut self) -> &mut Vec<Vec<Point2f>> {
        &mut self.samples_array_2d
    }

    fn reset_array_offsets(&mut self) {
        self.offset_1d = 0;
        self.offset_2d = 0;
    }

    pub fn start_pixel(&mut self, pixel: &Point2i) {
        self.current_pixel = *pixel;
        self.current_pixel_sample_index = 0;
        self.reset_array_offsets();
    }

    pub fn start_next_sample(&mut self) -> bool {
        self.reset_array_offsets();
        self.current_pixel_sample_index += 1;
        self.current_pixel_sample_index < self.samples_per_pixel
    }

    pub fn set_sample_number(&mut self, num: u64) -> bool {
        self.reset_array_offsets();
        self.current_pixel_sample_index = num;
        self.current_pixel_sample_index < self.samples_per_pixel
    }

    pub fn get_1d_array(&mut self, n: u32) -> Option<&[Float]> {
        if self.offset_1d >= self.samples_array_1d.len() {
            None
        } else if self.samples_array_1d_sizes[self.offset_1d] != n {
            None
        } else {
            let o = self.offset_1d;
            self.offset_1d += 1;

            let start: usize = (self.current_pixel_sample_index * n as u64) as usize;
            let end = start + n as usize;

            Some(&self.samples_array_1d[o][start..end])
        }
    }

    pub fn get_2d_array(&mut self, n: u32) -> Option<&[Point2f]> {
        if self.offset_2d >= self.samples_array_2d.len() {
            None
        } else if self.samples_array_2d_sizes[self.offset_2d] != n {
            None
        } else {
            let o = self.offset_2d;
            self.offset_2d += 1;

            let start: usize = (self.current_pixel_sample_index * n as u64) as usize;
            let end = start + n as usize;

            Some(&self.samples_array_2d[o][start..end])
        }
    }
}

pub struct PixelSamplerData {
    base: BaseSamplerData,
    rng: Xoroshiro128StarStar,
    samples_1d: Vec<Vec<Float>>,
    samples_2d: Vec<Vec<Point2f>>,
    current_1d: usize,
    current_2d: usize,
}

impl PixelSamplerData {
    pub fn new(samples_per_pixel: u64, max_dimensions: u32, seed: i32) -> Self {
        let mut samples_1d = Vec::with_capacity(max_dimensions as usize);
        let mut samples_2d = Vec::with_capacity(max_dimensions as usize);

        for _ in 0..max_dimensions {
            samples_1d.push(vec![float(0.0); samples_per_pixel as usize]);
            samples_2d.push(vec![Point2f::zero(); samples_per_pixel as usize]);
        }

        Self {
            base: BaseSamplerData::new(samples_per_pixel),
            rng: Xoroshiro128StarStar::from_seed_u64(seed as u64),
            samples_1d,
            samples_2d,
            current_1d: 0,
            current_2d: 0,
        }
    }

    pub fn samples_per_pixel(&self) -> u64 {
        self.base.samples_per_pixel
    }

    pub fn samples_array_1d(&self) -> &Vec<Vec<Float>> {
        &self.base.samples_array_1d
    }

    pub fn samples_array_2d(&self) -> &Vec<Vec<Point2f>> {
        &self.base.samples_array_2d
    }

    pub fn samples_array_1d_mut(&mut self) -> &mut Vec<Vec<Float>> {
        &mut self.base.samples_array_1d
    }

    pub fn samples_array_2d_mut(&mut self) -> &mut Vec<Vec<Point2f>> {
        &mut self.base.samples_array_2d
    }

    pub fn samples_array_1d_sizes(&self) -> &Vec<u32> {
        &self.base.samples_array_1d_sizes
    }

    pub fn samples_array_2d_sizes(&self) -> &Vec<u32> {
        &self.base.samples_array_2d_sizes
    }

    pub fn samples_array_1d_sizes_mut(&mut self) -> &mut Vec<u32> {
        &mut self.base.samples_array_1d_sizes
    }

    pub fn samples_array_2d_sizes_mut(&mut self) -> &mut Vec<u32> {
        &mut self.base.samples_array_2d_sizes
    }

    pub fn samples_1d(&self) -> &Vec<Vec<Float>> {
        &self.samples_1d
    }

    pub fn samples_2d(&self) -> &Vec<Vec<Point2f>> {
        &self.samples_2d
    }

    pub fn samples_1d_mut(&mut self) -> &mut Vec<Vec<Float>> {
        &mut self.samples_1d
    }

    pub fn samples_2d_mut(&mut self) -> &mut Vec<Vec<Point2f>> {
        &mut self.samples_2d
    }

    fn reset_dimensions(&mut self) {
        self.current_1d = 0;
        self.current_2d = 0;
    }

    pub fn start_pixel(&mut self, pixel: &Point2i) {
        self.base.start_pixel(pixel);
    }

    pub fn start_next_sample(&mut self) -> bool {
        self.reset_dimensions();
        self.base.start_next_sample()
    }

    pub fn set_sample_number(&mut self, num: u64) -> bool {
        self.reset_dimensions();
        self.base.set_sample_number(num)
    }

    pub fn get_1d(&mut self) -> Float {
        if self.current_1d < self.samples_1d.len() {
            let c = self.current_1d;
            self.current_1d += 1;
            self.samples_1d[c][self.base.current_pixel_sample_index as usize]
        } else {
            self.rng.gen()
        }
    }

    pub fn get_2d(&mut self) -> Point2f {
        if self.current_2d < self.samples_2d.len() {
            let c = self.current_2d;
            self.current_2d += 1;
            self.samples_2d[c][self.base.current_pixel_sample_index as usize]
        } else {
            Point2f::new(self.rng.gen(), self.rng.gen())
        }
    }

    pub fn get_1d_vec(&mut self, n: u32) -> Option<&[Float]> {
        self.base.get_1d_array(n)
    }

    pub fn get_2d_vec(&mut self, n: u32) -> Option<&[Point2f]> {
        self.base.get_2d_array(n)
    }

    pub fn request_1d_vec(&mut self, n: u32) {
        self.base.samples_array_1d_sizes.push(n);
        self.base.samples_array_1d.push(vec![float(0.0); n as usize * self.base.samples_per_pixel as usize]);
    }

    pub fn request_2d_vec(&mut self, n: u32) {
        self.base.samples_array_2d_sizes.push(n);
        self.base.samples_array_2d.push(vec![Point2f::zero(); n as usize * self.base.samples_per_pixel as usize]);
    }

    pub fn get_rng(&mut self) -> &mut impl Rng {
        &mut self.rng
    }
}

const ARRAY_START_DIMENSION: u32 = 5;

pub struct GlobalSamplerData {
    base: BaseSamplerData,
    dimension: u32,
    interval_sample_index: u64,
    array_end_dimension: u32,
}

impl GlobalSamplerData {
    pub fn start_pixel(&mut self, sampler: &mut impl GlobalSampler, pixel: &Point2i) {
        self.base.start_pixel(pixel);
        self.dimension = 0;
        self.interval_sample_index = sampler.get_index_for_sample(0);

        // compute array end dim for dimensions used for array samples
        self.array_end_dimension = ARRAY_START_DIMENSION + self.base.samples_array_1d.len() as u32 + self.base.samples_array_2d.len() as u32;

        // compute 1d array samples for globalsampler
        for (i, size) in self.base.samples_array_1d_sizes.iter().enumerate() {
            let n = *size * self.base.samples_per_pixel as u32;
            let n = n as usize;

            for j in 0..n {
                let index = sampler.get_index_for_sample(j as u64);
                self.base.samples_array_1d[i][j] = sampler.sample_dimension(index, ARRAY_START_DIMENSION + 1);
            }
        }

        let mut dim = ARRAY_START_DIMENSION as usize + self.base.samples_array_1d_sizes.len();
        // compute 2d array samples for globalsampler
        for (i, size) in self.base.samples_array_2d_sizes.iter().enumerate() {
            let n = *size * self.base.samples_per_pixel as u32;
            let n = n as usize;

            for j in 0..n {
                let index = sampler.get_index_for_sample(j as u64);
                self.base.samples_array_2d[i][j] = Point2f::new(
                    sampler.sample_dimension(index, dim as u32),
                    sampler.sample_dimension(index, dim as u32 + 1),
                );
            }
            dim += 2;
        }
    }

    pub fn start_next_sample(&mut self, sampler: &mut impl GlobalSampler) -> bool {
        self.dimension = 0;
        self.interval_sample_index = sampler.get_index_for_sample(self.base.current_pixel_sample_index + 1);
        self.base.start_next_sample()
    }

    pub fn set_sample_number(&mut self, sampler: &mut impl GlobalSampler, num: u64) -> bool {
        self.dimension = 0;
        self.interval_sample_index = sampler.get_index_for_sample(num);
        self.base.set_sample_number(num)
    }

    pub fn get_1d(&mut self, sampler: &mut impl GlobalSampler) -> Float {
        if self.dimension >= ARRAY_START_DIMENSION && self.dimension < self.array_end_dimension {
            self.dimension = self.array_end_dimension;
        }
        let sample = sampler.sample_dimension(self.interval_sample_index, self.dimension);
        self.dimension += 1;
        sample
    }

    pub fn get_2d(&mut self, sampler: &mut impl GlobalSampler) -> Point2f {
        if self.dimension + 1 >= ARRAY_START_DIMENSION && self.dimension < self.array_end_dimension {
            self.dimension = self.array_end_dimension;
        }
        let x = sampler.sample_dimension(self.interval_sample_index, self.dimension);
        let y = sampler.sample_dimension(self.interval_sample_index, self.dimension + 1);

        self.dimension += 2;

        Point2f::new(x, y)
    }
}

pub trait GlobalSampler {
    fn get_index_for_sample(&self, num: u64) -> u64;
    fn sample_dimension(&self, index: u64, dimension: u32) -> Float;
}
