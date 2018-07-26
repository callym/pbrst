use std::env;
use std::cmp::{ max, min };
use std::sync::{ Arc, Mutex };
use std::ops::{ Deref, DerefMut };
use atomic::{ Atomic, Ordering };
use image;
use prelude::*;
use filter::Filter;
use spectrum::utils::xyz_to_rgb;

const FILTER_TABLE_WIDTH: usize = 16;

#[repr(align(32))]
#[derive(Debug)]
pub struct Pixel {
    xyz: [Float; 3],
    filter_weight_sum: Float,
    // todo - this Atomic<Float> is probably not the best
    splat_xyz: [Atomic<Float>; 3],
}

impl Pixel {
    pub fn new() -> Self {
        Self {
            xyz: [float(0.0); 3],
            filter_weight_sum: float(0.0),
            splat_xyz: [
                Atomic::new(float(0.0)),
                Atomic::new(float(0.0)),
                Atomic::new(float(0.0)),
            ],
        }
    }
}

impl Clone for Pixel {
    fn clone(&self) -> Self {
        Self {
            xyz: self.xyz,
            filter_weight_sum: self.filter_weight_sum,
            splat_xyz: [
                Atomic::new(self.splat_xyz[0].load(Ordering::SeqCst)),
                Atomic::new(self.splat_xyz[1].load(Ordering::SeqCst)),
                Atomic::new(self.splat_xyz[2].load(Ordering::SeqCst)),
            ],
        }
    }
}

#[derive(Derivative)]
#[derivative(Debug)]
pub struct Film {
    sample_bounds: Bounds2<i32>,
    pub full_resolution: Point2i,
    pub cropped_pixel_bounds: Bounds2i,
    crop_window: Bounds2f,
    #[derivative(Debug = "ignore")]
    pub filter: Box<Filter>,
    pub diagonal: Float,
    scale: Float,
    pub filename: String,
    pixels: Mutex<Vec<Pixel>>,
    #[derivative(Debug = "ignore")]
    filter_table: Arc<[Float; FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH]>,
}

impl Film {
    pub fn new(
        sample_bounds: Bounds2<i32>,
        full_resolution: Point2i,
        crop_window: Bounds2f,
        filter: Box<Filter>,
        diagonal: Float,
        scale: Float,
        filename: String,
    ) -> Self {
        let cropped_pixel_bounds = {
            let full_resolution = full_resolution.map(float);

            Bounds2i::new(
                Point2i::new(
                    (full_resolution.x * crop_window.min.x).ceil().raw() as i32,
                    (full_resolution.y * crop_window.min.y).ceil().raw() as i32,
                ),
                Point2i::new(
                    (full_resolution.x * crop_window.max.x).ceil().raw() as i32,
                    (full_resolution.y * crop_window.max.y).ceil().raw() as i32,
                ),
            )
        };

        let pixels = vec![Pixel::new(); cropped_pixel_bounds.area() as usize];
        let pixels = Mutex::new(pixels);

        let mut filter_table = [float(0.0); FILTER_TABLE_WIDTH * FILTER_TABLE_WIDTH];
        let mut offset = 0;
        let width = float(FILTER_TABLE_WIDTH);
        for y in 0..FILTER_TABLE_WIDTH {
            for x in 0..FILTER_TABLE_WIDTH {
                let x = (float(x) + float(0.5)) * filter.radius().x / width;
                let y = (float(y) + float(0.5)) * filter.radius().y / width;
                let p = Point2f::new(x, y);
                filter_table[offset] = filter.evaluate(p);
                offset += 1;
            }
        }
        let filter_table = Arc::new(filter_table);

        Self {
            sample_bounds,
            full_resolution,
            cropped_pixel_bounds,
            crop_window,
            filter,
            // convert diagonal from mm to meters
            diagonal: diagonal * float(0.001),
            scale,
            filename,
            pixels,
            filter_table,
        }
    }

    pub fn sample_bounds(&self) -> Bounds2i {
        let bounds = Bounds2f::new(
            (self.cropped_pixel_bounds.min.map(float) + Vector2f::new(float(0.5), float(0.5)) - self.filter.radius()).floor(),
            (self.cropped_pixel_bounds.max.map(float) - Vector2f::new(float(0.5), float(0.5)) + self.filter.radius()).ceil(),
        );

        bounds.map(|f| f.raw() as i32)
    }

    pub fn physical_extent(&self) -> Bounds2f {
        let aspect = float(self.full_resolution.y) / float(self.full_resolution.x);
        let x = (self.diagonal.powi(2) / (float(1.0) + aspect * aspect)).sqrt();
        let y = aspect * x;

        Bounds2f::new(
            Point2f::new(-x / float(2.0), -y / float(2.0)),
            Point2f::new(x / float(2.0), y / float(2.0)),
        )
    }

