use cg;
use cg::prelude::*;
use prelude::*;
use math::transform::Transform;
#[macro_use] use super::*;

pub struct OrthographicCamera {
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
    film: Arc<Film>,
    medium: Option<()>,
}

impl OrthographicCamera {
    pub fn new(
        camera_to_world: AnimatedTransform,
        screen_window: Bounds2f,
        shutter_open: Float,
        shutter_close: Float,
        lens_radius: Float,
        focal_distance: Float,
        film: Arc<Film>,
        medium: Option<()>,
    ) -> Self {
        let camera_to_screen = Transform::identity();

        let (screen_to_raster, raster_to_camera) = projective_camera!(
            screen_window,
            film,
            camera_to_screen,
        );

        let camera_dx = raster_to_camera.transform_vector(Vector3f::new(float(1.0), float(0.0), float(0.0)));
        let camera_dy = raster_to_camera.transform_vector(Vector3f::new(float(0.0), float(1.0), float(0.0)));

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
            film,
            medium,
        }
    }

    fn generate_ray_not_transformed(&self, camera_sample: &CameraSample) -> Ray {
        // compute raster and cam sam pos
        let p_film = Point3f::new(camera_sample.film.x, camera_sample.film.y, float(0.0));
        let p_camera = self.raster_to_camera.transform_point(p_film);

        let mut ray = Ray::new(p_camera, Vector3f::new(float(0.0), float(0.0), float(1.0)));

        // todo: modify ray for DoF

        ray.time = self.shutter_open.lerp(self.shutter_close, camera_sample.time);
        ray.medium = self.medium;

        ray
    }
}

impl Camera for OrthographicCamera {
    fn film(&self) -> Arc<Film> {
        self.film.clone()
    }

    fn generate_ray(&self, camera_sample: &CameraSample) -> (Float, Ray) {
        let ray = self.generate_ray_not_transformed(camera_sample);
        let ray = self.camera_to_world.transform_ray(ray.time, ray);

        (float(1.0), ray)
    }

    fn generate_ray_differential(&self, camera_sample: &CameraSample) -> (Float, RayDifferential) {
        let ray = self.generate_ray_not_transformed(camera_sample);
        let mut ray = RayDifferential::from_ray(ray);

        if self.lens_radius > 0.0 {
            // todo: modify ray for DoF
            unimplemented!()
        } else {
            ray.x = Some(RayData::new(ray.origin + self.camera_dx, ray.direction));
            ray.y = Some(RayData::new(ray.origin + self.camera_dy, ray.direction));
        }

        let ray = self.camera_to_world.transform_ray_differential(ray.time, ray);

        (float(1.0), ray)
    }
}
