use std::fmt::Debug;
use std::sync::Arc;
use crate::prelude::*;
use crate::math::*;
use crate::math::Transform;
use crate::interaction::SurfaceInteraction;

mod cylinder;
pub use self::cylinder::Cylinder;

mod sphere;
pub use self::sphere::Sphere;

#[derive(Clone, Debug)]
pub struct ShapeData {
    pub object_to_world: Arc<Transform>,
    pub world_to_object: Arc<Transform>,
    pub reverse_orientation: bool,
}

impl ShapeData {
    pub fn new(object_to_world: Arc<Transform>, reverse_orientation: bool) -> Self {
        Self {
            world_to_object: Arc::new(object_to_world.inverse()),
            object_to_world,
            reverse_orientation,
        }
    }
}

pub trait Shape: Debug {
    fn data(&self) -> &ShapeData;

    fn object_to_world(&self) -> &Arc<Transform> {
        &self.data().object_to_world
    }

    fn world_to_object(&self) -> &Arc<Transform> {
        &self.data().world_to_object
    }

    fn reverse_orientation(&self) -> bool {
        self.data().reverse_orientation
    }

    fn transform_swaps_handedness(&self) -> bool {
        self.object_to_world().swaps_handedness()
    }

    fn object_bounds(&self) -> Bounds3f;

    fn world_bound(&self) -> Bounds3f {
        self.object_to_world().transform_bounds(self.object_bounds())
    }

    /// If the `Ray` intersects, returns both the distance and the `SurfaceInteraction`.
    fn intersect(&'a self, ray: &Ray, test_alpha_texture: bool) -> Option<(Float, SurfaceInteraction<'a>)>;

    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        self.intersect(ray, test_alpha_texture).is_some()
    }

    fn area(&self) -> Float;
}
