use std::sync::Arc;
use prelude::*;
use super::Primitive;
use shape::Shape;
use interaction::SurfaceInteraction;

#[derive(Clone, Debug)]
pub struct GeometricPrimitive {
    pub shape: Arc<Shape>,
    pub material: Option<Arc<()>>,
    pub area_light: Option<Arc<()>>,
    pub medium_interface: Option<()>,
}

impl Primitive for GeometricPrimitive {
    fn intersect<'a>(&'a self, ray: &mut Ray) -> Option<SurfaceInteraction<'a>> {
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

    fn get_area_light(&self) -> Option<Arc<()>> {
        self.area_light.as_ref().map(|a| a.clone())
    }

    fn get_material(&self) -> Option<Arc<()>> {
        self.material.as_ref().map(|m| m.clone())
    }

    fn compute_scattering_functions(&self, isect: SurfaceInteraction, arena: (), mode: (), allow_multiple_lobes: bool) {
        if let Some(material) = &self.material {
            // todo material->ComputeScatteringFunctions()
        }
        unimplemented!()
    }
}