    pub fn film_tile(&self, sample_bounds: &Bounds2i) -> FilmTile {
        // bound image pixels that samples in samplebounds contribute to
        let half_pixel = Vector2f::new(float(0.5), float(0.5));
        let float_bounds = sample_bounds.map(float);
        let p0 = (float_bounds.min - half_pixel - self.filter.radius()).ceil().map(|f| f.raw() as i32);
        let p1 = (float_bounds.max - half_pixel + self.filter.radius()).floor().map(|f| f.raw() as i32);

        let tile_pixel_bounds = Bounds2::new(p0, p1).intersect(self.cropped_pixel_bounds);

        FilmTile::new(tile_pixel_bounds, self.filter.radius(), self.filter_table.clone(), FILTER_TABLE_WIDTH)
    }

    pub fn merge_film_tile(&mut self, tile: FilmTile) {
        let mut pixels = self.pixels.lock().unwrap();

        for pixel in tile.pixel_bounds.into_iter() {
            let tile_pixel = tile.get_pixel(pixel);
            let merge_pixel = get_pixel_mut(self.cropped_pixel_bounds, &mut pixels, pixel);
            let xyz = tile_pixel.contrib_sum.to_xyz();

            for (merge, xyz) in merge_pixel.xyz.iter_mut().zip(xyz.iter()) {
                *merge += *xyz;
            }

            merge_pixel.filter_weight_sum += tile_pixel.filter_weight_sum;
        }
    }

    pub fn set_image(&mut self, spectrums: &[Spectrum]) {
        assert!(self.cropped_pixel_bounds.area() <= spectrums.len() as i32);
        let mut pixels = self.pixels.lock().unwrap();

        for i in 0..self.cropped_pixel_bounds.area() as usize {
            let pixel = &mut pixels[i];
            pixel.xyz = spectrums[i].to_xyz();
            pixel.filter_weight_sum = float(1.0);
            pixel.splat_xyz[0].store(float(0.0), Ordering::SeqCst);
            pixel.splat_xyz[1].store(float(0.0), Ordering::SeqCst);
            pixel.splat_xyz[2].store(float(0.0), Ordering::SeqCst);
        }
    }

    pub fn add_splat(&mut self, point: Point2f, v: Spectrum) {
        if !self.cropped_pixel_bounds.inside_exclusive(point.map(|f| f.raw() as i32)) {
            return;
        }

        let mut pixels = self.pixels.lock().unwrap();

        let xyz = v.to_xyz();
        let pixel = get_pixel_mut(self.cropped_pixel_bounds, &mut pixels, point.map(|f| f.raw() as i32));

        let x = pixel.splat_xyz[0].load(Ordering::SeqCst);
        let y = pixel.splat_xyz[1].load(Ordering::SeqCst);
        let z = pixel.splat_xyz[2].load(Ordering::SeqCst);

        pixel.splat_xyz[0].store(x + xyz[0], Ordering::SeqCst);
        pixel.splat_xyz[1].store(y + xyz[1], Ordering::SeqCst);
        pixel.splat_xyz[2].store(z + xyz[2], Ordering::SeqCst);
    }

    pub fn write_image(&self, splat_scale: Float) {
        let pixels = self.pixels.lock().unwrap();
        let mut rgb = vec![float(0.0); 3 * self.cropped_pixel_bounds.area() as usize];

        let mut offset = 0;
        for p in self.cropped_pixel_bounds.into_iter() {
            let pixel = get_pixel(self.cropped_pixel_bounds, &pixels, p);
            let rgb_vals = xyz_to_rgb(pixel.xyz);
            let offset_3 = offset * 3;

            rgb[offset_3] = rgb_vals[0];
            rgb[offset_3 + 1] = rgb_vals[1];
            rgb[offset_3 + 2] = rgb_vals[2];

            if pixel.filter_weight_sum != float(0.0) {
                let inv = float(1.0) / pixel.filter_weight_sum;
                rgb[offset_3] = max(float(0.0), rgb[offset_3] * inv);
                rgb[offset_3 + 1] = max(float(0.0), rgb[offset_3 + 1] * inv);
                rgb[offset_3 + 2] = max(float(0.0), rgb[offset_3 + 2] * inv);
            }

            let x = pixel.splat_xyz[0].load(Ordering::SeqCst);
            let y = pixel.splat_xyz[1].load(Ordering::SeqCst);
            let z = pixel.splat_xyz[2].load(Ordering::SeqCst);
            let splat_rgb = xyz_to_rgb([x, y, z]);

            rgb[offset_3] += splat_rgb[0] * splat_scale;
            rgb[offset_3 + 1] += splat_rgb[1] * splat_scale;
            rgb[offset_3 + 2] += splat_rgb[2] * splat_scale;

            offset += 1;
        }

        let dir = env::current_dir().unwrap();
        let path = dir.join(format!("{}.png", &self.filename));

        let buf: Vec<_> = rgb.iter().map(|p| p.raw() as u8).collect();
        let width = self.cropped_pixel_bounds.max.y - self.cropped_pixel_bounds.min.x;
        let height = self.cropped_pixel_bounds.max.y - self.cropped_pixel_bounds.min.y;

        image::save_buffer(path, &buf, width as u32, height as u32, image::RGB(8)).unwrap();
    }
}

