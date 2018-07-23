use prelude::*;
use math::*;
use interaction::SurfaceInteraction;

pub trait Primitive {
    fn intersect(&self, ray: &Ray) -> Option<SurfaceInteraction>;

    fn intersect_p(&self, ray: &Ray) -> bool;

    fn world_bound(&self) -> &Bounds3<Float>;
}
