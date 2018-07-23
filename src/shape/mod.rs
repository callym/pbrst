use std::fmt::Debug;
use std::sync::Arc;
use prelude::*;
use math::*;
use math::transform::Transform;
use interaction::SurfaceInteraction;

pub trait Shape: Debug {
    fn transform(&self) -> Arc<Transform>;

    fn reverse_orientation(&self) -> bool;

    fn transform_swaps_handedness(&self) -> bool {
        self.transform().swaps_handedness()
    }

    fn object_bound(&self) -> Bounds3f;

    fn world_bound(&self) -> Bounds3f {
        // OBJECT TO WORLD TRANSFORM NEEDED!
        self.object_bound()
    }

    fn intersect(&self, ray: &Ray, test_alpha_texture: bool) -> Option<(Float, SurfaceInteraction)>;

    fn intersect_p(&self, ray: &Ray, test_alpha_texture: bool) -> bool {
        self.intersect(ray, test_alpha_texture).is_some()
    }

    fn area(&self) -> Float;
}
