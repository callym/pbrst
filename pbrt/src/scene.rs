use std::sync::Arc;
use crate::prelude::*;
use crate::light::Light;
use crate::math::*;
use crate::primitive::Primitive;
use crate::interaction::SurfaceInteraction;

pub struct Scene {
    pub lights: Vec<Arc<dyn Light + Send + Sync>>,
    aggregate: Arc<dyn Primitive + Send + Sync>,
    world_bound: Bounds3<Float>,
}

impl Scene {
    pub fn new(aggregate: Arc<dyn Primitive + Send + Sync>, mut lights: Vec<Box<dyn Light + Send + Sync>>) -> Self {
        let mut scene = Self {
            world_bound: aggregate.world_bound(),
            lights: vec![],
            aggregate,
        };

        for light in &mut lights {
            light.preprocess(&scene);
        }

        let lights = lights.into_iter()
            .map(|l| l.into())
            .collect();

        scene.lights = lights;

        scene
    }

    pub fn world_bound(&self) -> &Bounds3<Float> {
        &self.world_bound
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction<'_>> {
        assert_ne!(ray.direction, Vector3f::new(float(0), float(0), float(0)));
        self.aggregate.intersect(ray)
    }

    pub fn intersect_p(&self, ray: &Ray) -> bool {
        self.aggregate.intersect_p(ray)
    }
}
