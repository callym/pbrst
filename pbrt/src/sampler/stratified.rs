use std::cmp::{ max, min };
use itertools::Itertools;
use rand::Rng;
use rand::distributions::{ Distribution, Standard, Uniform };
use prelude::*;
use super::*;

pub struct StratifiedSampler {
    pixel: PixelSamplerData,
    x_samples: u32,
    y_samples: u32,
    jitter: bool,
    dimensions: u32,
}

impl StratifiedSampler {
    pub fn new(x_samples: u32, y_samples: u32, jitter: bool, dimensions: u32, seed: i32) -> Self {
        let samples = x_samples as u64 * y_samples as u64;
        let pixel = PixelSamplerData::new(samples, dimensions, seed);

        Self {
            pixel,
            x_samples,
            y_samples,
            jitter,
            dimensions,
        }
    }
}

impl Sampler for StratifiedSampler {
    fn create_new(&self, seed: i32) -> Box<Sampler> {
        let sampler = Self::new(
            self.x_samples,
            self.y_samples,
            self.jitter,
            self.dimensions,
            seed,
        );
        Box::new(sampler)
    }

    fn samples_per_pixel(&self) -> u64 {
        self.pixel.samples_per_pixel()
    }

    fn start_pixel(&mut self, pixel: &Point2i) {
        let samples_per_pixel = self.pixel.samples_per_pixel();

        // generate single stratified samples for pixel
        let samples_1d = self.pixel.samples_1d();
        for i in 0..samples_1d.len() {
            let mut sample = stratified_sample_1d(self.x_samples * self.y_samples, self.jitter, self.pixel.get_rng());
            shuffle(&mut sample, 1, self.pixel.get_rng());
            let pixel = self.pixel.samples_1d_mut();
            pixel[i] = sample;
        }

        let samples_2d = self.pixel.samples_2d();
        for i in 0..samples_2d.len() {
            let mut sample = stratified_sample_2d(self.x_samples, self.y_samples, self.jitter, self.pixel.get_rng());
            shuffle(&mut sample, 1, self.pixel.get_rng());
            let pixel = self.pixel.samples_2d_mut();
            pixel[i] = sample;
        }

        // generate arrays of stratified samples for pixel
        let samples_1d_sizes = self.pixel.samples_array_1d_sizes();
        for i in 0..samples_1d_sizes.len() {
            for j in 0..samples_per_pixel {
                let count = self.pixel.samples_array_1d_sizes()[i];
                let mut sample = stratified_sample_1d(count, self.jitter, self.pixel.get_rng());
                shuffle(&mut sample, 1, self.pixel.get_rng());
                let pixel = self.pixel.samples_array_1d_mut();
                pixel[i] = sample;
            }
        }

        let samples_2d_sizes = self.pixel.samples_array_2d_sizes();
        for i in 0..samples_2d_sizes.len() {
            for j in 0..samples_per_pixel {
                let count = self.pixel.samples_array_2d_sizes()[i];
                let sample = latin_hypercube(count as usize, 2, self.pixel.get_rng());
                let pixel = self.pixel.samples_array_2d_mut();
                pixel[i] = sample
                    .chunks(2)
                    .map(|c| Point2f::new(c[0], c[1]))
                    .collect();
            }
        }


        self.pixel.start_pixel(pixel)
    }

    fn set_sample_number(&mut self, n: u64) -> bool {
        self.pixel.set_sample_number(n)
    }

    fn start_next_sample(&mut self) -> bool {
        self.pixel.start_next_sample()
    }

    fn get_1d(&mut self) -> Float {
        self.pixel.get_1d()
    }

    fn get_2d(&mut self) -> Point2f {
        self.pixel.get_2d()
    }

    fn request_1d_vec(&mut self, n: u32) {
        self.pixel.request_1d_vec(n)
    }

    fn request_2d_vec(&mut self, n: u32) {
        self.pixel.request_2d_vec(n)
    }

    fn get_1d_vec(&mut self, n: u32) -> Option<&[Float]> {
        self.pixel.get_1d_vec(n)
    }

    fn get_2d_vec(&mut self, n: u32) -> Option<&[Point2f]> {
        self.pixel.get_2d_vec(n)
    }
}

fn jitter_value(jitter: bool, rng: &mut impl Rng) -> Float {
    if jitter { rng.gen() } else { float(0.5) }
}

fn stratified_sample_1d(samples: u32, jitter: bool, rng: &mut impl Rng) -> Vec<Float> {
    let inv_n = float(1.0) / float(samples);

    let mut vec = Vec::with_capacity(samples as usize);

    for i in 0..samples {
        let i = float(i);
        let delta = jitter_value(jitter, rng);
        vec.push(min((i + delta) * inv_n, float(ONE_MINUS_EPSILON)));
    }

    vec
}

fn stratified_sample_2d(x_samples: u32, y_samples: u32, jitter: bool, rng: &mut impl Rng) -> Vec<Point2f> {
    let inv_x = float(1.0) / float(x_samples);
    let inv_y = float(1.0) / float(y_samples);

    let mut vec = Vec::with_capacity(x_samples as usize * y_samples as usize);

    for y in 0..y_samples {
        for x in 0..x_samples {
            let x = float(x);
            let y = float(y);

            let jx = jitter_value(jitter, rng);
            let jy = jitter_value(jitter, rng);

            let x = min((x + jx) * inv_x, float(ONE_MINUS_EPSILON));
            let y = min((y + jy) * inv_y, float(ONE_MINUS_EPSILON));

            vec.push(Point2f::new(x, y));
        }
    }

    vec
}

fn shuffle<T>(vec: &mut Vec<T>, dimensions: usize, rng: &mut impl Rng) {
    for i in 0..vec.len() {
        let bounds = Uniform::from(0..vec.len() - i);
        let other = i + bounds.sample(rng);

        for j in 0..dimensions {
            vec.swap(dimensions * i + j, dimensions * other + j);
        }
    }
}

fn latin_hypercube(n: usize, dimensions: usize, rng: &mut impl Rng) -> Vec<Float> {
    let inv_n = float(1.0) / float(n);

    let mut vec = Vec::with_capacity(n * dimensions);

    for i in 0..n {
        for j in 0..dimensions {
            let sj = (float(i) + rng.gen()) * inv_n;
            vec[dimensions * i + j] = min(sj, float(ONE_MINUS_EPSILON));
        }
    }

    for i in 0..dimensions {
        for j in 0..n {
            let bounds = Uniform::from(0..(n - j));
            let other = j + bounds.sample(rng);
            vec.swap(n * j + i, n * other + i);
        }
    }

    vec
}
