#![feature(nll, underscore_imports)]
extern crate cgmath;
extern crate pbrt;

use std::sync::{ Arc, Mutex };

use cgmath::{ Deg, Matrix4 };

use pbrt::aggregate::{ BvhAccel, SplitMethod };
use pbrt::camera::{ OrthographicCamera, PerspectiveCamera };
use pbrt::film::Film;
use pbrt::filter::TriangleFilter;
use pbrt::integrator::{ Integrator, DirectLightingIntegrator, NormalIntegrator, WhittedIntegrator };
use pbrt::integrator::LightStrategy;
use pbrt::light::{ Light, PointLight };
use pbrt::material::{ Material, MatteMaterial };
use pbrt::math::{ AnimatedTransform, Transform };
use pbrt::primitive::{ Primitive, GeometricPrimitive };
use pbrt::sampler::StratifiedSampler;
use pbrt::scene::Scene;
use pbrt::shape::{ ShapeData, Cylinder, Sphere };
use pbrt::spectrum::{ SpectrumType, Spectrum as PbrtSpectrum };
use pbrt::texture::ConstantTexture;

use pbrt::prelude::*;
use pbrt::camera::*;
use pbrt::sampler::*;
use pbrt::shape::*;

fn ring(transform: Arc<Transform>, material: Box<impl Material + Clone + Send + Sync + 'static>, radius: Float, height: Float) -> (Arc<Primitive + Send>, Arc<Primitive + Send>) {
    let data = ShapeData::new(transform.clone(), false);
    let sphere = Sphere::new(radius, -height, height, Deg(float(360.0)), data);
    let sphere = Arc::new(sphere);

    let primitive0 = GeometricPrimitive {
        shape: sphere,
        material: Some(material.clone()),
        area_light: None,
        medium_interface: None,
    };
    let primitive0 = Arc::new(primitive0);

    let data = ShapeData::new(transform, true);
    let sphere = Sphere::new(radius - float(0.01), -height, height, Deg(float(360.0)), data);
    let sphere = Arc::new(sphere);

    let primitive1 = GeometricPrimitive {
        shape: sphere,
        material: Some(material),
        area_light: None,
        medium_interface: None,
    };
    let primitive1 = Arc::new(primitive1);

    (primitive0, primitive1)
}

