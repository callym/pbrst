use std::sync::Arc;
use crate::prelude::*;
use crate::bxdf::TransportMode;
use super::Primitive;
use crate::light::Light;
use crate::material::Material;
use crate::shape::Shape;
use crate::interaction::SurfaceInteraction;

#[derive(Debug)]
pub struct GeometricPrimitive {
    pub shape: Arc<dyn Shape + Send + Sync>,
    pub material: Option<Box<dyn Material + Send + Sync>>,
    pub area_light: Option<Arc<dyn Light + Send + Sync>>,
    pub medium_interface: Option<()>,
}

impl Primitive for GeometricPrimitive {
    fn intersect(&'a self, ray: &mut Ray) -> Option<SurfaceInteraction<'a>> {
        if let Some((hit, mut isect)) = self.shape.intersect(ray, true) {
            ray.max = Some(hit);
            isect.primitive = Some(self);

            // Initialize mediumInterface

            Some(isect)
        } else {
            None
        }
    }

    fn intersect_p(&self, ray: &Ray) -> bool {
        self.shape.intersect_p(ray, true)
    }

    fn world_bound(&self) -> Bounds3<Float> {
        self.shape.world_bound()
    }

    fn get_area_light(&self) -> Option<Arc<dyn Light + Send + Sync>> {
        self.area_light.as_ref().cloned()
    }

    fn get_material(&self) -> Option<&(dyn Material + Send + Sync)> {
        self.material.as_ref()
            .map(|m| m.as_ref())
    }

    fn compute_scattering_functions(&'a self, isect: SurfaceInteraction<'a>, arena: &(), mode: TransportMode, allow_multiple_lobes: bool) -> SurfaceInteraction<'a> {
        if let Some(material) = &self.material {
            material.compute_scattering_functions(isect, arena, mode, allow_multiple_lobes)
        } else {
            panic!("GeometricPrimitive doesn't have material")
        }
    }
}