fn get_pixel(cropped_pixel_bounds: Bounds2i, pixels: &impl Deref<Target = Vec<Pixel>>, p: Point2i) -> &Pixel {
    let width = cropped_pixel_bounds.max.x - cropped_pixel_bounds.min.x;
    let offset = (p.x - cropped_pixel_bounds.min.x) + (p.y - cropped_pixel_bounds.min.y) * width;
    &pixels[offset as usize]
}

fn get_pixel_mut(cropped_pixel_bounds: Bounds2i, pixels: &mut impl DerefMut<Target = Vec<Pixel>>, p: Point2i) -> &mut Pixel {
    let width = cropped_pixel_bounds.max.x - cropped_pixel_bounds.min.x;
    let offset = (p.x - cropped_pixel_bounds.min.x) + (p.y - cropped_pixel_bounds.min.y) * width;
    &mut pixels[offset as usize]
}

#[derive(Copy, Clone, Debug)]
pub struct FilmTilePixel {
    contrib_sum: Spectrum,
    filter_weight_sum: Float,
}

impl FilmTilePixel {
    pub fn new() -> Self {
        Self {
            contrib_sum: Spectrum::new(0.0),
            filter_weight_sum: float(0.0),
        }
    }
}

#[derive(Debug)]
pub struct FilmTile {
    pixel_bounds: Bounds2i,
    filter_radius: Vector2f,
    filter_radius_inv: Vector2f,
    filter_table: Arc<[Float]>,
    filter_table_size: usize,
    pixels: Vec<FilmTilePixel>,
}

impl FilmTile {
    pub fn new(pixel_bounds: Bounds2i, filter_radius: Vector2f, filter_table: Arc<[Float]>, size: usize) -> Self {
        let pixels = vec![FilmTilePixel::new(); max(0, pixel_bounds.area() as usize)];

        Self {
            pixel_bounds,
            filter_radius,
            filter_radius_inv: filter_radius.map(|f| float(1.0) / f),
            filter_table,
            filter_table_size: size,
            pixels,
        }
    }

    pub fn pixel_bounds(&self) -> Bounds2i {
        self.pixel_bounds
    }

    pub fn add_sample(&mut self, film_point: Point2f, l: Spectrum, sample_weight: Float) {
        // compute raster bounds
        let film_discrete = film_point - Vector2f::new(float(0.5), float(0.5));

        let p0 = (film_discrete - self.filter_radius).ceil().map(|f| f.raw() as i32);
        let p1 = (film_discrete + self.filter_radius).floor().map(|f| f.raw() as i32) + Vector2i::new(1, 1);
        let p0 = Point2i::new(max(p0.x, self.pixel_bounds.min.x), max(p0.y, self.pixel_bounds.min.y));
        let p1 = Point2i::new(min(p0.x, self.pixel_bounds.max.x), min(p0.y, self.pixel_bounds.max.y));

        // loop over filter support & add sample to pix arrays
        // precompute x and y filter table offsets
        let mut ifx = vec![0; (p1.x - p0.x) as usize];
        let mut ify = vec![0; (p1.y - p0.y) as usize];

        for x in p0.x..p1.x {
            let fx = ((float(x) - film_discrete.x) * self.filter_radius_inv.x * float(self.filter_table_size)).abs();
            ifx[(x - p0.x) as usize] = min(fx.floor().raw() as usize, self.filter_table_size - 1);
        }

        for y in p0.y..p1.y {
            let fy = ((float(y) - film_discrete.y) * self.filter_radius_inv.y * float(self.filter_table_size)).abs();
            ifx[(y - p0.y) as usize] = min(fy.floor().raw() as usize, self.filter_table_size - 1);
        }

        for y in p0.y..p1.y {
            for x in p0.x..p1.x {
                let offset = ify[y as usize - p0.y as usize] * self.filter_table_size + ifx[x as usize - p0.x as usize];
                let filter_weight = self.filter_table[offset];

                let pixel = self.get_pixel_mut(Point2i::new(x, y));
                pixel.contrib_sum += l * sample_weight * filter_weight;
                pixel.filter_weight_sum += filter_weight;
            }
        }
    }

    pub fn get_pixel(&self, p: Point2i) -> &FilmTilePixel {
        let width = self.pixel_bounds.max.x - self.pixel_bounds.min.x;
        let offset = (p.x - self.pixel_bounds.min.x) + (p.y - self.pixel_bounds.min.y) * width;
        &self.pixels[offset as usize]
    }

    fn get_pixel_mut(&mut self, p: Point2i) -> &mut FilmTilePixel {
        let width = self.pixel_bounds.max.x - self.pixel_bounds.min.x;
        let offset = (p.x - self.pixel_bounds.min.x) + (p.y - self.pixel_bounds.min.y) * width;
        &mut self.pixels[offset as usize]
    }
}
