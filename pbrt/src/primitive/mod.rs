use std::fmt::Debug;
use std::sync::Arc;
use bxdf::TransportMode;
use light::Light;
use material::Material;
use math::*;
use interaction::SurfaceInteraction;

mod geometric;
pub use self::geometric::GeometricPrimitive;

mod transformed;
pub use self::transformed::TransformedPrimitive;

pub trait Primitive: Debug + Send + Sync {
    fn intersect(&self, ray: &mut Ray) -> Option<SurfaceInteraction>;

    fn intersect_p(&self, ray: &Ray) -> bool;

    fn world_bound(&self) -> Bounds3<Float>;

    fn get_area_light(&self) -> Option<Arc<Light + Send + Sync>>;

    fn get_material(&self) -> Option<&(Material + Send + Sync)>;

    fn compute_scattering_functions<'a>(&'a self, isect: SurfaceInteraction<'a>, arena: &(), mode: TransportMode, allow_multiple_lobes: bool) -> SurfaceInteraction<'a>;
}
