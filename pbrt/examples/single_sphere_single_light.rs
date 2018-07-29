#![feature(nll, underscore_imports)]
extern crate cgmath as cg;
extern crate pbrt;

use std::sync::{ Arc, Mutex };

use cg::{ Deg, Matrix4 };

use pbrt::prelude::*;

use pbrt::camera::PerspectiveCamera;
use pbrt::film::Film;
use pbrt::filter::TriangleFilter;
use pbrt::integrator::{ Integrator, WhittedIntegrator };
use pbrt::light::{ Light, PointLight };
use pbrt::material::MatteMaterial;
use pbrt::math::{ AnimatedTransform, Transform };
use pbrt::primitive::GeometricPrimitive;
use pbrt::sampler::StratifiedSampler;
use pbrt::scene::Scene;
use pbrt::shape::{ ShapeData, Sphere };
use pbrt::spectrum::{ SpectrumType, Spectrum as PbrtSpectrum };
use pbrt::texture::ConstantTexture;

fn main() {
    let transform = {
        let transform = Matrix4::from_translation(Vector3f::new(
            float(0.0),
            float(-1.0),
            float(0.0),
        ));

        Arc::new(Transform::new(transform))
    };

    let data = ShapeData::new(transform, false);
    let radius = float(1.0);

    let sphere = Sphere::new(radius, -radius, radius, Deg(float(360.0)), data);
    let sphere = Arc::new(sphere);

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

    let primitive = GeometricPrimitive {
        shape: sphere,
        material: Some(material),
        area_light: None,
        medium_interface: None,
    };
    let primitive = Arc::new(primitive);

    let light = {
        let transform = Matrix4::from_translation(Vector3f::new(
            float(5.0),
            float(3.0),
            float(4.0),
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
            float(5.0),
        ));
        let transform = Transform::new(transform);
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

    let scene = Scene::new(primitive, vec![light]);

    let sampler = StratifiedSampler::new(
        4,
        4,
        true,
        2,
        123456789,
    );
    let sampler = Box::new(sampler);

    let mut integrator = WhittedIntegrator::new(2, camera, sampler);

    integrator.render(scene);
}
