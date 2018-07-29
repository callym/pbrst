use cg;
use cg::{ Rad, Angle, Matrix4 };
use cg::prelude::*;
use prelude::*;
use math::transform::Transform;
#[macro_use] use super::*;

pub struct PerspectiveCamera {
    camera_to_world: AnimatedTransform,
    camera_to_screen: Transform,
    screen_to_raster: Transform,
    raster_to_camera: Transform,
    camera_dx: Vector3f,
    camera_dy: Vector3f,
    screen_window: Bounds2f,
    shutter_open: Float,
    shutter_close: Float,
    lens_radius: Float,
    focal_distance: Float,
    fov: Rad<Float>,
    film: Arc<Mutex<Film>>,
    medium: Option<()>,
}

impl PerspectiveCamera {
    pub fn new(
        camera_to_world: AnimatedTransform,
        screen_window: Bounds2f,
        shutter_open: Float,
        shutter_close: Float,
        lens_radius: Float,
        focal_distance: Float,
        fov: impl Into<Rad<Float>> + Copy,
        film: Arc<Mutex<Film>>,
        medium: Option<()>,
    ) -> Self {
        let full_resolution = {
            let film = film.lock().unwrap();
            film.full_resolution
        };

        let camera_to_screen = Transform::new(
            cg::perspective(
                fov,
                float(full_resolution.x as f32) / float(full_resolution.y as f32),
                float(1e-2),
                float(1000.0)));

        let (screen_to_raster, raster_to_camera) = projective_camera!(
            screen_window,
            film,
            camera_to_screen,
        );

        let camera_dx = raster_to_camera.transform_point(Point3f::new(float(1.0), float(0.0), float(0.0))) -
                        raster_to_camera.transform_point(Point3f::zero());

        let camera_dy = raster_to_camera.transform_point(Point3f::new(float(0.0), float(1.0), float(0.0))) -
                        raster_to_camera.transform_point(Point3f::zero());

        // todo: compute image plate bounds at z=1

        Self {
            camera_to_world,
            camera_to_screen,
            screen_to_raster,
            raster_to_camera,
            camera_dx,
            camera_dy,
            screen_window,
            shutter_open,
            shutter_close,
            lens_radius,
            focal_distance,
            fov: fov.into(),
            film,
            medium,
        }
    }

    fn generate_ray_not_transformed(&self, camera_sample: &CameraSample) -> Ray {
        // compute raster and cam sam pos
        let p_film = Point3f::new(camera_sample.film.x, camera_sample.film.y, float(0.0));
        let p_camera = self.raster_to_camera.transform_point(p_film);

        let mut ray = Ray::new(Point3f::zero(), p_camera.into_vector().normalize());

        // todo: modify ray for DoF

        ray.time = self.shutter_open.lerp(self.shutter_close, camera_sample.time);
        ray.medium = self.medium;

        ray
    }
}

impl Camera for PerspectiveCamera {
    fn film(&self) -> Arc<Mutex<Film>> {
        self.film.clone()
    }

    fn generate_ray(&self, camera_sample: &CameraSample) -> (Float, Ray) {
        let ray = self.generate_ray_not_transformed(camera_sample);
        let ray = self.camera_to_world.transform_ray(ray.time, ray);

        (float(1.0), ray)
    }

    fn generate_ray_differential(&self, camera_sample: &CameraSample) -> (Float, RayDifferential) {
        let p_film = Point3f::new(camera_sample.film.x, camera_sample.film.y, float(0.0));
        let p_camera = self.raster_to_camera.transform_point(p_film);
        let p_camera = p_camera.into_vector().normalize();

        let ray = self.generate_ray_not_transformed(camera_sample);
        let mut ray = RayDifferential::from_ray(ray);

        if self.lens_radius > 0.0 {
            // todo: modify ray for DoF
            unimplemented!()
        } else {
            ray.x = Some(RayData::new(ray.origin, p_camera + self.camera_dx));
            ray.y = Some(RayData::new(ray.origin, p_camera + self.camera_dy));
        }

        let ray = self.camera_to_world.transform_ray_differential(ray.time, ray);

        (float(1.0), ray)
    }
}
