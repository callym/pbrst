use std::sync::Arc;
use prelude::*;
use light::Light;
use math::*;
use primitive::Primitive;
use interaction::SurfaceInteraction;

pub struct Scene {
    pub lights: Arc<Vec<Box<Light>>>,
    aggregate: Arc<Primitive>,
    world_bound: Bounds3<Float>,
}

impl Scene {
    pub fn new(aggregate: Arc<Primitive>, mut lights: Vec<Box<Light>>) -> Self {
        let mut scene = Self {
            world_bound: aggregate.world_bound(),
            lights: Arc::new(vec![]),
            aggregate,
        };

        for light in &mut lights {
            light.preprocess(&scene);
        }

        scene.lights = Arc::new(lights);

        scene
    }

    pub fn world_bound(&self) -> &Bounds3<Float> {
        &self.world_bound
    }

    pub fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction> {
        self.aggregate.intersect(ray)
    }

    pub fn intersect_p(&self, ray: &Ray) -> bool {
        self.aggregate.intersect_p(ray)
    }
}
