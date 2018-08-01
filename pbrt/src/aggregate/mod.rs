use std::sync::Arc;
use crate::bxdf::TransportMode;
use crate::primitive::Primitive;
use crate::interaction::SurfaceInteraction;
use crate::light::Light;
use crate::material::Material;

mod bvh;
pub use self::bvh::{ BvhAccel, SplitMethod };

pub trait Aggregate: Primitive { }

default impl<T> Primitive for T where T: Aggregate {
    default fn get_area_light(&self) -> Option<Arc<dyn Light + Send + Sync>> {
        panic!("Aggregate::get_area_light should never be called")
    }

    default fn get_material(&self) -> Option<&(dyn Material + Send + Sync)> {
        panic!("Aggregate::get_material should never be called")
    }

    default fn compute_scattering_functions(&'a self, _: SurfaceInteraction<'a>, _: &(), _: TransportMode, _: bool) -> SurfaceInteraction<'a> {
        panic!("Aggregate::compute_scattering_functions should never be called");
    }
}