fn main() {
    quadratic();
    return;
    let sphere = {
        let sphere = {
            let transform = {
                let transform = Matrix4::from_translation(Vector3f::new(
                    float(0.0),
                    float(0.0),
                    float(0.0),
                ));
                let rot = Matrix4::from_angle_x(Deg(float(0.0)));

                Arc::new(Transform::new(transform * rot))
            };

            let data = ShapeData::new(transform, false);
            let radius = float(0.5);

            let sphere = Sphere::new(radius, -radius, radius, Deg(float(360.0)), data);
            Arc::new(sphere)
        };

        let material = {
            let kd = ConstantTexture::new(Spectrum::from_rgb([
                    float(0.3),
                    float(0.5),
                    float(0.7),
                ],
                SpectrumType::Reflectance));
            let kd = Arc::new(kd);

            let sigma = ConstantTexture::new(0.0);
            let sigma = Arc::new(sigma);

            MatteMaterial::new(kd, sigma, None)
        };
        let material = Box::new(material);

        let primitive = GeometricPrimitive {
            shape: sphere,
            material: Some(material),
            area_light: None,
            medium_interface: None,
        };
        Arc::new(primitive)
    };

    let transform = {
        let transform = Matrix4::from_translation(Vector3f::new(
            float(0.0),
            float(0.0),
            float(0.0),
        ));
        let rot = Matrix4::from_angle_x(Deg(float(60.0)));

        Arc::new(Transform::new(transform * rot))
    };

    let material = {
        let kd = ConstantTexture::new(Spectrum::from_rgb([
                float(0.7),
                float(0.5),
                float(0.3),
            ],
            SpectrumType::Reflectance));
        let kd = Arc::new(kd);

        let sigma = ConstantTexture::new(0.0);
        let sigma = Arc::new(sigma);

        MatteMaterial::new(kd, sigma, None)
    };
    let material = Box::new(material);

    let sphere_2 = ring(transform, material, float(1.0), float(0.05));

    let transform = {
        let transform = Matrix4::from_translation(Vector3f::new(
            float(0.0),
            float(0.0),
            float(0.0),
        ));
        let rot = Matrix4::from_angle_x(Deg(float(100.0)));

        Arc::new(Transform::new(transform * rot))
    };

    let material = {
        let kd = ConstantTexture::new(Spectrum::from_rgb([
                float(0.7),
                float(0.3),
                float(0.5),
            ],
            SpectrumType::Reflectance));
        let kd = Arc::new(kd);

        let sigma = ConstantTexture::new(0.0);
        let sigma = Arc::new(sigma);

        MatteMaterial::new(kd, sigma, None)
    };
    let material = Box::new(material);

    let sphere_3 = ring(transform, material, float(0.8), float(0.05));

    let bvh = BvhAccel::new(
        vec![
            sphere,
            sphere_2.0,
//            sphere_2.1,
            sphere_3.0,
//            sphere_3.1,
        ],
        SplitMethod::HLBVH,
    );

    let camera = {
        let film = {
            let full_resolution = Point2i::new(512, 512);
            let crop_window = Bounds2f::new(
                Point2f::new(float(0.0), float(0.0)),
                Point2f::new(float(1.0), float(1.0)),
            );

            let filter = TriangleFilter::new(Vector2f::new(float(1.0), float(1.0)));
            let filter = Box::new(filter);

            let film = Film::new(
                full_resolution,
                crop_window,
                filter,
                float(5.0),
                float(1.0),
                String::from("test_image"),
            );
            Arc::new(Mutex::new(film))
        };

        let transform = Matrix4::from_translation(Vector3f::new(
            float(0.0),
            float(0.0),
            float(3.0),
        ));
        let rot = Matrix4::from_angle_y(Deg(float(0.0)));
        let transform = Transform::new(transform * rot);
        let transform = Arc::new(transform);
        let transform = AnimatedTransform::new(
            transform.clone(),
            float(0.0),
            transform.clone(),
            float(1.0),
        );

        let screen_window = Bounds2f::new(
            Point2f::new(float(-1.0), float(-1.0)),
            Point2f::new(float(1.0), float(1.0)),
        );

        let camera = PerspectiveCamera::new(
            transform,
            screen_window,
            float(0.0),
            float(1.0),
            float(0.0),
            float(1.0),
            Deg(float(45.0)),
            film,
            None
        );
        camera
    };

/*
CameraSample {
    p_film: Point2 { x: 375.72485, y: 240.22925 },
    p_lens: Point2 { x: 0.5703919, y: 0.93715453 },
    time: 0.91514564
}
*/

    let sample = CameraSample {
        lens: Point2f::new(
            float(0.5703919),
            float(0.93715453)
        ),
        film: Point2f::new(
            float(375.72485),
            float(240.22925)
        ),
        time: float(0.91514564),
    };

    let (weight, mut ray) = camera.generate_ray_differential(&sample);

    if let Some(si) = bvh.intersect(&mut ray) {
        println!("si == {:#?}", si);
    }

    println!("weight == {:?}", weight);
    println!("ray == {:#?}", ray);

    let t = Matrix4::from_translation(Vector3f {
        x: float(-1.3),
        y: float(0.0),
        z: float(0.0),
    });
    let t = Transform::new(t);
    let o: Point3f = Point3f {
        x: float(2.0),
        y: float(1.99999988),
        z: float(4.99999905),
    };
    let d: Vector3f = Vector3f {
        x: float(-0.0607556403),
        y: float(-0.164096087),
        z: float(-0.984571517),
    };
    let r: Ray = Ray {
        origin: o,
        direction: d,
        max: Float::infinity(),
        time: float(0.0),
        medium: None,
    };

    let (r, o, d) = t.transform_ray_with_error(r);

    println!("{:#?}", r);
    println!("{:?}", o);
    println!("{:?}", d);
}

fn quadratic() {
    let sphere = {
        let transform = {
            let transform = Matrix4::from_translation(Vector3f::new(
                float(0.0),
                float(0.0),
                float(0.0),
            ));
            let rot = Matrix4::from_angle_x(Deg(float(0.0)));

            Arc::new(Transform::new(transform * rot))
        };

        let data = ShapeData::new(transform, false);
        let radius = float(1.0);

        let sphere = Sphere::new(radius, -radius, float(0.0), Deg(float(360.0)), data);
        sphere
    };

    let ray = Ray {
        origin: Point3f::new(
            float(0.0),
            float(0.0),
            float(3.0),
        ),
        direction: Vector3f::new(
            float(0.0),
            float(0.0),
            float(-1.0),
        ),
        max: Float::infinity(),
        time: float(0.0),
        medium: None,
    };

    if let Some((f, si)) = sphere.intersect(&ray, true) {
        println!("f == {:?}", f);
        println!("si == {:#?}", si);
    }
}
