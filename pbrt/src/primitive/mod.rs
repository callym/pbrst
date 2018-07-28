use std::fmt::Debug;
use std::sync::Arc;
use prelude::*;
use bxdf::{ Bsdf, TransportMode };
use material::Material;
use math::*;
use interaction::SurfaceInteraction;

pub mod geometric;
pub use self::geometric::GeometricPrimitive;

pub mod transformed;
pub use self::transformed::TransformedPrimitive;

pub trait Primitive: Debug {
    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction>;

    fn intersect_p(&self, ray: &Ray) -> bool;

    fn world_bound(&self) -> Bounds3<Float>;

    fn get_area_light(&self) -> Option<Arc<()>>;

    fn get_material(&self) -> Option<&Box<Material>>;

    fn compute_scattering_functions<'a>(&'a self, isect: SurfaceInteraction<'a>, arena: &(), mode: TransportMode, allow_multiple_lobes: bool) -> SurfaceInteraction<'a>;
}
