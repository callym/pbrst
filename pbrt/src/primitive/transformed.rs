use std::sync::Arc;
use prelude::*;
use bxdf::TransportMode;
use super::Primitive;
use interaction::SurfaceInteraction;
use light::Light;
use material::Material;
use math::transform::AnimatedTransform;

#[derive(Clone, Debug)]
pub struct TransformedPrimitive {
    pub primitive: Arc<Primitive + Send + Sync>,
    pub primitive_to_world: Arc<AnimatedTransform>,
}

impl Primitive for TransformedPrimitive {
    fn intersect<'a>(&'a self, ray: &mut Ray) -> Option<SurfaceInteraction<'a>> {
        let interpolated = self.primitive_to_world.interpolate(ray.time);
        let mut i_ray = interpolated.inverse().transform_ray(*ray);

        if let Some(mut isect) = self.primitive.intersect(&mut i_ray) {
            ray.max = i_ray.max;

            if !interpolated.is_identity() {
                isect = interpolated.transform_surface_interaction(&isect);
            }

            Some(isect)
        } else {
            None
        }
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        let interpolated = self.primitive_to_world.interpolate(ray.time);
        let ray = interpolated.inverse().transform_ray(*ray);

        self.primitive.intersect_p(&ray)
    }

    fn world_bound(&self) -> Bounds3<Float> {
        self.primitive_to_world.motion_bounds(self.primitive.world_bound())
    }

    fn get_area_light(&self) -> Option<Arc<Light + Send + Sync>> {
        panic!("TransformedPrimitive::get_area_light should never be called")
    }

    fn get_material(&self) -> Option<&(Material + Send + Sync)> {
        panic!("TransformedPrimitive::get_material should never be called")
    }

    fn compute_scattering_functions<'a>(&'a self, _: SurfaceInteraction<'a>, _: &(), _: TransportMode, _: bool) -> SurfaceInteraction<'a> {
        panic!("TransformedPrimitive::compute_scattering_functions should never be called");

    }
}
