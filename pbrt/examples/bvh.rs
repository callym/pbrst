#![feature(nll, underscore_imports)]
extern crate cgmath;
extern crate pbrt;

use std::sync::{ Arc, Mutex };

use cgmath::{ Deg, Matrix4 };

use pbrt::prelude::*;

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
    let bvh = Arc::new(bvh);

    let light = {
        let transform = Matrix4::from_translation(Vector3f::new(
            float(5.0),
            float(0.0),
            float(0.0),
        ));
        let transform = Transform::new(transform);
        let transform = Arc::new(transform);

        let light = PointLight::new(Spectrum::from_rgb([
                float(1.0),
                float(1.0),
                float(1.0),
            ], SpectrumType::Illumination) * float(10.0),
            transform);
        Box::new(light)
    };

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
        Arc::new(camera)
    };

    let scene = Scene::new(bvh, vec![light]);

    let sampler = StratifiedSampler::new(
        4,
        4,
        true,
        2,
        123456789,
    );
    let sampler = Box::new(sampler);

//    let mut integrator = WhittedIntegrator::new(2, camera, sampler);
//    let mut integrator = DirectLightingIntegrator::new(2, LightStrategy::UniformSampleAll, camera, sampler);
    let mut integrator = NormalIntegrator::new(camera, sampler);

    integrator.render(scene);
}
