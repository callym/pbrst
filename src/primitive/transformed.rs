use std::sync::Arc;
use prelude::*;
use super::Primitive;
use shape::Shape;
use interaction::SurfaceInteraction;
use math::transform::AnimatedTransform;

#[derive(Clone, Debug)]
pub struct TransformedPrimitive {
    pub primitive: Arc<Primitive>,
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

    fn get_area_light(&self) -> Option<Arc<()>> {
        panic!("TransformedPrimitive::get_area_light should never be called")
    }

    fn get_material(&self) -> Option<Arc<()>> {
        panic!("TransformedPrimitive::get_material should never be called")
    }

    fn compute_scattering_functions(&self, isect: SurfaceInteraction, arena: (), mode: (), allow_multiple_lobes: bool) {
        panic!("TransformedPrimitive::compute_scattering_functions should never be called")
    }
}
