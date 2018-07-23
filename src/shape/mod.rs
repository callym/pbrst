use std::fmt::Debug;
use std::sync::Arc;
use prelude::*;
use math::*;
use math::transform::Transform;
use interaction::SurfaceInteraction;

pub mod sphere;
pub use self::sphere::*;

#[derive(Clone, Debug)]
pub struct ShapeData {
    pub object_to_world: Arc<Transform>,
    pub world_to_object: Arc<Transform>,
    pub reverse_orientation: bool,
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

    fn intersect<'a>(&'a self, ray: &Ray, test_alpha_texture: bool) -> Option<(Float, SurfaceInteraction<'a>)>;

    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        self.intersect(ray, test_alpha_texture).is_some()
    }

    fn area(&self) -> Float;
}
