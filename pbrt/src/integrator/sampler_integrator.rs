use std::cmp;
use std::sync::Arc;
use rayon::prelude::*;

use crate::prelude::*;
use crate::camera::Camera;
use crate::math::*;
use crate::sampler::Sampler;
use crate::scene::Scene;
use super::Integrator;

pub trait ParIntegratorData: Send {
    fn li(&self, ray: RayDifferential, scene: &Scene, sampler: &mut dyn Sampler, arena: &(), depth: i32) -> Spectrum;
}

pub trait SamplerIntegrator: Integrator {
    type ParIntegratorData: ParIntegratorData;

    fn camera(&self) -> Arc<dyn Camera + Send + Sync>;
    fn sampler(&self) -> &dyn Sampler;
    fn sampler_mut(&mut self) -> &mut dyn Sampler;

    fn par_data(&self) -> Self::ParIntegratorData;

    fn preprocess(&mut self, _scene: &Scene, _sampler: &mut dyn Sampler) {

    }

    fn render(&mut self, scene: Arc<Scene>) {
        const TILE_SIZE: i32 = 16;

        <Self as SamplerIntegrator>::preprocess(self, &*scene, self.sampler().create_new(0).as_mut());

        let sample_bounds = {
            let camera = self.camera();
            let film = camera.film();
            let film = film.lock().unwrap();
            film.sample_bounds()
        };
        let sample_extent = sample_bounds.diagonal();

        let num_tiles = Point2i::new(
            (sample_extent.x + TILE_SIZE - 1) / TILE_SIZE,
            (sample_extent.y + TILE_SIZE - 1) / TILE_SIZE
        );

        println!("{} tiles to render", num_tiles.x * num_tiles.y);

        let num_tiles = (0..num_tiles.x).into_iter()
            .map(|x| {
                (0..num_tiles.y).map(|y| (x, y)).collect::<Vec<_>>()
            })
            .flatten()
            .map(|(x, y)| (
                x, y,
                scene.clone(),
                self.sampler().create_new(y * num_tiles.x + x),
                self.camera().clone(),
                self.par_data(),
            )).collect::<Vec<_>>();

        num_tiles.into_par_iter()
            .for_each(|(x, y, scene, mut tile_sampler, camera, p_self)| {
                let tile = Point2i::new(x, y);

                // allocate MemoryArena for tile
                let arena = ();

                // compute sample bounds for tile
                let x0 = sample_bounds.min.x + tile.x * TILE_SIZE;
                let x1 = cmp::min(x0 + TILE_SIZE, sample_bounds.max.x);

                let y0 = sample_bounds.min.y + tile.y * TILE_SIZE;
                let y1 = cmp::min(y0 + TILE_SIZE, sample_bounds.max.y);

                let tile_bounds = Bounds2::new(Point2i::new(x0, y0), Point2i::new(x1, y1));

                // get FilmTile for tile
                let mut film_tile = {
                    let film = camera.film();
                    let film = film.lock().unwrap();
                    film.film_tile(&tile_bounds)
                };

                // loop over pixels in tile to render them
                for x in tile_bounds.min.x..tile_bounds.max.x {
                    for y in tile_bounds.min.y..tile_bounds.max.y {
                        let pixel = Point2i::new(x, y);
                        tile_sampler.start_pixel(pixel);

                        while tile_sampler.start_next_sample() {
                            // initialize CameraSample for current sample
                            let camera_sample = tile_sampler.get_camera_sample(pixel);

                            // generate camera ray for current sample
                            let (ray_weight, mut ray) = camera.generate_ray_differential(&camera_sample);
                            ray.scale_differentials(float(1.0 / (tile_sampler.samples_per_pixel() as FloatPrim).sqrt()));

                            // evaluate radiance along camera ray
                            let mut l = if ray_weight > 0.0 {
                                p_self.li(ray, &*scene, tile_sampler.as_mut(), &arena, 0)
                            } else {
                                Spectrum::new(0.0)
                            };

                            // if l is negative or infinite
                            if l.y() < -10e-5 || l.y().is_infinite() {
                                //l = Spectrum::new(0.0);
                            }

                            // add camera ray's contribution to image
                            film_tile.add_sample(camera_sample.film, l, ray_weight);

                            // free MemoryArena memory from computing image sample value
                            // arena.reset();
                        }
                    }
                }

                // merge image tile into Film
                {
                    let film = camera.film();
                    let mut film = film.lock().unwrap();
                    film.merge_film_tile(&film_tile);
                };
            });

        {
            let camera = self.camera();
            let film = camera.film();
            let film = film.lock().unwrap();
            film.write_image(float(1.0));
        }
    }
}

impl<T: SamplerIntegrator> Integrator for T {
    fn render(&mut self, scene: Scene) {
        <Self as SamplerIntegrator>::render(self, Arc::new(scene));
    }

    fn preprocess(&mut self, scene: &Scene, sampler: &mut dyn Sampler) {
        <Self as SamplerIntegrator>::preprocess(self, scene, sampler);
    }
}
