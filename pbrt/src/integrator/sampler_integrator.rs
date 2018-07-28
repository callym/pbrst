use std::cmp;
use std::sync::Arc;
use cg::Point2;
use num_cpus;
use rayon::prelude::*;

use prelude::*;
use camera::Camera;
use math::*;
use sampler::Sampler;
use scene::Scene;
use super::Integrator;

pub trait ParIntegratorData: Send {
    fn li(&self, ray: RayDifferential, scene: &Scene, sampler: &mut Box<Sampler + Send>, arena: &(), depth: i32) -> Spectrum;
}

pub trait SamplerIntegrator: Integrator {
    type ParIntegratorData: ParIntegratorData;

    fn camera(&self) -> Arc<Camera + Send + Sync>;
    fn sampler<'a>(&'a self) -> &Box<Sampler + 'static>;
    fn sampler_mut<'a>(&'a mut self) -> &mut Box<Sampler + 'static>;

    fn par_data(&self) -> Self::ParIntegratorData;

    fn render(&mut self, scene: Arc<Scene>) {
        const TILE_SIZE: i32 = 16;

        self.preprocess(&*scene, &mut self.sampler().create_new(0));

        let sample_bounds = {
            let camera = self.camera();
            let film = camera.film();
            let film = film.lock().unwrap();
            film.sample_bounds()
        };
        let sample_extent = sample_bounds.diagonal();

        let num_tiles = Point2::new(
            (sample_extent.x + TILE_SIZE - 1) / TILE_SIZE,
            (sample_extent.y + TILE_SIZE - 1) / TILE_SIZE
        );

        let num_cores = num_cpus::get();

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
                let tile = Point2::new(x, y);

                // allocate MemoryArena for tile
                let arena = ();

                // compute sample bounds for tile
                let x0 = sample_bounds.min.x + tile.x * TILE_SIZE;
                let x1 = cmp::min(x0 + TILE_SIZE, sample_bounds.max.x);

                let y0 = sample_bounds.min.y + tile.y * TILE_SIZE;
                let y1 = cmp::min(y0 + TILE_SIZE, sample_bounds.max.y);

                let tile_bounds = Bounds2::new(Point2::new(x0, y0), Point2::new(x1, y1));

                // get FilmTile for tile
                let mut film_tile = {
                    let film = camera.film();
                    let film = film.lock().unwrap();
                    film.film_tile(&tile_bounds)
                };

                // loop over pixels in tile to render them
                for x in tile_bounds.min.x..tile_bounds.max.x {
                    for y in tile_bounds.min.y..tile_bounds.max.y {
                        let pixel = Point2::new(x, y);
                        tile_sampler.start_pixel(&pixel);

                        while tile_sampler.start_next_sample() {
                            // initialize CameraSample for current sample
                            let camera_sample = tile_sampler.get_camera_sample(&pixel);

                            // generate camera ray for current sample
                            let (ray_weight, mut ray) = camera.generate_ray_differential(&camera_sample);
                            ray.scale_differentials(float(1.0 / (tile_sampler.samples_per_pixel() as FloatPrim).sqrt()));

                            // evaluate radiance along camera ray
                            let mut l = Spectrum::new(0.0);
                            if ray_weight > 0.0 {
                                l = p_self.li(ray, &*scene, &mut tile_sampler, &arena, 0);
                            }

                            if l.y() < -1e-5 {
                                // NEGATIVE
                                l = Spectrum::new(0.0);
                            } else if l.y().is_infinite() {
                                // INFINITE
                                l = Spectrum::new(0.0);
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
                    println!("finished tile");
                    let film = camera.film();
                    let mut film = film.lock().unwrap();
                    film.merge_film_tile(film_tile);
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
}
