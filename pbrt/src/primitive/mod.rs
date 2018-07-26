use std::fmt::Debug;
use std::sync::Arc;
use prelude::*;
use math::*;
use interaction::SurfaceInteraction;

mod geometric;
mod transformed;

pub trait Primitive: Debug {
    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction>;

    fn intersect_p(&self, ray: &Ray) -> bool;

    fn world_bound(&self) -> Bounds3<Float>;

    fn get_area_light(&self) -> Option<Arc<()>>;

    fn get_material(&self) -> Option<Arc<()>>;

    fn compute_scattering_functions(&self, isect: SurfaceInteraction, arena: (), mode: (), allow_multiple_lobes: bool);
}
